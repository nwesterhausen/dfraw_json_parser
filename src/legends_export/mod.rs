use std::path::Path;

use crate::parser::{
    module_info_file::ModuleInfoFile, object_types::ObjectType, raw_locations::RawModuleLocation,
    raws::RawMetadata,
};

pub mod reader;
mod xml_creature;
mod xml_entity;

fn legends_metadata(filepath: &Path, object_type: &ObjectType) -> RawMetadata {
    // Create a module info file
    let file_name = filepath.file_name().unwrap().to_str().unwrap();
    let parent_dir = filepath.parent().unwrap().to_str().unwrap();
    let mut module_info_file =
        ModuleInfoFile::new(file_name, RawModuleLocation::LegendsExport, parent_dir);
    module_info_file.set_module_name("Legends Export");
    RawMetadata::new(&module_info_file, object_type, file_name, &filepath, false)
}
