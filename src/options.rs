use serde::{Serialize, Deserialize};

use crate::parser::{object_types::ObjectType, raw_locations::RawModuleLocation};

/// Option struct for passing to any parse function.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(ts_rs::TS)]
#[ts(export)]
pub struct ParserOptions {
    /// Whether to hide metadata in the result.
    /// If false, all raws will have a `metadata` field which shows information about the
    /// raw file, its path, its module, and its parent directory.
    /// Default: true.
    pub hide_metadata_in_result: bool,
    /// Whether to apply "copy tags from" tags. 
    /// If false, the creature will have a populated `copy_tags_from` field instead.
    /// Default: true.
    pub apply_copy_tags_from: bool,
    /// Whether to apply "creature variations" tags.
    /// When this is false, it will just leave the variations attached to the creature
    /// in a `creature_variations` field.
    /// Default: false.
    pub apply_creature_variations: bool,
    /// What kind of raws to parse. If this is left empty, all raws will be parsed.
    /// Default: [ Creature, Plant ]
    pub raws_to_parse: Vec<ObjectType>,
    /// What locations to parse raws from. If this is left empty, all raws will be parsed.
    /// When parsing a single file, this is ignored.
    /// Default: empty.
    pub locations_to_parse: Vec<RawModuleLocation>,
}

impl Default for ParserOptions {
    fn default() -> Self {
        Self {
            hide_metadata_in_result: true,
            apply_copy_tags_from: true,
            apply_creature_variations: false,
            raws_to_parse: vec![ObjectType::Creature, ObjectType::Plant],
            locations_to_parse: Vec::new(),
        }
    }
}