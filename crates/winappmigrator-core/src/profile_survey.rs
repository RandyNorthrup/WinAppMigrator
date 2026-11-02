//! Which accounts on a PC or drive have data for an app, for choosing whose goes in an installer.
//!
//! A person's files are their own, and an installer holds one account's, so this asks when there
//! is a choice to make. Finding out takes each account's registry, which on a drive or for
//! someone who is not signed in means loading a copy of it, so that is done once, when the list of
//! apps is read. What is kept is only the names of its keys, and asking about an app then takes a
//! look at some folders.

use std::collections::BTreeMap;
use std::path::PathBuf;

use walkdir::WalkDir;
use winreg::enums::KEY_READ;

use crate::errors::Result;
use crate::file_discovery::{
    appdata_bases_of, collect_matching_dirs, names_of_app, user_folders_of,
};
use crate::models::{FootprintKind, InstallScope, InstalledApp};
use crate::path_utils::{SignedInUser, signed_in_user};
use crate::product_match::{MatchScope, ProductNames};
use crate::registry_scan::product_keys_in_view;
use crate::system_context::{ScanSource, SystemContext, UserHive, VolumeMap};

const SOFTWARE: &str = "Software";
const WOW6432_SOFTWARE: &str = r"Software\WOW6432Node";
/// Enough to tell whose key is whose. Classes is enormous, and not named for anyone.
const SKIPPED_SOFTWARE_CHILDREN: &[&str] = &["classes"];

/// What was read about the accounts of a PC or drive.
#[derive(Debug, Clone, Default)]
pub struct ProfileSurvey {
    profiles: Vec<SurveyedProfile>,
    /// Who it is when no one says otherwise. On a drive, this is only a good guess, and is used
    /// when choosing is not needed.
    default_id: Option<String>,
    /// Set for a drive.
    drive: Option<VolumeMap>,
    /// Why whoever is signed in here could not be told.
    unknown_user: Option<String>,
}

#[derive(Debug, Clone)]
struct SurveyedProfile {
    id: String,
    name: String,
    dir: PathBuf,
    documents: Option<PathBuf>,
    saved_games: Option<PathBuf>,
    /// The names of its keys under `Software` and a level below, by the key they are in,
    /// lowercase.
    software_keys: BTreeMap<String, Vec<String>>,
}

/// Find out who has what, loading what it takes, and let it all go again.
pub fn survey_profiles(source: &ScanSource) -> Result<ProfileSurvey> {
    let system = SystemContext::open_every_profile(source)?;
    Ok(survey_of(&system))
}

/// [`survey_profiles`], from a context that already has every account's hive open.
pub fn survey_of(system: &SystemContext) -> ProfileSurvey {
    let mut warnings = Vec::new();
    let profiles = system
        .user_hives()
        .iter()
        .map(|user| {
            let folders = user_folders_of(user, system, &mut warnings);
            SurveyedProfile {
                id: user.id.clone(),
                name: user.name.clone(),
                dir: user.profile_dir.clone(),
                documents: folders.documents,
                saved_games: folders.saved_games,
                software_keys: read_software_keys(user),
            }
        })
        .collect();
    let unknown_user = match (system.is_offline(), signed_in_user()) {
        (false, SignedInUser::Unknown(reason)) => Some(reason.clone()),
        _ => None,
    };
    ProfileSurvey {
        profiles,
        default_id: system.default_profile_id(),
        drive: system.volume_map().cloned(),
        unknown_user,
    }
}

fn read_software_keys(user: &UserHive) -> BTreeMap<String, Vec<String>> {
    let children_of = |key_path: &str| -> Vec<String> {
        user.open(key_path, KEY_READ)
            .map(|key| key.enum_keys().flatten().collect())
            .unwrap_or_default()
    };
    let mut keys = BTreeMap::new();
    for view in [SOFTWARE, WOW6432_SOFTWARE] {
        let children = children_of(view);
        for child in &children {
            if SKIPPED_SOFTWARE_CHILDREN.contains(&child.to_ascii_lowercase().as_str()) {
                continue;
            }
            let path = format!(r"{view}\{child}");
            keys.insert(path.to_ascii_lowercase(), children_of(&path));
        }
        keys.insert(view.to_ascii_lowercase(), children);
    }
    keys
}

