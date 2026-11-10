//! Pairing and the encrypted channel.
//!
//! The builder connects and both sides run SPAKE2 with the code the client shows. Neither sends the
//! code, and what crosses the network says nothing that would let anyone test guesses at it away from
//! the client: each guess takes a connection, and the client gives a code only a few. The key SPAKE2
//! agrees is stretched with HKDF-SHA256 over both sides' messages into one key for each direction.
//! Each side then proves it has the key before anything else is said.
//!
//! After that, each message is sealed with ChaCha20-Poly1305 under its direction's key, with a
//! counter as its nonce, so a message cannot be changed, replayed, reordered, or sent back the other
//! way without the other side refusing it.

use std::io::{self, Read, Write};

use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use hkdf::Hkdf;
use sha2_0_10::{Digest, Sha256};
use spake2::{Ed25519Group, Identity, Password, Spake2};

use crate::{LinkError, Result};

const HELLO: &[u8] = b"WAMLINK1";
const BUILDER_IDENTITY: &[u8] = b"winappmigrator-builder";
const CLIENT_IDENTITY: &[u8] = b"winappmigrator-client";
const BUILDER_CONFIRMS: &[u8] = b"builder has the key";
const CLIENT_CONFIRMS: &[u8] = b"client has the key";
const TO_CLIENT_INFO: &[u8] = b"winappmigrator-link builder to client";
const TO_BUILDER_INFO: &[u8] = b"winappmigrator-link client to builder";
/// A SPAKE2 message on Ed25519 is 33 bytes. Nothing in the clear is longer than this.
const MAX_PLAIN: usize = 64;
/// The most a sealed message holds: a read of 1 MiB with what describes it.
pub const MAX_MESSAGE: usize = 1024 * 1024 + 64 * 1024;
const TAG_BYTES: usize = 16;

/// Which end of the link this is.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    Builder,
    Client,
}

/// A link both sides hold the keys of.
pub struct SecureChannel<S> {
    stream: S,
    sealing: ChaCha20Poly1305,
    opening: ChaCha20Poly1305,
    sent: u64,
    received: u64,
}

/// Pair as the builder, with the code the person read off the old PC.
pub fn pair_as_builder<S: Read + Write>(mut stream: S, code: &str) -> Result<SecureChannel<S>> {
    let (state, outbound) = Spake2::<Ed25519Group>::start_a(
        &Password::new(code_bytes(code).as_bytes()),
        &Identity::new(BUILDER_IDENTITY),
        &Identity::new(CLIENT_IDENTITY),
    );
    write_plain(&mut stream, HELLO)?;
    write_plain(&mut stream, &outbound)?;
    let inbound = read_plain(&mut stream).map_err(closed_means_wrong_code)?;
    let key = state
        .finish(&inbound)
        .map_err(|_| LinkError::Protocol("its pairing message was not one".to_owned()))?;
    let mut channel = SecureChannel::new(stream, &key, Side::Builder, &outbound, &inbound);
    channel.send(BUILDER_CONFIRMS)?;
    match channel.receive() {
        Ok(confirmed) if confirmed == CLIENT_CONFIRMS => Ok(channel),
        Ok(_) => Err(LinkError::Protocol("it did not confirm the key".to_owned())),
        Err(LinkError::Tampered | LinkError::Closed) => Err(LinkError::WrongCode),
        Err(LinkError::Io(error)) if error.kind() == io::ErrorKind::UnexpectedEof => {
            Err(LinkError::WrongCode)
        }
        Err(other) => Err(other),
    }
}

/// Pair as the client, with the code it is showing. [`LinkError::WrongCode`] means the builder
/// did not have it.
pub fn pair_as_client<S: Read + Write>(mut stream: S, code: &str) -> Result<SecureChannel<S>> {
    if read_plain(&mut stream)? != HELLO {
        return Err(LinkError::Protocol(
            "it is not a WinAppMigrator builder of this version".to_owned(),
        ));
    }
    let inbound = read_plain(&mut stream)?;
    let (state, outbound) = Spake2::<Ed25519Group>::start_b(
        &Password::new(code_bytes(code).as_bytes()),
        &Identity::new(BUILDER_IDENTITY),
        &Identity::new(CLIENT_IDENTITY),
    );
    write_plain(&mut stream, &outbound)?;
    let key = state
        .finish(&inbound)
        .map_err(|_| LinkError::Protocol("its pairing message was not one".to_owned()))?;
    let mut channel = SecureChannel::new(stream, &key, Side::Client, &inbound, &outbound);
    match channel.receive() {
        Ok(confirmed) if confirmed == BUILDER_CONFIRMS => {}
        Ok(_) => return Err(LinkError::Protocol("it did not confirm the key".to_owned())),
        Err(LinkError::Tampered) => return Err(LinkError::WrongCode),
        Err(other) => return Err(other),
    }
    channel.send(CLIENT_CONFIRMS)?;
    Ok(channel)
}

