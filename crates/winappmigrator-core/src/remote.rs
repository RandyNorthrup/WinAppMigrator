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
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream, ToSocketAddrs};
use std::path::{Component, Path, PathBuf, Prefix};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use winappmigrator_link::channel::{SecureChannel, pair_as_builder};
use winappmigrator_link::discovery::{self, Found};
use winappmigrator_link::protocol::{About, Entry, FailureKind, Request, Response, decode, encode};
use winappmigrator_link::{LINK_PORT, LinkError};

use crate::path_utils::{POWERSHELL_UTF8_OUTPUT, powershell_command};

/// What starts the root of every PC read through a client.
pub const LINK_ROOT_PREFIX: &str = "WAMLINK-";
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
/// How long one answer may take. A client reads from a snapshot, so this is generous.
const ANSWER_TIMEOUT: Duration = Duration::from_secs(120);
const DISCOVERY_WAIT: Duration = Duration::from_secs(2);
/// Every local IPv4 network this PC is on, as `address/prefix`, from Windows.
const NETWORKS_SCRIPT: &str = "Get-NetIPAddress -AddressFamily IPv4 | Where-Object { $_.IPAddress -notlike '127.*' -and $_.IPAddress -notlike '169.254.*' } | ForEach-Object { $_.IPAddress + '/' + $_.PrefixLength }";

/// A client this PC is paired with.
pub struct RemotePc {
    id: String,
    address: SocketAddr,
    about: About,
    channel: Mutex<SecureChannel<TcpStream>>,
    /// What listings said of each path, so that describing one in a folder just listed asks nothing.
    described: Mutex<HashMap<String, Entry>>,
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
    let stream = TcpStream::connect_timeout(&address, CONNECT_TIMEOUT)?;
    stream.set_nodelay(true)?;
    stream.set_read_timeout(Some(ANSWER_TIMEOUT))?;
    stream.set_write_timeout(Some(ANSWER_TIMEOUT))?;
    let mut channel = pair_as_builder(stream, code).map_err(link_error)?;
    let about = match ask(&mut channel, &Request::About)? {
        (Response::About(about), _) => about,
        _ => return Err(io::Error::other("the client did not say what it is")),
    };
    let id = format!(
        "{}{}",
        address.ip().to_string().replace(['.', ':'], "-"),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |since| since.as_millis())
    );
    let remote = Arc::new(RemotePc {
        id: id.clone(),
        address,
        about,
        channel: Mutex::new(channel),
        described: Mutex::new(HashMap::new()),
    });
    if let Ok(mut links) = links().lock() {
        links.insert(id, Arc::clone(&remote));
    }
    Ok(remote)
}

/// Close the link to a PC, which ends the client's session.
pub fn disconnect(remote: &RemotePc) {
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

fn ask(
    channel: &mut SecureChannel<TcpStream>,
    request: &Request,
) -> io::Result<(Response, Vec<u8>)> {
    channel
        .send(&encode(request, b"").map_err(link_error)?)
        .map_err(link_error)?;
    let message = channel.receive().map_err(link_error)?;
    let (response, body) = decode::<Response>(&message).map_err(link_error)?;
    Ok((response, body.to_vec()))
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

    /// One request, and its answer, or what the client said was wrong as an error of that kind.
    fn request(&self, request: &Request) -> io::Result<(Response, Vec<u8>)> {
        let mut channel = self
            .channel
            .lock()
            .map_err(|_| io::Error::other("the link to the other PC failed"))?;
        match ask(&mut channel, request)? {
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

    pub(crate) fn describe(&self, path: &str) -> io::Result<Entry> {
        if let Some(entry) = self
            .described
            .lock()
            .ok()
            .and_then(|described| described.get(&path.to_ascii_lowercase()).cloned())
        {
            return Ok(entry);
        }
        match self.request(&Request::Describe {
            path: path.to_owned(),
        })? {
            (Response::Described { entry }, _) => Ok(entry),
            _ => Err(io::Error::other("the client answered something else")),
        }
    }

    pub(crate) fn list(&self, path: &str) -> io::Result<Vec<Entry>> {
        let mut all = Vec::new();
        loop {
            match self.request(&Request::List {
                path: path.to_owned(),
                from: all.len(),
            })? {
                (Response::Listed { entries, more }, _) => {
                    let got = entries.len();
                    all.extend(entries);
                    if !more || got == 0 {
                        break;
                    }
                }
                _ => return Err(io::Error::other("the client answered something else")),
            }
        }
        if let Ok(mut described) = self.described.lock() {
            for entry in &all {
                let full = if path.is_empty() {
                    entry.name.clone()
                } else {
                    format!(r"{path}\{}", entry.name)
                };
                described.insert(full.to_ascii_lowercase(), entry.clone());
            }
        }
        Ok(all)
    }

    pub(crate) fn read(&self, path: &str, offset: u64, length: u32) -> io::Result<Vec<u8>> {
        match self.request(&Request::Read {
            path: path.to_owned(),
            offset,
            length,
        })? {
            (Response::Read, body) => Ok(body),
            _ => Err(io::Error::other("the client answered something else")),
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
        return Some(Err(io::Error::new(
            io::ErrorKind::NotConnected,
            "the link to the other PC is closed",
        )));
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
