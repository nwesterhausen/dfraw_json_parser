use std::path::Path;

use crate::{options::ParserOptions, parser::module_info_file::ModuleInfoFile};

pub mod biomes;
pub mod body_size;
pub mod color;
pub mod creature;
pub mod creature_caste;
pub mod creature_effect;
pub mod creature_variation;
pub mod entity;
pub mod graphics;
pub mod helpers;
pub mod inorganic;
pub mod material;
pub mod material_mechanics;
pub mod material_template;
pub mod milkable;
pub mod module_info_file;
pub mod names;
pub mod object_types;
pub mod plant;
pub mod plant_growth;
pub mod ranges;
pub mod raw_locations;
pub mod raws;
mod reader;
mod refs;
pub mod searchable;
pub mod seed_material;
pub mod select_creature;
pub mod serializer_helper;
pub mod shrub;
pub mod syndrome;
pub mod temperature;
pub mod tile;
pub mod tree;

pub fn parse_info_file_from_file_path<P: AsRef<Path>>(raw_file_path: &P) -> ModuleInfoFile {
    ModuleInfoFile::parse(&raw_file_path.as_ref())
}

pub fn parse_raws_from_single_file<P: AsRef<Path>>(
    entry_path: &P,
    options: &ParserOptions,
) -> Vec<Box<dyn raws::RawObject>> {
    reader::parse_file::parse_raw_file(entry_path, options)
}
