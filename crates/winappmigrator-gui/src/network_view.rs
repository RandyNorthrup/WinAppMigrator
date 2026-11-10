//! **Network PC…**: reading another PC through WinAppMigrator Client.
//!
//! Until a PC is paired, the list of apps is this, in three steps: put the client on the old PC, find
//! it, and type the code it shows. Once paired, that PC's Windows drive is the scan source, read
//! through the client by `winappmigrator_core::remote`, and its apps are listed as any drive's are.

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

use eframe::egui::{self, RichText};

use winappmigrator_core::path_utils::{POWERSHELL_UTF8_OUTPUT, powershell_command};
use winappmigrator_core::remote::{self, RemotePc};
use winappmigrator_link::discovery::Found;
use winappmigrator_ui::theme::{Role, SPACE_MD, SPACE_SM, SPACE_XS, Theme};
use winappmigrator_ui::widgets::{self, ButtonKind};

use crate::WinAppMigratorApp;

/// What the client is saved as, and the note beside it.
pub(crate) const CLIENT_FILE_NAME: &str = "WinAppMigrator Client.exe";
const CLIENT_NOTE_NAME: &str = "WinAppMigrator Client - start here.txt";
const CLIENT_NOTE: &str = "WinAppMigrator Client\r\n\
\r\n\
Run this on the PC you are moving apps off: Windows 7, 8, 10 or 11.\r\n\
\r\n\
1. Close the app you are moving.\r\n\
2. Double-click \"WinAppMigrator Client.exe\" and let it make changes.\r\n\
3. It shows a 6-digit pairing code. In WinAppMigrator on your new PC, choose\r\n\
   Network PC..., pick this PC, and type the code.\r\n\
4. When the new PC is done, press Enter in the client's window, or close it.\r\n\
\r\n\
It only reads. While it runs it keeps a snapshot of the Windows drive and lets\r\n\
its own two ports through Windows Firewall from your local network, and it\r\n\
removes both when it stops. What crosses the network is encrypted, and only a\r\n\
PC that has the code can read anything.\r\n";
/// How long to wait between asking the network who is there.
const SEARCH_EVERY: Duration = Duration::from_secs(5);
/// What lists the USB drives plugged in: drive type 2 is removable.
const USB_DRIVES_SCRIPT: &str = "Get-CimInstance Win32_LogicalDisk -Filter 'DriveType=2' | ForEach-Object { $_.DeviceID + '|' + $_.VolumeName + '|' + $_.FreeSpace }";

/// The client, built for Windows 7 and later, carried by the app (see `build.rs`). Empty in a
/// development build made before it.
pub(crate) const EMBEDDED_CLIENT: &[u8] = include_bytes!(env!("WAM_CLIENT"));

#[cfg(not(debug_assertions))]
const _: () = assert!(!EMBEDDED_CLIENT.is_empty());

/// A USB drive to save the client to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct UsbDrive {
    pub root: PathBuf,
    pub label: String,
    pub free_bytes: u64,
}

/// Where the wizard is.
#[derive(Default)]
pub(crate) struct NetworkWizard {
    pub(crate) usb_drives: Vec<UsbDrive>,
    usb_receiver: Option<Receiver<Vec<UsbDrive>>>,
    /// Where the client was saved, or why it could not be.
    pub(crate) saved: Option<Result<PathBuf, String>>,
    pub(crate) found: Vec<Found>,
    finding: Option<Receiver<Vec<Found>>>,
    last_search: Option<Instant>,
    pub(crate) chosen: Option<SocketAddr>,
    pub(crate) typed: String,
    pub(crate) code: String,
    connecting: Option<Receiver<Result<Arc<RemotePc>, String>>>,
    pub(crate) error: Option<String>,
    /// Whether to keep asking the network who is there, which a window does and a test does not.
    looking: bool,
}

impl NetworkWizard {
    pub(crate) fn new(ctx: &egui::Context) -> Self {
        let mut wizard = Self {
            looking: true,
            ..Self::default()
        };
        wizard.look_for_usb_drives(ctx);
        wizard
    }

