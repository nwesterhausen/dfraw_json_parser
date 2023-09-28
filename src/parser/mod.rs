use std::path::Path;

use walkdir::WalkDir;

use crate::{
    options::ParserOptions,
    parser::{
        module_info_file::ModuleInfoFile, raws::RawObject,
        reader::parse_file::parse_raw_file_with_info,
    },
};

mod biomes;
pub(crate) mod body_size;
pub(crate) mod color;
pub mod creature;
pub mod creature_caste;
pub mod creature_effect;
pub mod creature_variation;
mod graphics;
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
pub(crate) mod serializer_helper;
pub mod shrub;
pub mod syndrome;
pub mod termperatures;
pub(crate) mod tile;
pub(crate) mod tree;

pub(crate) fn parse_info_file_from_module_directory<P: AsRef<Path>>(
    raw_module_directory: &P,
) -> ModuleInfoFile {
    ModuleInfoFile::parse(&raw_module_directory.as_ref().join("info.txt"))
}

pub(crate) fn parse_info_file_from_file_path<P: AsRef<Path>>(raw_file_path: &P) -> ModuleInfoFile {
    ModuleInfoFile::parse(&raw_file_path.as_ref())
}

pub(crate) fn parse_raws_from_single_file<P: AsRef<Path>>(
    entry_path: &P,
    options: &ParserOptions,
) -> Vec<Box<dyn raws::RawObject>> {
    reader::parse_file::parse_raw_file(entry_path, options)
}

pub(crate) fn parse_raw_module<P: AsRef<Path>>(
    raw_module_directory: &P,
    options: &ParserOptions,
) -> Vec<Box<dyn raws::RawObject>> {
    //1. Get information from the info.txt file
    if !raw_module_directory.as_ref().exists() {
        log::error!(
            "Provided directory to parse raws does not exist: {:?}",
            raw_module_directory.as_ref().to_string_lossy()
        );
        return Vec::new();
    }
    if !raw_module_directory.as_ref().is_dir() {
        log::error!(
            "Provided 'directory' to parse is not actually a directory! {:?}",
            raw_module_directory.as_ref().to_string_lossy()
        );
        return Vec::new();
    }

    // Check for info.txt
    let info_txt_path = raw_module_directory.as_ref().join("info.txt");
    if !info_txt_path.exists() {
        let dir_name = raw_module_directory
            .as_ref()
            .file_name()
            .unwrap_or_default();
        let dir_name_str = dir_name.to_str().unwrap_or("");

        if !(dir_name_str.eq("mod_upload")
            || dir_name_str.eq("examples and notes")
            || dir_name_str.eq("interaction examples"))
        {
            log::error!(
                "No info.txt as expected in {:?}. Is this DF 50.xx?",
                raw_module_directory
                    .as_ref()
                    .file_name()
                    .unwrap_or_default()
            );
        }

        return Vec::new();
    }

    // Parse info.txt to get raw module information
    let info_text_file = ModuleInfoFile::parse(&info_txt_path);
    log::info!(
        "Parsing raws for {} v{}",
        info_text_file.get_identifier(),
        info_text_file.get_version(),
    );

    //2. Parse raws in the 'object' subdirectory
    let objects_path = raw_module_directory.as_ref().join("objects");
    if !objects_path.exists() {
        log::debug!("No objects subdirectory, no raws to parse.");
    }
    if !objects_path.is_dir() {
        log::error!("Objects subdirectory is not valid subdirectory! Unable to parse raws.");
    }
    //2. Parse raws in the 'graphics' subdirectory
    let graphics_path = raw_module_directory.as_ref().join("graphics");
    if !graphics_path.exists() {
        log::debug!("No graphics subdirectory, no raws to parse.");
    }
    if !graphics_path.is_dir() {
        log::error!("Graphics subdirectory is not valid subdirectory! Unable to parse raws.");
    }

    if !objects_path.exists() && !graphics_path.exists() {
        return Vec::new();
    }

    // Setup empty result vector
    let mut serializable_raws: Vec<Box<dyn RawObject>> = Vec::new();

    // Read all the files in the directory, selectively parse the .txt files
    for entry in WalkDir::new(objects_path)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.ends_with(".txt") {
            let entry_path = entry.path();
            serializable_raws.extend(parse_raw_file_with_info(
                &entry_path,
                &info_text_file,
                options,
            ));
        }
    }
    // Read all the files in the directory, selectively parse the .txt files
    for entry in WalkDir::new(graphics_path)
        .into_iter()
        .filter_map(std::result::Result::ok)
    {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.ends_with(".txt") {
            let entry_path = entry.path();
            serializable_raws.extend(parse_raw_file_with_info(
                &entry_path,
                &info_text_file,
                options,
            ));
        }
    }

    serializable_raws
}
