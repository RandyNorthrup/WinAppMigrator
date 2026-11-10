//! WinAppMigrator Client. Run it on the PC an app is moving off, and choose Network PC… in
//! WinAppMigrator on the new one. See the library for what it does.

use std::io::{self, BufRead};
use std::net::{Ipv4Addr, TcpListener, UdpSocket};
use std::os::windows::process::CommandExt;
use std::process::ExitCode;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use winappmigrator_client::serve::serve;
use winappmigrator_client::system::{
    RunState, Snapshot, about_this_pc, new_code, open_firewall, start_caretaker,
};
use winappmigrator_link::channel::pair_as_client;
use winappmigrator_link::discovery::{Announcement, answer};
use winappmigrator_link::{DISCOVERY_PORT, LINK_PORT, LinkError, MAGIC, PROTOCOL_VERSION};

/// Wrong codes a code is given before it is replaced, so that guessing it gets nowhere.
const ATTEMPTS_PER_CODE: u32 = 5;
/// How long a paired builder may go without asking anything before the link is closed.
const IDLE_LIMIT: Duration = Duration::from_secs(30 * 60);

fn main() -> ExitCode {
    if std::env::args().nth(1).as_deref() == Some(winappmigrator_client::system::CARETAKER) {
        winappmigrator_client::system::take_care();
        return ExitCode::SUCCESS;
    }
    // Run from a folder only an administrator can change, before anything here can have Windows
    // load a DLL by name: see `safe_folder`.
    if !winappmigrator_client::system::runs_from_safe_folder() {
        return match winappmigrator_client::system::run_from_safe_folder() {
            Ok(status) if status.success() => ExitCode::SUCCESS,
            Ok(_) => ExitCode::FAILURE,
            Err(error) => {
                println!("WinAppMigrator Client could not start from a folder only administrators");
                println!("can change: {error}");
                println!("Press Enter to close this window.");
                let _ = io::stdin().lock().read_line(&mut String::new());
                ExitCode::FAILURE
            }
        };
    }
    // The window says what it is, rather than whatever started it. `title` changes the console it
    // shares with this, and nothing else. Raw, since `title` keeps any quotes it is given.
    let _ = winappmigrator_client::system::windows_tool("cmd.exe")
        .args(["/d", "/c"])
        .raw_arg("title WinAppMigrator Client")
        .status();
    // Each line fits the 80 columns Windows 7 gives a console.
    println!("WinAppMigrator Client {}", winappmigrator_client::VERSION);
    println!();
    println!("This lets WinAppMigrator on your new PC read the apps on this PC over your");
    println!("network. It changes nothing here: it reads a snapshot of the Windows drive,");
    println!("which it removes when you stop it.");
    println!();
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            println!();
            println!("The client stopped: {error}");
            println!("Press Enter to close this window.");
            let _ = io::stdin().lock().read_line(&mut String::new());
            ExitCode::FAILURE
        }
    }
}