    fn look_for_usb_drives(&mut self, ctx: &egui::Context) {
        let (sender, receiver) = mpsc::channel();
        self.usb_receiver = Some(receiver);
        let ctx = ctx.clone();
        thread::spawn(move || {
            let _ = sender.send(usb_drives());
            ctx.request_repaint();
        });
    }

    fn search(&mut self, ctx: &egui::Context) {
        let (sender, receiver) = mpsc::channel();
        self.finding = Some(receiver);
        self.last_search = Some(Instant::now());
        let ctx = ctx.clone();
        thread::spawn(move || {
            let _ = sender.send(remote::find_clients());
            ctx.request_repaint();
        });
    }

    pub(crate) fn is_connecting(&self) -> bool {
        self.connecting.is_some()
    }
}

/// The USB drives plugged in, by what Windows says of each.
fn usb_drives() -> Vec<UsbDrive> {
    let Ok(output) =
        powershell_command(format!("{POWERSHELL_UTF8_OUTPUT}{USB_DRIVES_SCRIPT}")).output()
    else {
        return Vec::new();
    };
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let mut parts = line.trim().split('|');
            let device = parts.next()?.trim();
            let label = parts.next().unwrap_or_default().trim().to_owned();
            let free_bytes = parts
                .next()
                .and_then(|free| free.trim().parse().ok())
                .unwrap_or(0);
            (device.len() == 2 && device.ends_with(':')).then(|| UsbDrive {
                root: PathBuf::from(format!("{device}\\")),
                label,
                free_bytes,
            })
        })
        .collect()
}

/// Save the client, and a note on how to use it, in a folder. It replaces a client saved there
/// before.
pub(crate) fn save_client(folder: &Path) -> Result<PathBuf, String> {
    if EMBEDDED_CLIENT.is_empty() {
        return Err(
            "this build of WinAppMigrator does not carry the client. Build it with \
             scripts\\build-client.ps1, then build the app again."
                .to_owned(),
        );
    }
    let client = folder.join(CLIENT_FILE_NAME);
    let partial = folder.join(format!("{CLIENT_FILE_NAME}.partial"));
    std::fs::write(&partial, EMBEDDED_CLIENT)
        .and_then(|()| std::fs::rename(&partial, &client))
        .map_err(|error| {
            let _ = std::fs::remove_file(&partial);
            format!(
                "the client could not be saved in {}: {error}",
                folder.display()
            )
        })?;
    std::fs::write(folder.join(CLIENT_NOTE_NAME), CLIENT_NOTE)
        .map_err(|error| format!("the note beside it could not be saved: {error}"))?;
    Ok(client)
}

impl WinAppMigratorApp {
    /// Read apps from another PC through its client.
    pub(crate) fn select_network_source(&mut self) {
        self.source_error = None;
        self.image_receiver = None;
        if self.network.is_none() {
            self.network = Some(NetworkWizard::new(&self.ctx));
        }
    }

    /// Stop reading another PC, which ends the client's session there.
    pub(crate) fn leave_network_source(&mut self) {
        if let Some(remote) = self.remote.take() {
            remote::disconnect(&remote);
        }
        self.network = None;
    }

    pub(crate) fn poll_network(&mut self) {
        let ctx = self.ctx.clone();
        let Some(wizard) = self.network.as_mut() else {
            return;
        };
        if let Some(receiver) = wizard.usb_receiver.take() {
            match receiver.try_recv() {
                Ok(drives) => wizard.usb_drives = drives,
                Err(TryRecvError::Empty) => wizard.usb_receiver = Some(receiver),
                Err(TryRecvError::Disconnected) => {}
            }
        }
        if self.remote.is_none() {
            if let Some(receiver) = wizard.finding.take() {
                match receiver.try_recv() {
                    Ok(found) => {
                        if wizard.chosen.is_none() && found.len() == 1 {
                            wizard.chosen = found.first().map(|client| client.address);
                        }
                        wizard.found = found;
                    }
                    Err(TryRecvError::Empty) => wizard.finding = Some(receiver),
                    Err(TryRecvError::Disconnected) => {}
                }
            }
            let due = wizard
                .last_search
                .is_none_or(|last| last.elapsed() >= SEARCH_EVERY);
            if wizard.looking {
                if wizard.finding.is_none() && due {
                    wizard.search(&ctx);
                }
                ctx.request_repaint_after(SEARCH_EVERY);
            }
        }
        let mut paired = None;
        if let Some(receiver) = wizard.connecting.take() {
            match receiver.try_recv() {
                Ok(Ok(remote)) => paired = Some(remote),
                Ok(Err(error)) => wizard.error = Some(error),
                Err(TryRecvError::Empty) => wizard.connecting = Some(receiver),
                Err(TryRecvError::Disconnected) => {
                    wizard.error = Some("pairing stopped before it was done".to_owned());
                }
            }
        }
        if let Some(remote) = paired {
            wizard.code.clear();
            wizard.error = None;
            self.release_disk_image();
            self.scan_source = winappmigrator_core::ScanSource::offline(remote.root());
            self.remote = Some(remote);
            self.start_inventory_load();
        }
    }

