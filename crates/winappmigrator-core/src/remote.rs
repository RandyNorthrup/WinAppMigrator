//! Another PC, read through its client over the network.
//!
//! The client (`winappmigrator-client`) runs on the PC an app is moving off, takes a snapshot of its
//! Windows drive, and answers requests about it through an encrypted link it pairs by a code it
//! shows (see `winappmigrator_link`). This finds clients, pairs with one, and gives it a root of its
//! own, `\\?\WAMLINK-<id>\`, that is a [`crate::system_context::ScanSource::OfflineVolume`] like any
//! connected drive. What is read below that root goes through [`crate::source_fs`] to the client, so
//! the capture is the one a drive gets, and only what it reads crosses the network.
//!
//! No path of this PC can start with that root, and Windows refuses to open one that does, so
//! anything that reads it some other way fails rather than reading something of this PC's.

use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpStream, ToSocketAddrs};
use std::path::{Component, Path, PathBuf, Prefix};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use winappmigrator_link::channel::{SecureChannel, pair_as_builder};
use winappmigrator_link::discovery::{self, Found};
use winappmigrator_link::paths::{is_ordinary_name, names_of};
use winappmigrator_link::protocol::{
    About, Entry, FailureKind, MAX_LISTED, Request, Response, decode, encode,
};
use winappmigrator_link::{LINK_PORT, LinkError};

use crate::path_utils::{POWERSHELL_UTF8_OUTPUT, powershell_command};

/// What starts the root of every PC read through a client.
pub const LINK_ROOT_PREFIX: &str = "WAMLINK-";
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
/// How long one request may take, all of its answer included, however slowly that comes. A client
/// reads from a snapshot, so this is generous.
const ANSWER_TIMEOUT: Duration = Duration::from_secs(120);
/// How soon a request waiting on a client sees that the link was closed.
const CLOSED_CHECK_EVERY: Duration = Duration::from_millis(250);
const DISCOVERY_WAIT: Duration = Duration::from_secs(2);
/// The most a folder may hold, as a client lists it. More than any real folder has, and few enough
/// to keep, so that a client that always says there is more cannot fill this PC's memory.
const MAX_ENTRIES_IN_A_FOLDER: usize = 125 * MAX_LISTED;
/// Every local IPv4 network this PC is on, as `address/prefix`, from Windows.
const NETWORKS_SCRIPT: &str = "Get-NetIPAddress -AddressFamily IPv4 | Where-Object { $_.IPAddress -notlike '127.*' -and $_.IPAddress -notlike '169.254.*' } | ForEach-Object { $_.IPAddress + '/' + $_.PrefixLength }";

/// A client this PC is paired with.
pub struct RemotePc {
    id: String,
    address: SocketAddr,
    about: About,
    channel: Mutex<SecureChannel<Timed>>,
    /// The connection itself, to shut down while a request is waiting on it.
    socket: TcpStream,
    watch: Arc<Watch>,
    /// How long one request may take.
    answer_timeout: Duration,
    /// What the client first said of each path, by listing it or describing it. The snapshot does
    /// not change, so that is what the path is for as long as the link lasts, and a file is never
    /// read past the size it was first said to be.
    described: Mutex<HashMap<String, Entry>>,
}

/// What the connection under a link checks before each wait on it: whether the link is closed, and
/// when the request under way has to be answered by.
struct Watch {
    closed: AtomicBool,
    deadline: Mutex<Instant>,
}

impl Watch {
    /// Give the request that starts now `timeout` in all.
    fn start(&self, timeout: Duration) -> io::Result<()> {
        let now = Instant::now();
        let mut deadline = self.deadline.lock().map_err(|_| failed())?;
        *deadline = now.checked_add(timeout).unwrap_or(now);
        Ok(())
    }

    fn time_left(&self) -> io::Result<Duration> {
        if self.closed.load(Ordering::Relaxed) {
            return Err(closed());
        }
        let deadline = *self.deadline.lock().map_err(|_| failed())?;
        let left = deadline.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "the other PC took too long to answer",
            ));
        }
        Ok(left)
    }
}

