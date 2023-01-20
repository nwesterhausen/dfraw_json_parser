use std::path::Path;

use walkdir::WalkDir;

use crate::parser::{
    raws::{
        creature::DFCreature, graphics::SpriteGraphic, info_txt::DFInfoFile,
        inorganic::DFInorganic, plant::DFPlant, RawObjectKind,
    },
    TypedJsonSerializable,
};

use super::DFParser;

impl DFParser {
    /// It takes a path to a directory containing raws, and returns a vector of objects that can be
    /// serialized to JSON
    ///
    /// Arguments:
    ///
    /// * `raw_module_directory`: The path to the directory containing the raws.
    ///
    /// Returns:
    ///
    /// A vector of Box<dyn TypedJsonSerializable>
    pub fn parse_raw_module_into_serializable<P: AsRef<Path>>(
        raw_module_directory: &P,
    ) -> Vec<Box<dyn TypedJsonSerializable>> {
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
        let dfraw_module_info = DFParser::parse_info_file(&info_txt_path);
        log::info!(
            "Parsing raws for {} v{}",
            dfraw_module_info.get_identifier(),
            dfraw_module_info.displayed_version
        );

        DFParser::parse_raws_into_serializable(raw_module_directory, &dfraw_module_info)
    }

    /// It takes a raw module directory path and a `DFInfoFile`, and returns a vector of <impl TypedJsonSerializable>
    ///
    /// Arguments:
    ///
    /// * `raw_module_directory`: The path to the raws directory of the module you want to parse.
    /// * `info_text_file`: This is a struct that contains the contents of the info.txt file.
    ///
    /// Returns:
    ///
    /// A vector of Boxes containing the raws
    pub fn parse_raws_into_serializable<P: AsRef<Path>>(
        raw_module_directory: &P,
        info_text_file: &DFInfoFile,
    ) -> Vec<Box<dyn TypedJsonSerializable>> {
        //2. Parse raws in the 'object' subdirectory
        let objects_path = raw_module_directory.as_ref().join("objects");
        if !objects_path.exists() {
            log::debug!("No objects subdirectory, no raws to parse.");
            return Vec::new();
        }
        if !objects_path.is_dir() {
            log::error!("Objects subdirectory is not valid subdirectory! Unable to parse raws.");
            return Vec::new();
        }

        // Setup empty result vector
        let mut serializable_raws: Vec<Box<dyn TypedJsonSerializable>> = Vec::new();

        // Read all the files in the directory, selectively parse the .txt files
        for entry in WalkDir::new(objects_path)
            .into_iter()
            .filter_map(std::result::Result::ok)
        {
            let f_name = entry.file_name().to_string_lossy();

            if f_name.ends_with(".txt") {
                let entry_path = entry.path();
                serializable_raws.extend(DFParser::parse_raws_from_single_file_into_serializable(
                    &entry_path,
                    info_text_file,
                ));
            }
        }

        serializable_raws
    }

    /// It takes a path to a file, and a `DFInfoFile`, and returns a vector of <impl TypedJsonSerializable>
    ///
    /// Arguments:
    ///
    /// * `entry_path`: &Path - the path to the file to be parsed
    /// * `info_text_file`: This is a struct that contains a `HashMap` of the `info_text_file.txt` file.
    ///
    /// Returns:
    ///
    /// A vector of Boxes containing a trait object.
    pub fn parse_raws_from_single_file_into_serializable<P: AsRef<Path>>(
        entry_path: &P,
        info_text_file: &DFInfoFile,
    ) -> Vec<Box<dyn TypedJsonSerializable>> {
        // Setup empty result vector
        let mut serializable_raws: Vec<Box<dyn TypedJsonSerializable>> = Vec::new();
        let caller = "parse_raws_into_serializable";

        match DFParser::read_raw_file_type(entry_path) {
            RawObjectKind::Creature => {
                log::debug!("parsing {}", entry_path.as_ref().display());
                let creature_raw_vec = DFCreature::parse_raw_file(entry_path, info_text_file);
                for creature in creature_raw_vec {
                    serializable_raws.push(Box::new(creature.clone()));
                }
            }
            RawObjectKind::Plant => {
                log::debug!("parsing {}", entry_path.as_ref().display());
                let plant_raw_vec = DFPlant::parse_raw_file(entry_path, info_text_file);
                for plant in plant_raw_vec {
                    serializable_raws.push(Box::new(plant.clone()));
                }
            }
            RawObjectKind::Inorganic => {
                log::debug!("parsing {}", entry_path.as_ref().display());
                let inorganic_raw_vec = DFInorganic::parse(entry_path, info_text_file);
                for inorganic in inorganic_raw_vec {
                    serializable_raws.push(Box::new(inorganic.clone()));
                }
            }
            _ => log::trace!("{} - skipping {}", caller, entry_path.as_ref().display()),
        }

        serializable_raws
    }

    /// It takes a path to a file, and a `DFInfoFile`, and returns a vector of <impl TypedJsonSerializable>
    ///
    /// Arguments:
    ///
    /// * `entry_path`: &Path - the path to the file to be parsed
    /// * `info_text_file`: This is a struct that contains a `HashMap` of the `info_text_file.txt` file.
    ///
    /// Returns:
    ///
    /// A vector of Boxes containing a trait object.
    pub fn parse_graphics_raws_from_single_file_into_serializable<P: AsRef<Path>>(
        entry_path: &P,
        info_text_file: &DFInfoFile,
    ) -> Vec<Box<dyn TypedJsonSerializable>> {
        let mut serializable_raws: Vec<Box<dyn TypedJsonSerializable>> = Vec::new();
        let sprite_vec = SpriteGraphic::parse(entry_path, info_text_file);
        for sprite in sprite_vec {
            serializable_raws.push(Box::new(sprite.clone()));
        }
        serializable_raws
    }
}