/// What to offer for one app.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ProfilePicker {
    pub rows: Vec<ProfileRow>,
    /// The account chosen so far. `None` means no one's data, and that a person has to say.
    pub selected: Option<String>,
    /// What a person ought to be told, whether or not there is anything to choose.
    pub note: Option<String>,
}

/// An account to offer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileRow {
    pub id: String,
    pub name: String,
    pub has_data: bool,
    pub is_default: bool,
}

impl ProfilePicker {
    /// Whether there is a choice to show: more than one account, or one account that no one has
    /// said it should be.
    pub fn offers_a_choice(&self) -> bool {
        self.rows.len() > 1 || (self.selected.is_none() && !self.rows.is_empty())
    }
}

impl ProfileSurvey {
    /// Who can be chosen for an app, and who is chosen unless someone says otherwise. This looks
    /// at some folders, and on a drive that may be failing, so not on a window's thread.
    pub fn picker_for(&self, app: &InstalledApp) -> ProfilePicker {
        // An app installed for one account has that account's settings, whatever anyone else has
        // named after it, and there is no one else's to want.
        if app.install_scope == InstallScope::PerUser
            && let Some(owner) = app.owner_profile.as_deref()
            && let Some(profile) = self
                .profiles
                .iter()
                .find(|profile| profile.id.eq_ignore_ascii_case(owner))
        {
            return ProfilePicker {
                rows: Vec::new(),
                selected: Some(profile.id.clone()),
                note: None,
            };
        }
        let names = names_of_app(
            app.main_executable.as_deref(),
            app.install_location.as_deref(),
            Some(&app.display_name),
            app.publisher.as_deref(),
        );
        let rows: Vec<ProfileRow> = self
            .profiles
            .iter()
            .map(|profile| ProfileRow {
                id: profile.id.clone(),
                name: profile.name.clone(),
                has_data: self.has_data(profile, &names),
                is_default: self
                    .default_id
                    .as_deref()
                    .is_some_and(|id| id.eq_ignore_ascii_case(&profile.id)),
            })
            .collect();
        pick(rows, self.drive.is_some(), self.unknown_user.as_deref())
    }

    fn has_data(&self, profile: &SurveyedProfile, names: &ProductNames) -> bool {
        let children_of = |key_path: &str| -> Vec<String> {
            profile
                .software_keys
                .get(&key_path.to_ascii_lowercase())
                .cloned()
                .unwrap_or_default()
        };
        let has_keys = [SOFTWARE, WOW6432_SOFTWARE]
            .into_iter()
            .any(|view| !product_keys_in_view(names, view, &children_of).is_empty());
        if has_keys {
            return true;
        }
        let mut roots = Vec::new();
        let mut warnings = Vec::new();
        let drive = self.drive.as_ref();
        for (base, kind) in appdata_bases_of(&profile.dir) {
            collect_matching_dirs(
                &base,
                kind,
                names,
                MatchScope::ProgramFolders,
                drive,
                &mut roots,
                &mut warnings,
            );
        }
        for (base, kind) in [
            (profile.documents.as_ref(), FootprintKind::Documents),
            (profile.saved_games.as_ref(), FootprintKind::SavedGames),
        ] {
            if let Some(base) = base {
                collect_matching_dirs(
                    base,
                    kind,
                    names,
                    MatchScope::UserFolders,
                    drive,
                    &mut roots,
                    &mut warnings,
                );
            }
        }
        roots.iter().any(|root| {
            drive.is_none_or(|drive| drive.may_read(&root.path))
                && WalkDir::new(&root.path)
                    .follow_links(false)
                    .into_iter()
                    .filter_entry(|entry| drive.is_none_or(|drive| drive.may_read(entry.path())))
                    .flatten()
                    .any(|entry| entry.file_type().is_file())
        })
    }
}

