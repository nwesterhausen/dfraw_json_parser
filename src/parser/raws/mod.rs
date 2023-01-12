use std::path::Path;

use slug::slugify;

use self::info::DFInfoFile;

pub mod biomes;
pub mod creature;
pub mod environment;
pub mod info;
pub mod inorganic;
pub mod material;
pub mod names;
pub mod plant;
pub mod roll_chance;
pub mod tags;
pub mod temperatures;

#[derive(serde::Serialize, Debug, serde::Deserialize, Clone, Copy)]
/// There are multiple types of raws, these are the different types handled by dfraw_json_parser.
pub enum RawObjectKind {
    Creature,
    Inorganic,
    Plant,
    Material,
    None,
}

#[derive(serde::Serialize, Debug, serde::Deserialize, Clone, Copy)]
/// Raws are part of modules since 50.xx. Raw modules are loaded from 3 common locations:
/// `{df_directory}/data/vanilla`, `{df_directory}/mods`, and `{df_directory/data/installed_mods}`
pub enum RawModuleLocation {
    InstalledMods,
    Mods,
    Vanilla,
    Unknown,
}

#[derive(Debug, Clone)]
/// Struct to contain what common raw values there are, so it can be used
/// within the other structs and only get changed in one place etc.
pub struct DFRawCommon {
    /// The identifier is typically the 2nd half of the top line for the raw definition.
    /// [ITEM_WEAPON:ITEM_WEAPON_SHORT_SWORD] => "ITEM_WEAPON_SHORT_SWORD"
    /// [CREATURE:TOAD] => "TOAD"
    identifier: String,
    /// The raw-defined name of the raw file this raw is located in. This is defined
    /// at the top of the raw file (the first line of text)
    parent_raw: String,
    /// Raws are part of modules since 50.xx. They have their own identifier defined in info.txt
    /// in the root of the module directory.
    dfraw_identifier: String,
    /// Raws are part of modules since 50.xx. The version of the raw module is defined in info.txt
    /// in the root of the module directory.
    dfraw_version: String,
    /// Raws are part of modules since 50.xx. Raw modules are loaded from 3 common locations, and this
    /// refers to where the raw module was located..
    dfraw_found_in: RawModuleLocation,
    /// Raws are part of modules since 50.xx. This should hold a human-readable `{name} v{version}` for
    /// display purposes.
    dfraw_display: String,
    /// Raws are part of modules since 50.xx. To help located the raw file read from, this has the
    /// raw module's directory name.
    dfraw_relative_path: String,
    /// Raws can be of many different types, this helps differentiate them when reading from
    /// a large bucket of "raws".
    raw_type: RawObjectKind,
}

impl DFRawCommon {
    pub fn from(id: &str, raw: &str, info_txt: &DFInfoFile, variant: RawObjectKind) -> Self {
        Self {
            identifier: String::from(id),
            parent_raw: String::from(raw),
            dfraw_identifier: String::from(info_txt.get_identifier()),
            dfraw_version: String::from(info_txt.displayed_version.as_str()),
            dfraw_found_in: info_txt.get_location(),
            dfraw_display: format!("{} v{}", info_txt.name, info_txt.displayed_version),
            dfraw_relative_path: String::from(info_txt.get_parent_directory()),
            raw_type: variant,
        }
    }

    pub fn get_identifier(&self) -> String {
        String::from(&self.identifier)
    }
    pub fn get_raw_module(&self) -> String {
        String::from(&self.dfraw_identifier)
    }
    pub fn get_raw_module_version(&self) -> String {
        String::from(&self.dfraw_version)
    }
    pub fn get_dfraw_found_in(&self) -> RawModuleLocation {
        self.dfraw_found_in
    }
    pub fn get_dfraw_display(&self) -> String {
        String::from(&self.dfraw_display)
    }
    pub fn get_dfraw_relative_path(&self) -> String {
        String::from(&self.dfraw_relative_path)
    }
    pub fn get_parent_raw(&self) -> String {
        String::from(&self.parent_raw)
    }
    pub fn get_raw_type(&self) -> String {
        format!("{:?}", self.raw_type)
    }
    pub fn get_object_id(&self) -> String {
        format!(
            "{}-{}-{}",
            self.get_parent_raw(),
            "PLANT",
            slugify(self.get_identifier())
        )
    }
}

impl RawModuleLocation {
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
                return RawModuleLocation::Unknown;
            }
        }
    }
    pub fn from_info_text_file_path(full_path: &Path) -> Self {
        // info.txt is relative by 2 parents from our module location
        // <MODULE LOCATION>/<RAW MODULE>/info.txt
        match full_path.parent() {
            Some(parent_dir) => match parent_dir.parent() {
                Some(grandparent_dir) => {
                    let path_string = String::from(
                        grandparent_dir
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy(),
                    );
                    return Self::from_sourced_directory(&path_string.as_str());
                }
                None => RawModuleLocation::Unknown,
            },
            None => RawModuleLocation::Unknown,
        }
    }
}
