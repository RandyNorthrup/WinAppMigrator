//! How a builder finds clients on its network. It sends a [`Query`] to each network's broadcast
//! address, and each client answers it with an [`Announcement`]. Nothing here is secret or trusted:
//! it only says where to connect, and pairing by the code is what says who is there.

use std::collections::BTreeMap;
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use crate::{DISCOVERY_PORT, MAGIC, PROTOCOL_VERSION};

/// More than any query or announcement, so that a long one is not taken for either.
const MAX_DATAGRAM: usize = 2048;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Query {
    pub magic: String,
    pub version: u32,
    /// Answered back, so that an answer is to this query.
    pub nonce: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Announcement {
    pub magic: String,
    pub version: u32,
    pub nonce: u64,
    pub computer_name: String,
    pub windows_name: String,
    pub architecture: String,
    /// The TCP port it takes a connection on.
    pub port: u16,
}

/// A client that answered, and where it can be reached.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Found {
    pub address: SocketAddr,
    pub announcement: Announcement,
}

/// Ask who is there, on each broadcast address given, and wait `wait` for answers. A client is listed
/// once, however many networks it answered on.
pub fn ask(
    broadcasts: &[std::net::Ipv4Addr],
    nonce: u64,
    wait: Duration,
) -> io::Result<Vec<Found>> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    let query = serde_json::to_vec(&Query {
        magic: MAGIC.to_owned(),
        version: PROTOCOL_VERSION,
        nonce,
    })
    .map_err(io::Error::other)?;
    for broadcast in broadcasts {
        // One network that will not take it says nothing of the others.
        let _ = socket.send_to(&query, (*broadcast, DISCOVERY_PORT));
    }
    let deadline = Instant::now() + wait;
    let mut found = BTreeMap::new();
    let mut buffer = [0_u8; MAX_DATAGRAM];
    while let Some(left) = deadline.checked_duration_since(Instant::now()) {
        socket.set_read_timeout(Some(left.max(Duration::from_millis(1))))?;
        let (length, from) = match socket.recv_from(&mut buffer) {
            Ok(received) => received,
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut
                ) =>
            {
                break;
            }
            // A network that refused the query can say so to the next read. That is not an answer.
            Err(_) => continue,
        };
        let Ok(announcement) = serde_json::from_slice::<Announcement>(&buffer[..length]) else {
            continue;
        };
        if announcement.magic != MAGIC
            || announcement.version != PROTOCOL_VERSION
            || announcement.nonce != nonce
        {
            continue;
        }
        let address = SocketAddr::new(from.ip(), announcement.port);
        found.insert(
            address,
            Found {
                address,
                announcement,
            },
        );
    }
    Ok(found.into_values().collect())
}

/// What a client answers a datagram with, if it is a query of this version.
pub fn answer(datagram: &[u8], about: &Announcement) -> Option<Vec<u8>> {
    let query = serde_json::from_slice::<Query>(datagram).ok()?;
    if query.magic != MAGIC || query.version != PROTOCOL_VERSION {
        return None;
    }
    serde_json::to_vec(&Announcement {
        nonce: query.nonce,
        ..about.clone()
    })
    .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn about() -> Announcement {
        Announcement {
            magic: MAGIC.to_owned(),
            version: PROTOCOL_VERSION,
            nonce: 0,
            computer_name: "OLD-PC".to_owned(),
            windows_name: "Windows 7 Professional Service Pack 1".to_owned(),
            architecture: "x86".to_owned(),
            port: crate::LINK_PORT,
        }
    }

    #[test]
    fn a_query_is_answered_with_its_own_nonce_and_anything_else_is_not() {
        let query = serde_json::to_vec(&Query {
            magic: MAGIC.to_owned(),
            version: PROTOCOL_VERSION,
            nonce: 42,
        })
        .unwrap_or_default();
        let answered = answer(&query, &about())
            .and_then(|bytes| serde_json::from_slice::<Announcement>(&bytes).ok());
        assert_eq!(answered.map(|announcement| announcement.nonce), Some(42));

        let other_version = serde_json::to_vec(&Query {
            magic: MAGIC.to_owned(),
            version: PROTOCOL_VERSION + 1,
            nonce: 42,
        })
        .unwrap_or_default();
        assert_eq!(answer(&other_version, &about()), None);
        assert_eq!(answer(b"hello", &about()), None);
    }

    #[test]
    fn a_client_on_this_pc_is_found_by_asking() {
        let Ok(listener) = UdpSocket::bind(("127.0.0.1", DISCOVERY_PORT)) else {
            // Something else has the port here. What is tested above does not need it.
            return;
        };
        let client = std::thread::spawn(move || {
            let mut buffer = [0_u8; MAX_DATAGRAM];
            if let Ok((length, from)) = listener.recv_from(&mut buffer)
                && let Some(reply) = answer(&buffer[..length], &about())
            {
                let _ = listener.send_to(&reply, from);
            }
        });
        let found =
            ask(&[std::net::Ipv4Addr::LOCALHOST], 7, Duration::from_secs(2)).unwrap_or_default();
        let _ = client.join();
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].announcement.computer_name, "OLD-PC");
        assert_eq!(found[0].address.port(), crate::LINK_PORT);
    }
}