/// Only the digits of a code count, so `123 456` and `123-456` are the same code.
fn code_bytes(code: &str) -> String {
    code.chars().filter(char::is_ascii_digit).collect()
}

fn closed_means_wrong_code(error: LinkError) -> LinkError {
    match error {
        LinkError::Io(error) if error.kind() == io::ErrorKind::UnexpectedEof => {
            LinkError::WrongCode
        }
        other => other,
    }
}

impl<S: Read + Write> SecureChannel<S> {
    /// `from_builder` and `from_client` are the two SPAKE2 messages, which tie the keys to this
    /// pairing alone.
    fn new(stream: S, key: &[u8], side: Side, from_builder: &[u8], from_client: &[u8]) -> Self {
        let mut transcript = Sha256::new();
        transcript.update(from_builder);
        transcript.update(from_client);
        let salt = transcript.finalize();
        let expand = Hkdf::<Sha256>::new(Some(&salt), key);
        let derive = |info: &[u8]| {
            let mut okm = [0_u8; 32];
            // 32 bytes is far below what HKDF-SHA256 can give, so this cannot fail.
            let _ = expand.expand(info, &mut okm);
            ChaCha20Poly1305::new(Key::from_slice(&okm))
        };
        let (sealing, opening) = match side {
            Side::Builder => (derive(TO_CLIENT_INFO), derive(TO_BUILDER_INFO)),
            Side::Client => (derive(TO_BUILDER_INFO), derive(TO_CLIENT_INFO)),
        };
        Self {
            stream,
            sealing,
            opening,
            sent: 0,
            received: 0,
        }
    }

    /// Seal and send one message.
    pub fn send(&mut self, message: &[u8]) -> Result<()> {
        if message.len() > MAX_MESSAGE {
            return Err(LinkError::Protocol("a message was too long".to_owned()));
        }
        let nonce = nonce_of(self.sent)?;
        let sealed = self
            .sealing
            .encrypt(Nonce::from_slice(&nonce), message)
            .map_err(|_| LinkError::Protocol("a message could not be sealed".to_owned()))?;
        self.sent += 1;
        let length = u32::try_from(sealed.len())
            .map_err(|_| LinkError::Protocol("a message was too long".to_owned()))?;
        // In one write: a short write and then the rest waits on TCP's delayed acknowledgement, which
        // made each request take a fifth of a second.
        let mut frame = Vec::with_capacity(4 + sealed.len());
        frame.extend_from_slice(&length.to_be_bytes());
        frame.extend_from_slice(&sealed);
        self.stream.write_all(&frame)?;
        self.stream.flush()?;
        Ok(())
    }

    /// Receive and open one message.
    pub fn receive(&mut self) -> Result<Vec<u8>> {
        let mut length = [0_u8; 4];
        if let Err(error) = self.stream.read_exact(&mut length) {
            return Err(if error.kind() == io::ErrorKind::UnexpectedEof {
                LinkError::Closed
            } else {
                LinkError::Io(error)
            });
        }
        let length = u32::from_be_bytes(length) as usize;
        if !(TAG_BYTES..=MAX_MESSAGE + TAG_BYTES).contains(&length) {
            return Err(LinkError::Tampered);
        }
        let mut sealed = vec![0_u8; length];
        self.stream.read_exact(&mut sealed)?;
        let nonce = nonce_of(self.received)?;
        let opened = self
            .opening
            .decrypt(Nonce::from_slice(&nonce), sealed.as_slice())
            .map_err(|_| LinkError::Tampered)?;
        self.received += 1;
        Ok(opened)
    }

    /// The connection under the link, for setting how long it waits.
    pub fn stream(&self) -> &S {
        &self.stream
    }
}

/// The nonce of the `count`th message one way: 4 zero bytes and the count. A key is only ever used
/// for one direction of one pairing, so no nonce is used twice.
fn nonce_of(count: u64) -> Result<[u8; 12]> {
    if count == u64::MAX {
        return Err(LinkError::Protocol(
            "the link has sent all it can".to_owned(),
        ));
    }
    let mut nonce = [0_u8; 12];
    nonce[4..].copy_from_slice(&count.to_le_bytes());
    Ok(nonce)
}

fn write_plain(stream: &mut impl Write, bytes: &[u8]) -> Result<()> {
    let length = u8::try_from(bytes.len())
        .map_err(|_| LinkError::Protocol("a pairing message was too long".to_owned()))?;
    let mut message = Vec::with_capacity(1 + bytes.len());
    message.push(length);
    message.extend_from_slice(bytes);
    stream.write_all(&message)?;
    stream.flush()?;
    Ok(())
}