/// The connection under a link. Each wait on it lasts only as long as the request under way has
/// left, so a client that sends an answer a byte at a time still runs out of time, and no wait
/// starts once the link is closed.
struct Timed {
    stream: TcpStream,
    watch: Arc<Watch>,
}

impl Read for Timed {
    /// Windows does not end a wait for bytes when the connection is shut down from another thread,
    /// so this waits in short turns, looking at the watch between them. It waits with a peek, which
    /// takes nothing, so a turn that ends as bytes arrive loses none of them.
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        loop {
            let left = self.watch.time_left()?;
            self.stream
                .set_read_timeout(Some(left.min(CLOSED_CHECK_EVERY)))?;
            match self.stream.peek(&mut [0_u8; 1]) {
                Ok(_) => break,
                Err(error)
                    if matches!(
                        error.kind(),
                        io::ErrorKind::TimedOut | io::ErrorKind::WouldBlock
                    ) => {}
                Err(error) => return Err(error),
            }
        }
        self.stream.read(buffer)
    }
}

impl Write for Timed {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let left = self.watch.time_left()?;
        self.stream.set_write_timeout(Some(left))?;
        self.stream.write(buffer)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}

fn closed() -> io::Error {
    io::Error::new(
        io::ErrorKind::NotConnected,
        "the link to the other PC is closed",
    )
}

fn failed() -> io::Error {
    io::Error::other("the link to the other PC failed")
}

fn answered_something_else() -> io::Error {
    io::Error::other("the client answered something else")
}

fn links() -> &'static Mutex<HashMap<String, Arc<RemotePc>>> {
    static LINKS: OnceLock<Mutex<HashMap<String, Arc<RemotePc>>>> = OnceLock::new();
    LINKS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Ask every network this PC is on which clients are there.
pub fn find_clients() -> Vec<Found> {
    let mut broadcasts: Vec<Ipv4Addr> = local_networks()
        .into_iter()
        .map(|(address, prefix)| broadcast_of(address, prefix))
        .collect();
    broadcasts.push(Ipv4Addr::BROADCAST);
    broadcasts.sort();
    broadcasts.dedup();
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since| since.as_nanos() as u64);
    discovery::ask(&broadcasts, nonce, DISCOVERY_WAIT).unwrap_or_default()
}

fn local_networks() -> Vec<(Ipv4Addr, u8)> {
    let Ok(output) =
        powershell_command(format!("{POWERSHELL_UTF8_OUTPUT}{NETWORKS_SCRIPT}")).output()
    else {
        return Vec::new();
    };
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let (address, prefix) = line.trim().split_once('/')?;
            Some((address.parse().ok()?, prefix.parse().ok()?))
        })
        .collect()
}

/// The broadcast address of the network `address/prefix` is on.
pub fn broadcast_of(address: Ipv4Addr, prefix: u8) -> Ipv4Addr {
    let host_bits = 32_u32.saturating_sub(u32::from(prefix.min(32)));
    let mask = u32::MAX.checked_shl(host_bits).unwrap_or(0);
    Ipv4Addr::from(u32::from(address) | !mask)
}

/// Where a client typed in by name or address is, on the port clients take connections on.
pub fn address_of(typed: &str) -> io::Result<SocketAddr> {
    let typed = typed.trim().trim_start_matches(r"\\");
    if let Ok(ip) = typed.parse::<IpAddr>() {
        return Ok(SocketAddr::new(ip, LINK_PORT));
    }
    (typed, LINK_PORT)
        .to_socket_addrs()?
        .find(SocketAddr::is_ipv4)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("{typed} was not found on the network"),
            )
        })
}

/// Pair with the client at `address` by the code it shows, and keep the link until
/// [`disconnect`]. What it reads is below [`RemotePc::root`].
pub fn connect(address: SocketAddr, code: &str) -> io::Result<Arc<RemotePc>> {
    connect_waiting(address, code, ANSWER_TIMEOUT)
}