    fn pair(&mut self, address: SocketAddr) {
        let ctx = self.ctx.clone();
        let Some(wizard) = self.network.as_mut() else {
            return;
        };
        let code = wizard.code.clone();
        let (sender, receiver) = mpsc::channel();
        wizard.connecting = Some(receiver);
        wizard.error = None;
        thread::spawn(move || {
            let result = remote::connect(address, &code).map_err(|error| {
                if error.kind() == std::io::ErrorKind::PermissionDenied {
                    "that is not the code the old PC shows. Check it and try again.".to_owned()
                } else {
                    format!("the old PC could not be reached: {error}")
                }
            });
            let _ = sender.send(result);
            ctx.request_repaint();
        });
    }

    /// The three steps, in place of the list of apps.
    pub(crate) fn draw_network_wizard(&mut self, ui: &mut egui::Ui, theme: &Theme) {
        let mut save_to: Option<PathBuf> = None;
        let mut pick_folder = false;
        let mut refresh_drives = false;
        let mut pair_with: Option<SocketAddr> = None;
        let Some(wizard) = self.network.as_mut() else {
            return;
        };
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                widgets::card_titled(ui, theme, "1. Put the client on the old PC", None, |ui| {
                    widgets::hint(
                        ui,
                        theme,
                        "WinAppMigrator Client runs on the PC the app is on now, Windows 7 or \
                         later. It only lets this PC read it, encrypted, by a code it shows.",
                    );
                    ui.add_space(SPACE_SM);
                    widgets::control_row(ui, |ui| {
                        for drive in &wizard.usb_drives {
                            let label = if drive.label.is_empty() {
                                format!("Save to USB drive {}", drive.root.display())
                            } else {
                                format!(
                                    "Save to USB drive {} ({})",
                                    drive.root.display(),
                                    drive.label
                                )
                            };
                            if widgets::button(ui, theme, ButtonKind::Primary, &label, true)
                                .clicked()
                            {
                                save_to = Some(drive.root.clone());
                            }
                        }
                        if widgets::button(ui, theme, ButtonKind::Secondary, "Save to a folder…", true)
                            .clicked()
                        {
                            pick_folder = true;
                        }
                        if widgets::button(ui, theme, ButtonKind::Ghost, "Look for USB drives again", true)
                            .clicked()
                        {
                            refresh_drives = true;
                        }
                    });
                    if wizard.usb_drives.is_empty() {
                        ui.add_space(SPACE_XS);
                        widgets::hint(
                            ui,
                            theme,
                            "No USB drive is plugged in. Plug one in, or save to a folder: one you \
                             can reach from the old PC, such as a shared folder on your network.",
                        );
                    }
                    match &wizard.saved {
                        Some(Ok(client)) => {
                            ui.add_space(SPACE_SM);
                            widgets::notice(
                                ui,
                                theme,
                                Role::Success,
                                &format!(
                                    "Saved as {}. On the old PC, close the app you are moving, then \
                                     double-click it and let it make changes. It shows a code.",
                                    client.display()
                                ),
                            );
                        }
                        Some(Err(error)) => {
                            ui.add_space(SPACE_SM);
                            widgets::notice(ui, theme, Role::Danger, error);
                        }
                        None => {}
                    }
                });
                ui.add_space(SPACE_MD);
                widgets::card_titled(ui, theme, "2. Pick the old PC", None, |ui| {
                    if wizard.found.is_empty() {
                        busy_line(ui, theme, "Looking on your network for PCs running the client…");
                    }
                    for client in &wizard.found {
                        let about = &client.announcement;
                        let label = format!(
                            "{}  ·  {}  ·  {}",
                            about.computer_name,
                            about.windows_name,
                            client.address.ip()
                        );
                        let chosen = wizard.chosen == Some(client.address);
                        if ui.radio(chosen, RichText::new(label).size(14.0)).clicked() {
                            wizard.chosen = Some(client.address);
                            wizard.typed.clear();
                        }
                    }
                    ui.add_space(SPACE_SM);
                    widgets::control_row(ui, |ui| {
                        widgets::field_label(ui, theme, "Or type its name or address");
                        let typed = widgets::text_field(ui, &mut wizard.typed, "OLD-PC or 192.168.1.20", 220.0);
                        if typed.changed() && !wizard.typed.trim().is_empty() {
                            wizard.chosen = None;
                        }
                    });
                });
                ui.add_space(SPACE_MD);
                widgets::card_titled(ui, theme, "3. Type the code it shows", None, |ui| {
                    let ready = (wizard.chosen.is_some() || !wizard.typed.trim().is_empty())
                        && wizard.code.chars().filter(char::is_ascii_digit).count()
                            == winappmigrator_link::CODE_DIGITS
                        && !wizard.is_connecting();
                    widgets::control_row(ui, |ui| {
                        widgets::field_label(ui, theme, "Pairing code");
                        widgets::text_field(ui, &mut wizard.code, "123 456", 120.0);
                        if widgets::button(ui, theme, ButtonKind::Primary, "Connect", ready).clicked() {
                            match wizard.chosen {
                                Some(address) => pair_with = Some(address),
                                None => match remote::address_of(&wizard.typed) {
                                    Ok(address) => pair_with = Some(address),
                                    Err(error) => wizard.error = Some(error.to_string()),
                                },
                            }
                        }
                    });
                    if wizard.is_connecting() {
                        busy_line(ui, theme, "Pairing…");
                    }
                    if let Some(error) = &wizard.error {
                        ui.add_space(SPACE_SM);
                        widgets::notice(ui, theme, Role::Danger, error);
                    }
                });
            });
        if refresh_drives {
            wizard.look_for_usb_drives(&self.ctx);
        }
        if pick_folder {
            save_to = rfd::FileDialog::new()
                .set_title("Save WinAppMigrator Client in…")
                .pick_folder();
            let _ = std::env::set_current_dir(winappmigrator_core::path_utils::system_root());
        }
        if let (Some(folder), Some(wizard)) = (save_to, self.network.as_mut()) {
            wizard.saved = Some(save_client(&folder));
        }
        if let Some(address) = pair_with {
            self.pair(address);
        }
    }
}

fn busy_line(ui: &mut egui::Ui, theme: &Theme, text: &str) {
    ui.horizontal(|ui| {
        ui.spinner();
        ui.label(RichText::new(text).size(13.0).color(theme.text_muted));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_client_and_its_note_are_saved_side_by_side() {
        let folder = std::env::temp_dir().join(format!("wam-client-save-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        if std::fs::create_dir_all(&folder).is_err() {
            return;
        }
        let saved = save_client(&folder);
        if EMBEDDED_CLIENT.is_empty() {
            assert!(saved.is_err_and(|error| error.contains("does not carry the client")));
        } else {
            assert_eq!(saved.ok(), Some(folder.join(CLIENT_FILE_NAME)));
            assert_eq!(
                std::fs::read(folder.join(CLIENT_FILE_NAME))
                    .ok()
                    .map(|bytes| bytes.len()),
                Some(EMBEDDED_CLIENT.len())
            );
            assert!(folder.join(CLIENT_NOTE_NAME).is_file());
            // Again, over the first.
            assert!(save_client(&folder).is_ok());
        }
        let _ = std::fs::remove_dir_all(&folder);
    }
}