fn read_plain(stream: &mut impl Read) -> Result<Vec<u8>> {
    let mut length = [0_u8; 1];
    stream.read_exact(&mut length)?;
    let length = usize::from(length[0]);
    if length > MAX_PLAIN {
        return Err(LinkError::Protocol(
            "a pairing message was too long".to_owned(),
        ));
    }
    let mut bytes = vec![0_u8; length];
    stream.read_exact(&mut bytes)?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{TcpListener, TcpStream};
    use std::thread;

    fn pair(builder_code: &str, client_code: &str) -> (Result<()>, Result<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").map_err(LinkError::Io);
        let Ok(listener) = listener else {
            return (Err(LinkError::Closed), Err(LinkError::Closed));
        };
        let Ok(address) = listener.local_addr() else {
            return (Err(LinkError::Closed), Err(LinkError::Closed));
        };
        let client_code = client_code.to_owned();
        let client = thread::spawn(move || -> Result<()> {
            let (stream, _) = listener.accept()?;
            let mut channel = pair_as_client(stream, &client_code)?;
            let asked = channel.receive()?;
            channel.send(&[asked.as_slice(), b" back"].concat())?;
            Ok(())
        });
        let builder = (|| -> Result<()> {
            let stream = TcpStream::connect(address)?;
            let mut channel = pair_as_builder(stream, builder_code)?;
            channel.send(b"hello")?;
            if channel.receive()? != b"hello back" {
                return Err(LinkError::Protocol("wrong answer".to_owned()));
            }
            Ok(())
        })();
        let client = client
            .join()
            .unwrap_or_else(|_| Err(LinkError::Protocol("the client panicked".to_owned())));
        (builder, client)
    }

    #[test]
    fn the_same_code_pairs_and_messages_go_both_ways() {
        let (builder, client) = pair("482913", "482 913");
        assert!(builder.is_ok(), "{builder:?}");
        assert!(client.is_ok(), "{client:?}");
    }

    #[test]
    fn another_code_pairs_with_nothing() {
        let (builder, client) = pair("482914", "482913");
        assert!(matches!(builder, Err(LinkError::WrongCode)), "{builder:?}");
        assert!(matches!(client, Err(LinkError::WrongCode)), "{client:?}");
    }

    /// One end of a link over memory, as each side writes it.
    struct Pipe {
        input: io::Cursor<Vec<u8>>,
        output: Vec<u8>,
    }

    impl Read for Pipe {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            self.input.read(buffer)
        }
    }

    impl Write for Pipe {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            self.output.write(buffer)
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn channels() -> (SecureChannel<Pipe>, SecureChannel<Pipe>) {
        let pipe = || Pipe {
            input: io::Cursor::new(Vec::new()),
            output: Vec::new(),
        };
        let key = [7_u8; 32];
        (
            SecureChannel::new(pipe(), &key, Side::Builder, b"a", b"b"),
            SecureChannel::new(pipe(), &key, Side::Client, b"a", b"b"),
        )
    }

    fn deliver(from: &mut SecureChannel<Pipe>, to: &mut SecureChannel<Pipe>) {
        let written = std::mem::take(&mut from.stream.output);
        to.stream.input = io::Cursor::new(written);
    }

    #[test]
    fn a_changed_replayed_or_reflected_message_is_refused() {
        let (mut builder, mut client) = channels();
        assert!(builder.send(b"read this").is_ok());
        let sealed = builder.stream.output.clone();

        // Changed on the way.
        let mut changed = sealed.clone();
        if let Some(last) = changed.last_mut() {
            *last ^= 1;
        }
        client.stream.input = io::Cursor::new(changed);
        assert!(matches!(client.receive(), Err(LinkError::Tampered)));

        // As sent, it opens, once.
        let (mut builder, mut client) = channels();
        assert!(builder.send(b"read this").is_ok());
        deliver(&mut builder, &mut client);
        assert_eq!(client.receive().ok().as_deref(), Some(&b"read this"[..]));
        client.stream.input = io::Cursor::new(sealed.clone());
        assert!(
            matches!(client.receive(), Err(LinkError::Tampered)),
            "replayed"
        );

        // Sent back the way it came.
        let (mut builder, _) = channels();
        builder.stream.input = io::Cursor::new(sealed);
        assert!(
            matches!(builder.receive(), Err(LinkError::Tampered)),
            "reflected"
        );
    }

    #[test]
    fn a_length_past_what_a_message_can_be_is_refused_before_it_is_read() {
        let (_, mut client) = channels();
        client.stream.input = io::Cursor::new(u32::MAX.to_be_bytes().to_vec());
        assert!(matches!(client.receive(), Err(LinkError::Tampered)));
        client.stream.input = io::Cursor::new(Vec::new());
        assert!(matches!(client.receive(), Err(LinkError::Closed)));
    }

    #[test]
    fn only_the_digits_of_a_code_count() {
        assert_eq!(code_bytes("482 913"), "482913");
        assert_eq!(code_bytes("482-913\r\n"), "482913");
    }
}