/// [`connect`], giving each request `answer_timeout` in all.
fn connect_waiting(
    address: SocketAddr,
    code: &str,
    answer_timeout: Duration,
) -> io::Result<Arc<RemotePc>> {
    let stream = TcpStream::connect_timeout(&address, CONNECT_TIMEOUT)?;
    stream.set_nodelay(true)?;
    let socket = stream.try_clone()?;
    let watch = Arc::new(Watch {
        closed: AtomicBool::new(false),
        deadline: Mutex::new(Instant::now()),
    });
    // Pairing has as long as a request does.
    watch.start(answer_timeout)?;
    let timed = Timed {
        stream,
        watch: Arc::clone(&watch),
    };
    let mut channel = pair_as_builder(timed, code).map_err(link_error)?;
    watch.start(answer_timeout)?;
    let about = match ask(&mut channel, &Request::About)? {
        (Response::About(about), _) => about,
        _ => return Err(io::Error::other("the client did not say what it is")),
    };
    // With a count, as two links made in the same millisecond would otherwise share a root.
    static LINKS_MADE: AtomicU64 = AtomicU64::new(0);
    let id = format!(
        "{}{}-{}",
        address.ip().to_string().replace(['.', ':'], "-"),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |since| since.as_millis()),
        LINKS_MADE.fetch_add(1, Ordering::Relaxed)
    );
    let remote = Arc::new(RemotePc {
        id: id.clone(),
        address,
        about,
        channel: Mutex::new(channel),
        socket,
        watch,
        answer_timeout,
        described: Mutex::new(HashMap::new()),
    });
    if let Ok(mut links) = links().lock() {
        links.insert(id, Arc::clone(&remote));
    }
    Ok(remote)
}

/// Close the link to a PC, which ends the client's session, and any request still waiting on it.
pub fn disconnect(remote: &RemotePc) {
    remote.close();
    if let Ok(mut links) = links().lock() {
        links.remove(&remote.id);
    }
}

fn link_error(error: LinkError) -> io::Error {
    match error {
        LinkError::Io(error) => error,
        LinkError::WrongCode => io::Error::new(io::ErrorKind::PermissionDenied, error.to_string()),
        other => io::Error::other(other.to_string()),
    }
}

fn ask(channel: &mut SecureChannel<Timed>, request: &Request) -> io::Result<(Response, Vec<u8>)> {
    channel
        .send(&encode(request, b"").map_err(link_error)?)
        .map_err(link_error)?;
    let message = channel.receive().map_err(link_error)?;
    let (response, body) = decode::<Response>(&message).map_err(link_error)?;
    Ok((response, body.to_vec()))
}

/// The path a request asks about, if it asks about one.
fn path_of(request: &Request) -> Option<&str> {
    match request {
        Request::About => None,
        Request::Describe { path } | Request::List { path, .. } | Request::Read { path, .. } => {
            Some(path)
        }
    }
}

