//! The Network PC steps, driven headlessly, and a real pairing with the client's own code over this
//! PC's loopback, which a person would only see as the old PC's apps in the list.

use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::path::{Path, PathBuf};

use egui_kittest::kittest::Queryable;

use winappmigrator_core::{ScanSource, SourceIdentity};
use winappmigrator_link::discovery::{Announcement, Found};
use winappmigrator_link::protocol::About;
use winappmigrator_link::{LINK_PORT, MAGIC, PROTOCOL_VERSION};

use crate::network_view::{NetworkWizard, UsbDrive};
use crate::theme::ThemeMode;
use crate::ui_tests::{harness_with, preview};

const CODE: &str = "482913";

fn found(name: &str, windows: &str, last: u8) -> Found {
    Found {
        address: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, last)), LINK_PORT),
        announcement: Announcement {
            magic: MAGIC.to_owned(),
            version: PROTOCOL_VERSION,
            nonce: 0,
            computer_name: name.to_owned(),
            windows_name: windows.to_owned(),
            architecture: "AMD64".to_owned(),
            port: LINK_PORT,
        },
    }
}

fn wizard() -> NetworkWizard {
    let mut wizard = NetworkWizard::default();
    wizard.usb_drives = vec![UsbDrive {
        root: PathBuf::from(r"E:\"),
        label: "KINGSTON".to_owned(),
        free_bytes: 31_000_000_000,
    }];
    wizard
}

#[test]
fn the_steps_show_until_a_pc_is_paired() {
    for (mode, name) in [(ThemeMode::Dark, "dark"), (ThemeMode::Light, "light")] {
        let mut harness = harness_with(mode, |app| app.network = Some(wizard()));
        assert!(
            harness
                .query_by_label_contains("1. Put the client on the old PC")
                .is_some()
        );
        assert!(
            harness
                .query_by_label_contains("Save to USB drive E:")
                .is_some()
        );
        assert!(
            harness
                .query_by_label_contains("Looking on your network")
                .is_some()
        );
        assert!(harness.query_by_label("Connect").is_some());
        // The list of this PC's apps is not what is shown while another is being paired with.
        assert!(harness.query_by_label_contains("Contoso Notes").is_none());
        preview(&mut harness, &format!("network-steps-{name}"));
    }
}

#[test]
fn a_pc_found_on_the_network_is_offered_with_its_windows() {
    for (mode, name) in [(ThemeMode::Dark, "dark"), (ThemeMode::Light, "light")] {
        let mut harness = harness_with(mode, |app| {
            let mut wizard = wizard();
            wizard.found = vec![
                found("OLD-PC", "Windows 7 Professional Service Pack 1", 20),
                found("DEN-LAPTOP", "Windows 10 Home", 31),
            ];
            wizard.chosen = Some(wizard.found[0].address);
            wizard.code = "482 913".to_owned();
            wizard.saved = Some(Ok(PathBuf::from(r"E:\WinAppMigrator Client.exe")));
            app.network = Some(wizard);
        });
        assert!(
            harness
                .query_by_label_contains("OLD-PC  ·  Windows 7 Professional Service Pack 1")
                .is_some()
        );
        assert!(harness.query_by_label_contains("Saved as E:").is_some());
        preview(&mut harness, &format!("network-found-{name}"));
    }
}

/// A client, as it runs on the old PC, serving a folder of this one over loopback.
fn serve_a_folder(root: PathBuf) -> Option<SocketAddr> {
    let listener = TcpListener::bind("127.0.0.1:0").ok()?;
    let address = listener.local_addr().ok()?;
    std::thread::spawn(move || {
        let Ok((stream, _)) = listener.accept() else {
            return;
        };
        let about = About {
            client_version: winappmigrator_client::VERSION.to_owned(),
            computer_name: "OLD-PC".to_owned(),
            windows_name: "Windows 7 Professional Service Pack 1".to_owned(),
            architecture: "AMD64".to_owned(),
            snapshot_taken: String::new(),
        };
        if let Ok(mut channel) = winappmigrator_link::channel::pair_as_client(stream, CODE) {
            let _ = winappmigrator_client::serve::serve(&mut channel, &root, &about);
        }
    });
    Some(address)
}

#[test]
fn once_paired_the_old_pcs_drive_is_the_source_and_says_so() {
    let folder = std::env::temp_dir().join(format!("wam-network-ui-{}", std::process::id()));
    let _ = std::fs::create_dir_all(folder.join("Windows"));
    let Some(address) = serve_a_folder(folder.clone()) else {
        return;
    };
    let remote = winappmigrator_core::remote::connect(address, CODE);
    let Ok(remote) = remote else {
        panic!("pairing over loopback failed: {:?}", remote.err());
    };
    // What the app reads of it goes through the link: the folder served is there.
    assert!(winappmigrator_core::source_fs::is_dir(
        remote.root().join("Windows")
    ));
    assert!(!winappmigrator_core::source_fs::exists(Path::new(
        r"\\?\WAMLINK-none\Windows"
    )));
    for (mode, name) in [(ThemeMode::Dark, "dark"), (ThemeMode::Light, "light")] {
        let paired = std::sync::Arc::clone(&remote);
        let mut harness = harness_with(mode, move |app| {
            app.network = Some(wizard());
            app.scan_source = ScanSource::offline(paired.root());
            app.remote = Some(paired);
            app.source_windows = Some(SourceIdentity {
                computer_name: None,
                windows_version: None,
                windows_build: Some(7601),
                architecture_name: Some("AMD64".to_owned()),
                product_name: Some("Windows 7 Professional".to_owned()),
                service_pack: Some("Service Pack 1".to_owned()),
            });
        });
        assert!(
            harness
                .query_by_label_contains("OLD-PC over the network (127.0.0.1)")
                .is_some()
        );
        assert!(harness.query_by_label("Disconnect").is_some());
        preview(&mut harness, &format!("network-paired-{name}"));
        assert!(harness.query_by_label_contains("Contoso Notes").is_some());
    }
    winappmigrator_core::remote::disconnect(&remote);
    let _ = std::fs::remove_dir_all(&folder);
}
