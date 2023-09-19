use std::path::{Path, PathBuf};

#[derive(serde::Serialize, Debug, serde::Deserialize, Clone, Copy, PartialEq, Eq)]
/// Raws are part of modules since 50.xx. Raw modules are loaded from 3 common locations:
/// `{df_directory}/data/vanilla`, `{df_directory}/mods`, and `{df_directory/data/installed_mods}`
pub enum RawModuleLocation {
    InstalledMods,
    Mods,
    Vanilla,
    Unknown,
}
impl RawModuleLocation {
    pub fn get_path(self) -> PathBuf {
        match self {
            RawModuleLocation::Mods => PathBuf::from("mods"),
            RawModuleLocation::InstalledMods => ["data", "installed_mods"].iter().collect(),
            RawModuleLocation::Vanilla => ["data", "vanilla"].iter().collect(),
            RawModuleLocation::Unknown => PathBuf::from("unknown"),
        }
    }
    pub fn from_sourced_directory(sourced_directory: &str) -> Self {
        match sourced_directory {
            "mods" => RawModuleLocation::Mods,
            "vanilla" => RawModuleLocation::Vanilla,
            "installed_mods" => RawModuleLocation::InstalledMods,
            _ => {
                log::warn!(
                    "RawModuleLocation - Unable to match source directory \"{dir}\"",
                    dir = sourced_directory
                );
                RawModuleLocation::Unknown
            }
        }
    }
    pub fn from_info_text_file_path<P: AsRef<Path>>(full_path: &P) -> Self {
        // info.txt is relative by 2 parents from our module location
        // <MODULE LOCATION>/<RAW MODULE>/info.txt
        match full_path.as_ref().parent() {
            Some(parent_dir) => match parent_dir.parent() {
                Some(grandparent_dir) => {
                    let path_string = String::from(
                        grandparent_dir
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy(),
                    );
                    return Self::from_sourced_directory(path_string.as_str());
                }
                None => RawModuleLocation::Unknown,
            },
            None => RawModuleLocation::Unknown,
        }
    }
}
