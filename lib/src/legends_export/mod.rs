use std::path::Path;

use crate::{
    options::ParserOptions,
    parser::{
        module_info_file::ModuleInfoFile, object_types::ObjectType,
        raw_locations::RawModuleLocation, raws::RawMetadata,
    },
};

mod reader;
mod xml_creature;
mod xml_entity;

#[allow(clippy::module_name_repetitions)]
pub use reader::parse_legends_export;

fn legends_metadata(
    filepath: &Path,
    object_type: &ObjectType,
    options: &ParserOptions,
) -> RawMetadata {
    // Create a module info file
    let file_name = filepath.file_name().unwrap().to_str().unwrap();
    let parent_dir = filepath.parent().unwrap().to_str().unwrap();
    let mut module_info_file =
        ModuleInfoFile::new(file_name, RawModuleLocation::LegendsExport, parent_dir);
    module_info_file.set_module_name("Legends Export");
    RawMetadata::new(
        &module_info_file,
        object_type,
        file_name,
        &filepath,
        options.attach_metadata_to_raws,
    )
}
