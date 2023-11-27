use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::parser::{ObjectType, RawModuleLocation};

/// Option struct for passing to any parse function.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct ParserOptions {
    /// Whether to attach a metadata field to the raws.
    /// If true, all raws will have a `metadata` field which shows information about the
    /// raw file, its path, its module, and its parent directory.
    ///
    /// Default: false.
    pub attach_metadata_to_raws: bool,
    /// Whether to skip the "copy tags from" resolution step.
    /// If true, the creature will have a populated `copy_tags_from` field instead.
    ///
    /// Default: false.
    pub skip_apply_copy_tags_from: bool,
    /// Whether to skip the apply "creature variations" resolution step.
    /// When this is true, it will just leave the variations attached to the creature
    /// in a `creature_variations` field.
    /// If false, it will modify the creature data to include the variations.
    ///
    /// Note: This is currently not implemented.
    ///
    /// Default: false.
    pub skip_apply_creature_variations: bool,
    /// What kind of raws to parse. If this is left empty, all raws will be parsed.
    ///
    /// Default: [ Creature, Plant, Inorganic, MaterialTemplate, Graphics, TilePage ]
    pub raws_to_parse: Vec<ObjectType>,
    /// What locations to parse raws from. If this is left empty, no locations will be parsed.
    ///
    /// (i.e. only the specific raw files, modules, or legends_plus exports specified will be parsed)
    ///
    /// Default: [Vanilla]
    pub locations_to_parse: Vec<RawModuleLocation>,
    /// The path to the dwarf fortress directory. If no locations are specified, then this is not used.
    ///
    /// If specific raw files, modules or legends_plus exports are specified, then this is not used when
    /// parsing those.
    pub dwarf_fortress_directory: PathBuf,
    /// Whether to serialize the result to json. If true, the result will be serialized to json before
    /// being returned.
    ///
    /// (This means the result will be a `Vec` of `String` instead of a `Vec` of `Box<dyn RawObject>`.)
    ///
    /// Default: false
    pub serialize_result_to_json: bool,
    /// Optionally specify one or more legends_plus exports to parse in addition to the raws.
    /// These exports include information about generated creatures which are not included in the
    /// raws.
    ///
    /// Default: None
    pub legends_exports_to_parse: Vec<PathBuf>,
    /// Optionally specify one or more raw files to parse directly. These should be the raw files
    /// themselves, not the containing directory.
    ///
    /// (e.g. `creature_standard.txt` in `data/vanilla/vanilla_creatures/objects/`)
    ///
    /// Note that these will be parsed in addition to the raws in the specified locations in the other
    /// options. That means that if you specify a raw file that is also in the vanilla raws, it will
    /// be parsed twice (if vanilla is in the locations to parse).
    ///
    /// Default: None
    pub raw_files_to_parse: Vec<PathBuf>,
    /// Optionally specify one or more raw modules to parse directly. These should be the module
    /// directories, not the info.txt file.
    ///
    /// (e.g. `vanilla_creatures` in `data/vanilla/`)
    ///
    /// Note that these will be parsed in addition to the raws in the specified locations in the other
    /// options. That means that if you specify a module that is also in the vanilla raws, it will
    /// be parsed twice (if vanilla is in the locations to parse).
    ///
    /// Default: None
    pub raw_modules_to_parse: Vec<PathBuf>,
    /// Optionally specify one or more module info files to parse directly. These should be the info.txt
    /// files themselves, not the containing directory.
    ///
    /// (e.g. `info.txt` in `data/vanilla/vanilla_creatures/`)
    ///
    /// Note that if you are calling the `parse` function, this will be ignored. This is only used
    /// when calling the `parse_module_info_files` function.
    pub module_info_files_to_parse: Vec<PathBuf>,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self {
            attach_metadata_to_raws: false,
            skip_apply_copy_tags_from: false,
            skip_apply_creature_variations: false,
            serialize_result_to_json: false,
            raws_to_parse: vec![
                ObjectType::Creature,
                ObjectType::Plant,
                ObjectType::Inorganic,
                ObjectType::MaterialTemplate,
                ObjectType::Graphics,
                ObjectType::TilePage,
            ],
            locations_to_parse: vec![RawModuleLocation::Vanilla],
            dwarf_fortress_directory: PathBuf::from(""),
            legends_exports_to_parse: Vec::new(),
            raw_files_to_parse: Vec::new(),
            raw_modules_to_parse: Vec::new(),
            module_info_files_to_parse: Vec::new(),
        }
    }
}

