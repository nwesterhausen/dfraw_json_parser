use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::parser::{object_types::ObjectType, raw_locations::RawModuleLocation};

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
    /// Default: [ Creature, Plant, Inorganic, MaterialTemplate ]
    pub raws_to_parse: Vec<ObjectType>,
    /// What locations to parse raws from. If this is left empty, all locations will be parsed.
    /// When parsing a single file, this is ignored. If the job is to parse a single location,
    /// only the first location in this list will be used.
    /// Default: Vanilla.
    pub locations_to_parse: Vec<RawModuleLocation>,
    /// The path to the dwarf fortress directory if parsing ALL or a SingleLocation. If
    /// parsing a single module, this should be the path to the module (which includes the
    /// info.txt file). If parsing a single raw file, this should be the path directly to the raw.
    pub target_path: PathBuf,
    /// The job to perform.
    /// Default: All
    pub job: ParsingJob,
    /// Whether to serialize the result to json. If true, the result will be serialized to json before
    /// being returned.
    ///
    /// (This means the result will be a `Vec` of `String` instead of a `Vec` of `Box<dyn RawObject>`.)
    ///
    /// Default: false
    pub serialize_result_to_json: bool,
    /// The path to write the json output to. This is only used if `serialize_result_to_json` is true.
    ///
    /// If left empty, ./output.json will be used.
    pub output_path: PathBuf,
    /// Whether output to a file or not
    /// Default: false
    pub output_to_file: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ts_rs::TS)]
#[ts(export)]
pub enum ParsingJob {
    SingleRaw,
    SingleModule,
    AllModulesInLocations,
    SingleModuleInfoFile,
    AllModuleInfoFiles,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self {
            attach_metadata_to_raws: false,
            skip_apply_copy_tags_from: false,
            skip_apply_creature_variations: false,
            serialize_result_to_json: false,
            output_to_file: false,
            raws_to_parse: vec![
                ObjectType::Creature,
                ObjectType::Plant,
                ObjectType::Inorganic,
                ObjectType::MaterialTemplate,
                ObjectType::Graphics,
                ObjectType::TilePage,
            ],
            locations_to_parse: vec![RawModuleLocation::Vanilla],
            target_path: PathBuf::from(""),
            job: ParsingJob::AllModulesInLocations,
            output_path: PathBuf::from(""),
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
            target_path: target_path.as_ref().to_path_buf(),
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

    /// Sets the job to perform.
    /// * `ParsingJob::SingleRaw` will parse a single raw file. (e.g. `creature_standard.txt` in `data/vanilla/vanilla_creatures/objects/`)
    /// * `ParsingJob::SingleModule` will parse a single module (e.g. `vanilla_creatures` in `data/vanilla/`)
    /// * `ParsingJob::SingleLocation` will parse a single location (e.g. `data/vanilla`)
    /// * `ParsingJob::All` will parse all raws in all locations (i.e. all locations in `locations_to_parse`)
    /// * `ParsingJob::SingleModuleInfoFile` will parse the info.txt file at the provided path.
    /// * `ParsingJob::AllModuleInfoFiles` will parse all info.txt files in all locations (i.e. all locations in `locations_to_parse`)
    ///
    /// Default: All
    pub fn set_job(&mut self, job: ParsingJob) {
        self.job = job;
    }

    /// Whether to serialize the result to json. If true, the result will be serialized to json before
    ///
    /// (This means the result will be a `Vec` of `String` instead of a `Vec` of `Box<dyn RawObject>`.)
    ///
    /// Default: false
    pub fn serialize_result_to_json(&mut self) {
        self.serialize_result_to_json = true;
    }

    /// Sets the path to write the json output to. This is only used if `serialize_result_to_json` is true.
    ///
    /// If left empty, ./output.json will be used.
    pub fn set_output_path<P: AsRef<Path>>(&mut self, output_path: P) {
        self.output_path = output_path.as_ref().to_path_buf();
    }

    /// Whether output to a file or not
    /// Default: false
    pub fn output_to_file(&mut self) {
        self.output_to_file = true;
    }
}
