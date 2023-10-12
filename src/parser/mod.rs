use std::path::Path;

use crate::{options::ParserOptions, parser::module_info_file::ModuleInfoFile};

mod biomes;
pub(crate) mod body_size;
pub(crate) mod color;
pub mod creature;
pub mod creature_caste;
pub mod creature_effect;
pub mod creature_variation;
mod graphics;
pub(crate) mod helpers;
pub mod inorganic;
pub(crate) mod material;
pub mod material_template;
pub(crate) mod milkable;
pub(crate) mod module_info_file;
mod names;
pub mod object_types;
pub mod plant;
pub mod plant_growth;
pub(crate) mod ranges;
pub mod raw_locations;
pub(crate) mod raws;
mod reader;
mod refs;
pub(crate) mod seed_material;
pub mod select_creature;
pub(crate) mod serializer_helper;
pub mod shrub;
pub mod syndrome;
pub mod temperature;
pub(crate) mod tile;
pub(crate) mod tree;

pub(crate) fn parse_info_file_from_file_path<P: AsRef<Path>>(raw_file_path: &P) -> ModuleInfoFile {
    ModuleInfoFile::parse(&raw_file_path.as_ref())
}

pub(crate) fn parse_raws_from_single_file<P: AsRef<Path>>(
    entry_path: &P,
    options: &ParserOptions,
) -> Vec<Box<dyn raws::RawObject>> {
    reader::parse_file::parse_raw_file(entry_path, options)
}