impl RemotePc {
    /// Where everything on that PC is, as a scan sees it: `\\?\WAMLINK-<id>\`.
    pub fn root(&self) -> PathBuf {
        PathBuf::from(format!(r"\\?\{LINK_ROOT_PREFIX}{}\", self.id))
    }

    pub fn about(&self) -> &About {
        &self.about
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    /// End the link. A request waiting on the client stops waiting, and none is sent after.
    fn close(&self) {
        self.watch.closed.store(true, Ordering::Relaxed);
        let _ = self.socket.shutdown(Shutdown::Both);
    }

    /// One request, and its answer, or what the client said was wrong as an error of that kind.
    fn request(&self, request: &Request) -> io::Result<(Response, Vec<u8>)> {
        if path_of(request).is_some_and(|path| names_of(path).is_none()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "only a path below the other PC's root, in ordinary names, is asked about",
            ));
        }
        if self.watch.closed.load(Ordering::Relaxed) {
            return Err(closed());
        }
        let mut channel = self.channel.lock().map_err(|_| failed())?;
        self.watch.start(self.answer_timeout)?;
        let answered = match ask(&mut channel, request) {
            Ok(answered) => answered,
            Err(error) => {
                // Part of a message may have been sent or read, so the next could not be told
                // from it. Nothing more is asked.
                self.close();
                return Err(error);
            }
        };
        match answered {
            (Response::Failed { kind, message }, _) => Err(io::Error::new(
                match kind {
                    FailureKind::NotFound => io::ErrorKind::NotFound,
                    FailureKind::Denied => io::ErrorKind::PermissionDenied,
                    FailureKind::Refused => io::ErrorKind::InvalidInput,
                    FailureKind::Other => io::ErrorKind::Other,
                },
                message,
            )),
            answered => Ok(answered),
        }
    }

    /// What the client said of `path` before, or says now, the first time it is asked.
    pub(crate) fn describe(&self, path: &str) -> io::Result<Entry> {
        let key = path.to_ascii_lowercase();
        if let Some(entry) = self
            .described
            .lock()
            .ok()
            .and_then(|described| described.get(&key).cloned())
        {
            return Ok(entry);
        }
        let entry = match self.request(&Request::Describe {
            path: path.to_owned(),
        })? {
            (Response::Described { entry }, _) => entry,
            _ => return Err(answered_something_else()),
        };
        Ok(match self.described.lock() {
            Ok(mut described) => described.entry(key).or_insert(entry).clone(),
            Err(_) => entry,
        })
    }

    /// A folder's entries, each as the client first said it was. A name that is not an ordinary
    /// one, such as `..`, `C:\Windows` or `\\server\share`, is left out, as joined to the folder's
    /// path it would lead somewhere else.
    pub(crate) fn list(&self, path: &str) -> io::Result<Vec<Entry>> {
        let mut all = Vec::new();
        let mut from = 0_usize;
        loop {
            let (entries, more) = match self.request(&Request::List {
                path: path.to_owned(),
                from,
            })? {
                (Response::Listed { entries, more }, _) => (entries, more),
                _ => return Err(answered_something_else()),
            };
            if more && entries.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "the other PC said a folder had more in it, and gave none of it",
                ));
            }
            from = from.saturating_add(entries.len());
            if from > MAX_ENTRIES_IN_A_FOLDER {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "the other PC listed more than {MAX_ENTRIES_IN_A_FOLDER} things in one folder"
                    ),
                ));
            }
            all.extend(
                entries
                    .into_iter()
                    .filter(|entry| is_ordinary_name(&entry.name)),
            );
            if !more {
                break;
            }
        }
        let Ok(mut described) = self.described.lock() else {
            return Ok(all);
        };
        Ok(all
            .into_iter()
            .map(|entry| {
                let full = if path.is_empty() {
                    entry.name.clone()
                } else {
                    format!(r"{path}\{}", entry.name)
                };
                let first = described
                    .entry(full.to_ascii_lowercase())
                    .or_insert_with(|| entry.clone());
                Entry {
                    name: entry.name,
                    ..first.clone()
                }
            })
            .collect())
    }

    /// Up to `length` bytes of a file from `offset`. An answer longer than that is an error.
    pub(crate) fn read(&self, path: &str, offset: u64, length: u32) -> io::Result<Vec<u8>> {
        match self.request(&Request::Read {
            path: path.to_owned(),
            offset,
            length,
        })? {
            (Response::Read, body) if body.len() <= length as usize => Ok(body),
            (Response::Read, _) => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "the other PC sent more of a file than was asked for",
            )),
            _ => Err(answered_something_else()),
        }
    }
}

