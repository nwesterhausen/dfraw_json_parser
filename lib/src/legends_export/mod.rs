use std::path::Path;

use crate::{
    options::ParserOptions,
    parser::{ModuleInfoFile, ObjectType, RawMetadata, RawModuleLocation},
};

mod reader;
pub mod xml_creature;
pub mod xml_entity;

pub use reader::parse_legends_export as parse;

fn legends_metadata(
    filepath: &Path,
    object_type: &ObjectType,
    options: &ParserOptions,
) -> RawMetadata {
    // Create a module info file
    #[allow(clippy::unwrap_used)]
    let file_name = filepath.file_name().unwrap().to_str().unwrap();
    #[allow(clippy::unwrap_used)]
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