impl ParserOptions {
    /// Creates a new `ParserOptions` struct with the default values.
    /// * `target_path` is the path to parse in.
    ///
    /// For `ParsingJob::ALL` or `ParsingJob::SingleLocation`, this should be the path to the dwarf fortress directory.
    ///
    /// For `ParsingJob::SingleModule`, this should be the path to the module (which includes the
    /// info.txt file).
    ///
    /// For `ParsingJob::SingleRaw`, this should be the path directly to the raw.
    pub fn new<P: AsRef<Path>>(target_path: P) -> Self {
        Self {
            dwarf_fortress_directory: target_path.as_ref().to_path_buf(),
            ..Default::default()
        }
    }

    /// If applied, all raws will have a `metadata` field which shows information about the
    /// raw file, its path, its module, and its parent directory.
    pub fn attach_metadata_to_raws(&mut self) {
        self.attach_metadata_to_raws = true;
    }

    /// Skip the "copy tags from" resolution step.
    ///
    /// Default: true.
    pub fn skip_apply_copy_tags_from(&mut self) {
        self.skip_apply_copy_tags_from = true;
    }

    /// Skip the apply "creature variations" resolution step.
    ///
    /// Note: This is currently not implemented.
    pub fn skip_apply_creature_variations(&mut self) {
        self.skip_apply_creature_variations = true;
    }

    /// Sets what kind of raws to parse.
    /// The default value will parse all the raws that are currently supported:
    /// * `ObjectType::Creature`
    /// * `ObjectType::Plant`
    /// * `ObjectType::Inorganic`
    /// * `ObjectType::MaterialTemplate`
    pub fn set_raws_to_parse(&mut self, raws_to_parse: Vec<ObjectType>) {
        self.raws_to_parse = raws_to_parse;
    }

    /// Sets what locations to parse raws from.
    /// * `RawModuleLocation::Vanilla` will parse the vanilla raws.
    /// * `RawModuleLocation::InstalledMods` will parse the installed mods folder.
    /// * `RawModuleLocation::Mods` will parse the downloaded mods folder.
    ///
    /// If left unset, only `RawModuleLocation::Vanilla` will be parsed.
    pub fn set_locations_to_parse(&mut self, locations_to_parse: Vec<RawModuleLocation>) {
        self.locations_to_parse = locations_to_parse;
    }

    /// Whether to serialize the result to json. If true, the result will be serialized to json before
    ///
    /// (This means the result will be a `Vec` of `String` instead of a `Vec` of `Box<dyn RawObject>`.)
    ///
    /// Default: false
    pub fn serialize_result_to_json(&mut self) {
        self.serialize_result_to_json = true;
    }

    /// Optionally specify one or more `legends_plus` exports to parse in addition to the raws.
    ///
    /// These exports include information about generated creatures which are not included in the
    /// raws.
    ///
    /// Default: None
    pub fn add_legends_export_to_parse<P: AsRef<Path>>(&mut self, legends_export_to_parse: &P) {
        self.legends_exports_to_parse
            .push(legends_export_to_parse.as_ref().to_path_buf());
    }

    /// Optionally specify one or more raw files to parse directly. These should be the raw files
    ///
    /// (e.g. `creature_standard.txt` in `data/vanilla/vanilla_creatures/objects/`)
    ///
    /// Note that these will be parsed in addition to the raws in the specified locations in the other
    /// options. That means that if you specify a raw file that is also in the vanilla raws, it will
    /// be parsed twice (if vanilla is in the locations to parse).
    ///
    /// Default: None
    pub fn add_raw_file_to_parse<P: AsRef<Path>>(&mut self, raw_file_to_parse: &P) {
        self.raw_files_to_parse
            .push(raw_file_to_parse.as_ref().to_path_buf());
    }

    /// Optionally specify one or more raw modules to parse directly. These should be the module
    /// directories, not the info.txt file.
    ///     
    /// (e.g. `vanilla_creatures` in `data/vanilla/`)
    ///
    /// Note that these will be parsed in addition to the raws in the specified locations in the other
    /// options. That means that if you specify a module that is also in the vanilla raws, it will
    /// be parsed twice (if vanilla is in the locations to parse).
    ///
    /// Default: None
    pub fn add_raw_module_to_parse<P: AsRef<Path>>(&mut self, raw_module_to_parse: &P) {
        self.raw_modules_to_parse
            .push(raw_module_to_parse.as_ref().to_path_buf());
    }

    /// Optionally specify one or more module info files to parse directly. These should be the info.txt
    /// files themselves, not the containing directory.
    ///
    /// (e.g. `info.txt` in `data/vanilla/vanilla_creatures/`)
    ///
    /// Note that if you are calling the `parse` function, this will be ignored. This is only used
    /// when calling the `parse_module_info_files` function.
    ///
    /// Default: None
    pub fn add_module_info_file_to_parse<P: AsRef<Path>>(&mut self, module_info_file_to_parse: &P) {
        self.module_info_files_to_parse
            .push(module_info_file_to_parse.as_ref().to_path_buf());
    }
}