fn run() -> io::Result<()> {
    let mut state = RunState::begin()?;
    // Kept until this returns, which tells the caretaker the run is over. It then waits for the
    // record to be let go of, on a way out that drops it rather than ending it.
    let _caretaker = start_caretaker()
        .inspect_err(|error| {
            println!("If this window is closed rather than stopped, the snapshot and firewall");
            println!("rules stay until the client is next run here ({error}).");
        })
        .ok();
    print!("Taking a snapshot of the Windows drive... ");
    let snapshot = match Snapshot::take(&mut state) {
        Ok(snapshot) => snapshot,
        Err(error) => {
            state.end(None);
            return Err(error);
        }
    };
    println!("done.");
    let program = std::env::current_exe()?;
    if let Err(error) = open_firewall(&program) {
        state.end(Some(&snapshot));
        return Err(error);
    }
    let about = about_this_pc(&snapshot.taken);
    // `--code 123456` sets the first code, for a script that starts the client and the builder both.
    // It still has to be typed into the builder, and wrong ones still replace it.
    let given = std::env::args()
        .skip_while(|argument| argument != "--code")
        .nth(1)
        .map(|code| {
            code.chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
        })
        .filter(|digits| digits.len() == winappmigrator_link::CODE_DIGITS)
        .map(|digits| format!("{} {}", &digits[..3], &digits[3..]));
    let code = Arc::new(Mutex::new(match given {
        Some(code) => code,
        None => new_code()?,
    }));

    let announcement = Announcement {
        magic: MAGIC.to_owned(),
        version: PROTOCOL_VERSION,
        nonce: 0,
        computer_name: about.computer_name.clone(),
        windows_name: about.windows_name.clone(),
        architecture: about.architecture.clone(),
        port: LINK_PORT,
    };
    let discovery = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, DISCOVERY_PORT));
    match discovery {
        Ok(socket) => {
            thread::spawn(move || answer_queries(&socket, &announcement));
        }
        Err(error) => {
            println!("This PC will not show up by itself ({error}).");
            println!("On the new PC, type this PC's name or address instead.");
        }
    }
    let listener = match TcpListener::bind((Ipv4Addr::UNSPECIFIED, LINK_PORT)) {
        Ok(listener) => listener,
        Err(error) => {
            state.end(Some(&snapshot));
            return Err(error);
        }
    };

    println!();
    let bits = if about.architecture.eq_ignore_ascii_case("x86") {
        "32-bit"
    } else {
        "64-bit"
    };
    println!("  This PC:       {}", about.computer_name);
    println!("  Windows:       {}, {bits}", about.windows_name);
    println!("  Pairing code:  {}", lock(&code));
    println!();
    println!("On your new PC, in WinAppMigrator, choose Network PC... and type this code.");
    println!("Close the app you are moving first, so its files are as it left them.");
    println!("To stop, press Enter here or close this window.");
    println!();

    let root = snapshot.root.clone();
    let serving_code = Arc::clone(&code);
    thread::spawn(move || {
        let mut wrong = 0;
        for stream in listener.incoming() {
            let Ok(stream) = stream else {
                continue;
            };
            let from = stream.peer_addr().map_or_else(
                |_| "another PC".to_owned(),
                |address| address.ip().to_string(),
            );
            let _ = stream.set_read_timeout(Some(IDLE_LIMIT));
            // Each answer goes as soon as it is sealed, rather than waiting to be joined by more.
            let _ = stream.set_nodelay(true);
            let current = lock(&serving_code);
            match pair_as_client(stream, &current) {
                Ok(mut channel) => {
                    wrong = 0;
                    println!("Paired with {from}. It is reading what it needs...");
                    match serve(&mut channel, &root, &about) {
                        Ok(answered) => println!("{from} is done ({answered} requests answered)."),
                        Err(error) => println!("The link to {from} ended: {error}"),
                    }
                }
                Err(LinkError::WrongCode) => {
                    wrong += 1;
                    println!("{from} tried a code that is not this one.");
                    if wrong >= ATTEMPTS_PER_CODE {
                        wrong = 0;
                        if let Ok(fresh) = new_code() {
                            println!("Too many wrong codes. The new pairing code is: {fresh}");
                            if let Ok(mut code) = serving_code.lock() {
                                *code = fresh;
                            }
                        }
                    }
                }
                Err(error) => println!("{from} could not pair: {error}"),
            }
        }
    });

    let _ = io::stdin().lock().read_line(&mut String::new());
    println!("Stopping: removing the snapshot and the firewall rules...");
    state.end(Some(&snapshot));
    println!("Done.");
    Ok(())
}

fn answer_queries(socket: &UdpSocket, announcement: &Announcement) {
    let mut buffer = [0_u8; 2048];
    loop {
        let Ok((length, from)) = socket.recv_from(&mut buffer) else {
            continue;
        };
        if let Some(reply) = answer(&buffer[..length], announcement) {
            let _ = socket.send_to(&reply, from);
        }
    }
}

fn lock(code: &Mutex<String>) -> String {
    code.lock().map(|code| code.clone()).unwrap_or_default()
}