/// The rules for what to offer. On this PC, whoever is signed in is who it is, and anyone else with
/// data makes it a choice, since nothing of another person's goes in without being asked for. On a
/// drive every account's is the owner's to take, so the one with data needs no asking. When who is
/// signed in cannot be told, that is not guessed at.
fn pick(rows: Vec<ProfileRow>, on_a_drive: bool, unknown_user: Option<&str>) -> ProfilePicker {
    let default = rows
        .iter()
        .find(|row| row.is_default)
        .map(|row| row.id.clone());
    let with_data: Vec<&ProfileRow> = rows.iter().filter(|row| row.has_data).collect();
    if on_a_drive {
        return match with_data.as_slice() {
            [] => ProfilePicker {
                rows: Vec::new(),
                selected: default.or_else(|| match rows.as_slice() {
                    [only] => Some(only.id.clone()),
                    _ => None,
                }),
                note: None,
            },
            [only] => ProfilePicker {
                selected: Some(only.id.clone()),
                note: None,
                rows: Vec::new(),
            },
            many => {
                let selected = many
                    .iter()
                    .find(|row| row.is_default)
                    .or_else(|| many.first())
                    .map(|row| row.id.clone());
                let rows = many.iter().map(|row| (*row).clone()).collect();
                ProfilePicker {
                    rows,
                    selected,
                    note: None,
                }
            }
        };
    }
    if let Some(reason) = unknown_user {
        let note = format!("Could not tell who is signed in ({reason}).");
        return match with_data.as_slice() {
            [] => ProfilePicker {
                rows: Vec::new(),
                selected: None,
                note: Some(note),
            },
            many => {
                let rows = many.iter().map(|row| (*row).clone()).collect();
                ProfilePicker {
                    rows,
                    selected: None,
                    note: Some(format!("{note} Choose whose data to include.")),
                }
            }
        };
    }
    let offered: Vec<ProfileRow> = rows
        .iter()
        .filter(|row| row.has_data || row.is_default)
        .cloned()
        .collect();
    let others_have_data = offered.iter().any(|row| !row.is_default && row.has_data);
    ProfilePicker {
        rows: if others_have_data {
            offered
        } else {
            Vec::new()
        },
        selected: default,
        note: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(id: &str, has_data: bool, is_default: bool) -> ProfileRow {
        ProfileRow {
            id: id.to_owned(),
            name: id.to_owned(),
            has_data,
            is_default,
        }
    }

    #[test]
    fn on_this_pc_it_is_whoever_is_signed_in_unless_someone_else_has_data() {
        let alone = pick(
            vec![row("me", true, true), row("you", false, false)],
            false,
            None,
        );
        assert_eq!(alone.selected.as_deref(), Some("me"));
        assert!(!alone.offers_a_choice(), "no one else has any");

        let shared = pick(
            vec![row("me", true, true), row("you", true, false)],
            false,
            None,
        );
        assert_eq!(shared.selected.as_deref(), Some("me"));
        assert!(shared.offers_a_choice());

        // Someone else's is not taken for not having any of one's own, and it is not hidden.
        let only_theirs = pick(
            vec![
                row("me", false, true),
                row("you", true, false),
                row("them", false, false),
            ],
            false,
            None,
        );
        assert_eq!(only_theirs.selected.as_deref(), Some("me"));
        assert_eq!(only_theirs.rows.len(), 2, "not those with nothing to offer");
    }

    #[test]
    fn on_a_drive_the_only_account_with_data_needs_no_asking() {
        let picker = pick(
            vec![row("old", false, true), row("alice", true, false)],
            true,
            None,
        );
        assert_eq!(picker.selected.as_deref(), Some("alice"));
        assert!(!picker.offers_a_choice());

        let picker = pick(
            vec![
                row("bob", true, false),
                row("alice", true, true),
                row("c", false, false),
            ],
            true,
            None,
        );
        assert_eq!(picker.selected.as_deref(), Some("alice"));
        assert_eq!(picker.rows.len(), 2);

        // With no one to say, the first with data is offered and can be changed.
        let picker = pick(
            vec![row("bob", true, false), row("alice", true, false)],
            true,
            None,
        );
        assert_eq!(picker.selected.as_deref(), Some("bob"));
        assert!(picker.offers_a_choice());

        // No data at all: settings still go by the account that last signed in, or the only one.
        let picker = pick(vec![row("bob", false, false)], true, None);
        assert_eq!(picker.selected.as_deref(), Some("bob"));
        let picker = pick(
            vec![row("a", false, false), row("b", false, false)],
            true,
            None,
        );
        assert_eq!(picker.selected, None);
    }

    #[test]
    fn when_who_is_signed_in_cannot_be_told_that_is_said_and_not_guessed() {
        let picker = pick(
            vec![row("a", true, false), row("b", true, false)],
            false,
            Some("why"),
        );
        assert_eq!(picker.selected, None, "a person has to say");
        assert!(picker.offers_a_choice());
        assert!(
            picker
                .note
                .as_deref()
                .is_some_and(|note| note.contains("why"))
        );

        let picker = pick(
            vec![row("a", true, false), row("b", false, false)],
            false,
            Some("why"),
        );
        assert_eq!(
            picker.selected, None,
            "not even the only one with data, which may be someone else's"
        );
        assert!(picker.offers_a_choice());

        let picker = pick(vec![row("a", false, false)], false, Some("why"));
        assert_eq!(picker.selected, None);
        assert!(picker.note.is_some());
    }

    fn app(scope: InstallScope, owner: Option<&str>) -> InstalledApp {
        InstalledApp {
            display_name: "Demo App".to_owned(),
            publisher: Some("Demo Vendor".to_owned()),
            version: None,
            install_location: None,
            uninstall_string: None,
            main_executable: None,
            registry_source_path: String::new(),
            architecture: crate::models::AppArchitecture::X64,
            install_scope: scope,
            owner_profile: owner.map(ToOwned::to_owned),
        }
    }

    #[test]
    fn who_has_data_is_found_in_their_folders_and_in_their_keys() -> std::io::Result<()> {
        let work = tempfile::tempdir()?;
        let with_files = work.path().join("files");
        std::fs::create_dir_all(with_files.join(r"AppData\Roaming\Demo App"))?;
        std::fs::write(with_files.join(r"AppData\Roaming\Demo App\a.txt"), b"x")?;
        let in_documents = work.path().join("documents");
        std::fs::create_dir_all(in_documents.join(r"OneDrive\Papers\Demo App"))?;
        std::fs::write(in_documents.join(r"OneDrive\Papers\Demo App\a.txt"), b"x")?;
        let empty_folder = work.path().join("empty");
        std::fs::create_dir_all(empty_folder.join(r"AppData\Roaming\Demo App"))?;
        let other_app = work.path().join("other");
        std::fs::create_dir_all(other_app.join(r"AppData\Roaming\Some Other App"))?;
        std::fs::write(
            other_app.join(r"AppData\Roaming\Some Other App\a.txt"),
            b"x",
        )?;
        let with_keys = work.path().join("keys");
        std::fs::create_dir_all(&with_keys)?;

        let profile = |id: &str, dir: &PathBuf| SurveyedProfile {
            id: id.to_owned(),
            name: id.to_owned(),
            dir: dir.clone(),
            documents: None,
            saved_games: None,
            software_keys: BTreeMap::new(),
        };
        let mut documents = profile("documents", &in_documents);
        documents.documents = Some(in_documents.join(r"OneDrive\Papers"));
        let mut keys = profile("keys", &with_keys);
        keys.software_keys
            .insert("software".to_owned(), vec!["Demo Vendor".to_owned()]);
        keys.software_keys.insert(
            r"software\demo vendor".to_owned(),
            vec!["Demo App".to_owned()],
        );
        let survey = ProfileSurvey {
            profiles: vec![
                profile("files", &with_files),
                documents,
                profile("empty", &empty_folder),
                profile("other", &other_app),
                keys,
            ],
            default_id: Some("empty".to_owned()),
            drive: None,
            unknown_user: None,
        };

        let picker = survey.picker_for(&app(InstallScope::PerMachine, None));
        let found: Vec<(&str, bool)> = picker
            .rows
            .iter()
            .map(|row| (row.id.as_str(), row.has_data))
            .collect();
        assert_eq!(
            found,
            [
                ("files", true),
                ("documents", true),
                ("empty", false),
                ("keys", true)
            ],
            "the one signed in is there with nothing, and whoever has none of this app is not"
        );
        assert_eq!(picker.selected.as_deref(), Some("empty"));

        let owned = survey.picker_for(&app(InstallScope::PerUser, Some("FILES")));
        assert_eq!(owned.selected.as_deref(), Some("files"));
        assert!(!owned.offers_a_choice(), "it is that account's app");
        Ok(())
    }
}
