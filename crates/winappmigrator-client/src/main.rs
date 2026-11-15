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
use winappmigrator_client::{CODE, HELP, LANG, argument_after};
use winappmigrator_i18n::t;
use winappmigrator_link::channel::pair_as_client;
use winappmigrator_link::discovery::{Announcement, answer};
use winappmigrator_link::{DISCOVERY_PORT, LINK_PORT, LinkError, MAGIC, PROTOCOL_VERSION};

/// Wrong codes a code is given before it is replaced, so that guessing it gets nowhere.
const ATTEMPTS_PER_CODE: u32 = 5;
/// Wrong codes a run is given in all, after which it stops letting anything pair. Each guess has one
/// chance in a million, so this many keeps the chance of a guess getting in at one in fifty thousand,
/// however long the client is left running.
const WRONG_CODES_PER_RUN: u32 = 20;
/// How long a wrong code holds up the next try.
const WRONG_CODE_PAUSE: Duration = Duration::from_secs(2);
/// How long pairing may take. A PC that connects and says nothing would otherwise keep any other
/// from pairing for as long as it liked.
const PAIRING_LIMIT: Duration = Duration::from_secs(15);
/// How long a paired builder may go without asking anything before the link is closed.
const IDLE_LIMIT: Duration = Duration::from_secs(30 * 60);
/// How long an answer may wait for a builder to take it.
const SEND_LIMIT: Duration = Duration::from_secs(120);
/// What a PC that connected is called when its address cannot be told, which each message words as
/// "another PC".
const UNKNOWN_PC: &str = "unknown";