/// The PC a path is on and where it is there, as the client is asked about it, if it is below a
/// paired PC's root. `None` for any path of this PC. A path below a root this PC is no longer
/// paired with is an error.
pub(crate) fn remote_of(path: &Path) -> Option<io::Result<(Arc<RemotePc>, String)>> {
    let mut components = path.components();
    let Some(Component::Prefix(prefix)) = components.next() else {
        return None;
    };
    let Prefix::Verbatim(name) = prefix.kind() else {
        return None;
    };
    let id = name.to_str()?.strip_prefix(LINK_ROOT_PREFIX)?.to_owned();
    let remote = links()
        .lock()
        .ok()
        .and_then(|links| links.get(&id).cloned());
    let Some(remote) = remote else {
        return Some(Err(closed()));
    };
    let mut names = Vec::new();
    for component in components {
        match component {
            Component::RootDir => {}
            Component::Normal(name) => match name.to_str() {
                Some(name) => names.push(name),
                None => {
                    return Some(Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "a name that is not text cannot be asked about",
                    )));
                }
            },
            _ => {
                return Some(Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "only a path below the other PC's root can be asked about",
                )));
            }
        }
    }
    Some(Ok((remote, names.join("\\"))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    use std::thread;

    use winappmigrator_link::channel::pair_as_client;
    use winappmigrator_link::protocol::EntryKind;

    use crate::source_fs;

    const CODE: &str = "246810";

    fn entry(name: &str, kind: EntryKind, len: u64) -> Entry {
        Entry {
            name: name.to_owned(),
            kind,
            len,
            attributes: 0,
            modified: 0,
            created: 0,
            accessed: 0,
        }
    }

    /// A client over loopback that says what it is, then answers each request as `answer` does,
    /// until that has nothing to say and it closes the link.
    fn client(
        mut answer: impl FnMut(Request) -> Option<(Response, Vec<u8>)> + Send + 'static,
    ) -> SocketAddr {
        let Some((listener, address)) = TcpListener::bind("127.0.0.1:0")
            .ok()
            .and_then(|listener| Some((listener.local_addr().ok()?, listener)))
            .map(|(address, listener)| (listener, address))
        else {
            panic!("nothing could listen on loopback");
        };
        thread::spawn(move || {
            let Ok((stream, _)) = listener.accept() else {
                return;
            };
            let Ok(mut channel) = pair_as_client(stream, CODE) else {
                return;
            };
            while let Ok(message) = channel.receive() {
                let Ok((request, _)) = decode::<Request>(&message) else {
                    return;
                };
                let answered = match request {
                    Request::About => Some((Response::About(about()), Vec::new())),
                    other => answer(other),
                };
                let Some(message) =
                    answered.and_then(|(response, body)| encode(&response, &body).ok())
                else {
                    return;
                };
                if channel.send(&message).is_err() {
                    return;
                }
            }
        });
        address
    }

    fn about() -> About {
        About {
            client_version: "test".to_owned(),
            computer_name: "OLD-PC".to_owned(),
            windows_name: "Windows 7".to_owned(),
            architecture: "AMD64".to_owned(),
            snapshot_taken: String::new(),
        }
    }

    fn paired(
        answer: impl FnMut(Request) -> Option<(Response, Vec<u8>)> + Send + 'static,
    ) -> Arc<RemotePc> {
        match connect(client(answer), CODE) {
            Ok(remote) => remote,
            Err(error) => panic!("pairing over loopback failed: {error}"),
        }
    }

    #[test]
    fn a_client_that_always_has_more_to_list_is_stopped() {
        let remote = paired(|request| match request {
            Request::List { .. } => Some((
                Response::Listed {
                    entries: Vec::new(),
                    more: true,
                },
                Vec::new(),
            )),
            _ => None,
        });
        assert!(remote.list("Windows").is_err());
        disconnect(&remote);

        // Pages that go on past what a folder may hold. These end, so that without a limit this
        // fails rather than waits.
        let mut pages = 0;
        let remote = paired(move |request| match request {
            Request::List { from, .. } => {
                pages += 1;
                let entries = (from..from + MAX_LISTED)
                    .map(|n| entry(&format!("f{n}"), EntryKind::File, 1))
                    .collect();
                let more = pages <= MAX_ENTRIES_IN_A_FOLDER / MAX_LISTED + 1;
                Some((Response::Listed { entries, more }, Vec::new()))
            }
            _ => None,
        });
        assert!(remote.list("Windows").is_err());
        disconnect(&remote);
    }

    #[test]
    fn a_file_is_never_read_past_the_size_it_was_first_said_to_be() {
        let mut described = 0;
        let mut reads = 0;
        let remote = paired(move |request| match request {
            Request::Describe { path } => {
                described += 1;
                // Small when the size is checked, then as large as can be when it is read.
                let len = if described == 1 { 10 } else { u64::MAX };
                Some((
                    Response::Described {
                        entry: entry(&path, EntryKind::File, len),
                    },
                    Vec::new(),
                ))
            }
            Request::List { .. } => Some((
                Response::Listed {
                    entries: vec![entry("listed.bin", EntryKind::File, 10)],
                    more: false,
                },
                Vec::new(),
            )),
            // As much as is asked for, a few times, so that without the limit this fails
            // rather than fills memory.
            Request::Read { length, .. } => {
                reads += 1;
                (reads < 8).then(|| (Response::Read, vec![7; length as usize]))
            }
            Request::About => None,
        });
        let described = remote.root().join("described.bin");
        assert_eq!(
            source_fs::metadata(&described).map(|m| m.len()).ok(),
            Some(10)
        );
        assert_eq!(source_fs::read(&described).map(|b| b.len()).ok(), Some(10));
        let listed = remote.root().join("Folder").join("listed.bin");
        assert_eq!(
            source_fs::read_dir(remote.root().join("Folder"))
                .map(Iterator::count)
                .ok(),
            Some(1)
        );
        assert_eq!(source_fs::read(&listed).map(|b| b.len()).ok(), Some(10));
        disconnect(&remote);
    }

    #[test]
    fn more_bytes_than_were_asked_for_are_an_error() {
        let remote = paired(|request| match request {
            Request::Describe { path } => Some((
                Response::Described {
                    entry: entry(&path, EntryKind::File, 1000),
                },
                Vec::new(),
            )),
            Request::Read { length, .. } => Some((Response::Read, vec![1; length as usize + 100])),
            _ => None,
        });
        let path = remote.root().join("a.bin");
        let Ok(mut file) = source_fs::open(&path) else {
            panic!("the file did not open");
        };
        let mut buffer = [0_u8; 16];
        assert!(file.seek_read(&mut buffer, 0).is_err());
        assert!(file.read(&mut buffer).is_err());
        disconnect(&remote);
    }

    #[test]
    fn a_name_that_would_lead_out_of_its_folder_is_dropped() {
        let remote = paired(|request| match request {
            Request::List { .. } => Some((
                Response::Listed {
                    entries: [
                        "..",
                        ".",
                        r"C:\Windows",
                        r"\\attacker\share\x",
                        r"a\..\..\b",
                        "a:stream",
                        "ab:stream",
                        "nul",
                        "ok.txt",
                    ]
                    .iter()
                    .map(|name| entry(name, EntryKind::File, 1))
                    .collect(),
                    more: false,
                },
                Vec::new(),
            )),
            Request::Describe { path } => Some((
                Response::Described {
                    entry: entry(&path, EntryKind::File, 1),
                },
                Vec::new(),
            )),
            _ => None,
        });
        let names: Vec<String> = source_fs::read_dir(remote.root().join("Windows"))
            .map(|entries| {
                entries
                    .flatten()
                    .map(|entry| entry.file_name().to_string_lossy().into_owned())
                    .collect()
            })
            .unwrap_or_default();
        assert_eq!(names, ["ok.txt"]);
        // Nor is such a name asked about.
        let asked = source_fs::metadata(remote.root().join("Windows").join("ab:stream"));
        assert!(asked.is_err_and(|error| error.kind() == io::ErrorKind::InvalidInput));
        disconnect(&remote);
    }

    /// A client that pairs and says what it is, then does `then` with the next request waiting.
    fn client_that_stalls(then: impl FnOnce(&mut TcpStream) + Send + 'static) -> SocketAddr {
        let Some((listener, address)) = TcpListener::bind("127.0.0.1:0")
            .ok()
            .and_then(|listener| Some((listener.local_addr().ok()?, listener)))
            .map(|(address, listener)| (listener, address))
        else {
            panic!("nothing could listen on loopback");
        };
        thread::spawn(move || {
            let Ok((stream, _)) = listener.accept() else {
                return;
            };
            let Ok(mut raw) = stream.try_clone() else {
                return;
            };
            let Ok(mut channel) = pair_as_client(stream, CODE) else {
                return;
            };
            let Some(message) = channel
                .receive()
                .ok()
                .and_then(|_| encode(&Response::About(about()), b"").ok())
            else {
                return;
            };
            if channel.send(&message).is_ok() && channel.receive().is_ok() {
                then(&mut raw);
            }
        });
        address
    }

    #[test]
    fn an_answer_a_byte_at_a_time_still_runs_out_of_time() {
        let address = client_that_stalls(|raw| {
            // The length of a whole message, then a byte of it every fifth of a second, for longer
            // than the request has.
            let _ = raw.write_all(&[0, 0x10, 0, 0]);
            for _ in 0..75 {
                if raw.write_all(&[0]).is_err() {
                    return;
                }
                thread::sleep(Duration::from_millis(200));
            }
        });
        let remote = match connect_waiting(address, CODE, Duration::from_secs(2)) {
            Ok(remote) => remote,
            Err(error) => panic!("pairing over loopback failed: {error}"),
        };
        let started = Instant::now();
        let asked = remote.describe("Windows");
        assert!(asked.is_err_and(|error| error.kind() == io::ErrorKind::TimedOut));
        assert!(started.elapsed() < Duration::from_secs(8));
        disconnect(&remote);
    }

    #[test]
    fn disconnecting_ends_a_request_waiting_on_the_client() {
        let address = client_that_stalls(|_| thread::sleep(Duration::from_secs(20)));
        let remote = match connect(address, CODE) {
            Ok(remote) => remote,
            Err(error) => panic!("pairing over loopback failed: {error}"),
        };
        let asking = Arc::clone(&remote);
        let started = Instant::now();
        let request = thread::spawn(move || asking.describe("Windows").is_err());
        thread::sleep(Duration::from_millis(300));
        disconnect(&remote);
        assert!(request.join().unwrap_or(false));
        assert!(started.elapsed() < Duration::from_secs(10));
        // And nothing is asked after.
        assert!(
            remote
                .describe("Users")
                .is_err_and(|error| error.kind() == io::ErrorKind::NotConnected)
        );
    }

    #[test]
    fn a_networks_broadcast_address_is_its_host_bits_set() {
        let address = Ipv4Addr::new(192, 168, 77, 1);
        assert_eq!(broadcast_of(address, 24), Ipv4Addr::new(192, 168, 77, 255));
        assert_eq!(
            broadcast_of(Ipv4Addr::new(10, 10, 11, 205), 23),
            Ipv4Addr::new(10, 10, 11, 255)
        );
        assert_eq!(
            broadcast_of(Ipv4Addr::new(10, 1, 2, 3), 8),
            Ipv4Addr::new(10, 255, 255, 255)
        );
        assert_eq!(broadcast_of(address, 32), address);
        assert_eq!(broadcast_of(address, 0), Ipv4Addr::BROADCAST);
    }

    #[test]
    fn only_a_root_of_a_paired_pc_is_another_pcs() {
        assert!(remote_of(Path::new(r"C:\Windows")).is_none());
        assert!(remote_of(Path::new(r"\\?\C:\Windows")).is_none());
        assert!(remote_of(Path::new(r"\\server\share\x")).is_none());
        // A root of the right form that this PC is not paired with goes nowhere.
        let unpaired = remote_of(Path::new(r"\\?\WAMLINK-nothing\Windows"));
        assert!(
            matches!(unpaired, Some(Err(error)) if error.kind() == io::ErrorKind::NotConnected)
        );
    }

    #[test]
    fn a_typed_address_is_on_the_clients_port() {
        assert_eq!(
            address_of("192.168.77.10").ok(),
            Some(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(192, 168, 77, 10)),
                LINK_PORT
            ))
        );
    }
}