fn main() -> ExitCode {
    // Started in the folder it was run from, which can be a USB drive or a share anyone can write to,
    // and which nothing here needs.
    let _ =
        std::env::set_current_dir(winappmigrator_client::system::windows_folder().join("System32"));
    if std::env::args().nth(1).as_deref() == Some(winappmigrator_client::system::CARETAKER) {
        winappmigrator_client::system::take_care();
        return ExitCode::SUCCESS;
    }
    // Run from a folder only an administrator can change, before anything here can have Windows
    // load a DLL by name: see `safe_folder`. The copy is given the same arguments, `--lang` too, and
    // chooses its language itself, so this one only does if the copy cannot be run.
    if !winappmigrator_client::system::runs_from_safe_folder() {
        return match winappmigrator_client::system::run_from_safe_folder() {
            Ok(status) if status.success() => ExitCode::SUCCESS,
            Ok(_) => ExitCode::FAILURE,
            Err(error) => {
                speak();
                println!(
                    "{}",
                    t!("client-safe-folder-failed", error = error.to_string())
                );
                println!("{}", t!("client-press-enter-to-close"));
                let _ = io::stdin().lock().read_line(&mut String::new());
                ExitCode::FAILURE
            }
        };
    }
    speak();
    // Waits for Enter, as an elevated program started from a console that is not elevated gets a
    // window of its own, which would otherwise close before this could be read.
    if std::env::args()
        .skip(1)
        .any(|argument| HELP.contains(&argument.as_str()))
    {
        println!("{}", t!("client-usage"));
        println!();
        println!("{}", t!("client-press-enter-to-close"));
        let _ = io::stdin().lock().read_line(&mut String::new());
        return ExitCode::SUCCESS;
    }
    // The window says what it is, rather than whatever started it. `title` changes the console it
    // shares with this, and nothing else. Raw, since `title` keeps any quotes it is given.
    let _ = winappmigrator_client::system::windows_tool("cmd.exe")
        .args(["/d", "/c"])
        .raw_arg("title WinAppMigrator Client")
        .status();
    // Each line fits the 80 columns Windows 7 gives a console: a message's lines are broken where its
    // language breaks them, to fit.
    println!(
        "{}",
        t!("client-heading", version = winappmigrator_client::VERSION)
    );
    println!();
    println!("{}", t!("client-intro"));
    println!();
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            println!();
            println!("{}", t!("client-stopped", error = error.to_string()));
            println!("{}", t!("client-press-enter-to-close"));
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
            println!(
                "{}",
                t!("client-caretaker-failed", error = error.to_string())
            );
        })
        .ok();
    print!("{}", t!("client-snapshot-taking"));
    let snapshot = match Snapshot::take(&mut state) {
        Ok(snapshot) => snapshot,
        Err(error) => {
            state.end(None);
            return Err(error);
        }
    };
    println!("{}", t!("client-snapshot-taken"));
    let program = std::env::current_exe()?;
    if let Err(error) = open_firewall(&program) {
        state.end(Some(&snapshot));
        return Err(error);
    }
    let about = about_this_pc(&snapshot.taken);
    // `--code 123456` sets the first code, for a script that starts the client and the builder both.
    // It still has to be typed into the builder, and wrong ones still replace it.
    let given = argument_after(std::env::args(), CODE)
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
            println!(
                "{}",
                t!("client-discovery-failed", error = error.to_string())
            );
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
    let bits: u32 = if about.architecture.eq_ignore_ascii_case("x86") {
        32
    } else {
        64
    };
    println!(
        "{}",
        t!(
            "client-this-pc",
            computer_name = about.computer_name.as_str()
        )
    );
    println!(
        "{}",
        t!(
            "client-windows",
            windows_name = about.windows_name.as_str(),
            bits = bits
        )
    );
    println!("{}", t!("client-pairing-code", code = lock(&code)));
    println!();
    println!("{}", t!("client-pairing-steps"));
    println!();

    let root = snapshot.root.clone();
    let serving_code = Arc::clone(&code);
    thread::spawn(move || {
        let mut wrong = 0;
        let mut wrong_in_all = 0;
        for stream in listener.incoming() {
            let Ok(stream) = stream else {
                continue;
            };
            let from = stream.peer_addr().map_or_else(
                |_| UNKNOWN_PC.to_owned(),
                |address| address.ip().to_string(),
            );
            let _ = stream.set_read_timeout(Some(PAIRING_LIMIT));
            let _ = stream.set_write_timeout(Some(PAIRING_LIMIT));
            // Each answer goes as soon as it is sealed, rather than waiting to be joined by more.
            let _ = stream.set_nodelay(true);
            let current = lock(&serving_code);
            match pair_as_client(stream, &current) {
                Ok(mut channel) => {
                    wrong = 0;
                    let _ = channel.stream().set_read_timeout(Some(IDLE_LIMIT));
                    let _ = channel.stream().set_write_timeout(Some(SEND_LIMIT));
                    println!("{}", t!("client-paired", from = from.as_str()));
                    match serve(&mut channel, &root, &about) {
                        Ok(answered) => println!(
                            "{}",
                            t!(
                                "client-builder-done",
                                from = from.as_str(),
                                count = answered
                            )
                        ),
                        Err(error) => println!(
                            "{}",
                            t!(
                                "client-link-ended",
                                from = from.as_str(),
                                error = error.to_string()
                            )
                        ),
                    }
                }
                Err(LinkError::WrongCode) => {
                    wrong += 1;
                    wrong_in_all += 1;
                    println!("{}", t!("client-wrong-code", from = from.as_str()));
                    if wrong_in_all >= WRONG_CODES_PER_RUN {
                        println!();
                        println!("{}", t!("client-too-many-wrong-codes"));
                        return;
                    }
                    thread::sleep(WRONG_CODE_PAUSE);
                    if wrong >= ATTEMPTS_PER_CODE {
                        wrong = 0;
                        if let Ok(fresh) = new_code() {
                            println!("{}", t!("client-new-code", code = fresh.as_str()));
                            if let Ok(mut code) = serving_code.lock() {
                                *code = fresh;
                            }
                        }
                    }
                }
                Err(error) => println!(
                    "{}",
                    t!(
                        "client-pair-failed",
                        from = from.as_str(),
                        error = error.to_string()
                    )
                ),
            }
        }
    });

    let _ = io::stdin().lock().read_line(&mut String::new());
    println!("{}", t!("client-stopping"));
    state.end(Some(&snapshot));
    println!("{}", t!("client-stop-done"));
    Ok(())
}

/// Speak the language `--lang` asks for, or Windows' display language, saying first, in English, if
/// the console cannot show it.
fn speak() {
    let speaking = winappmigrator_client::speak(argument_after(std::env::args(), LANG).as_deref());
    if let Some(note) = winappmigrator_client::console_note(&speaking) {
        println!("{note}");
    }
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
