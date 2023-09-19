use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use encoding_rs_io::DecodeReaderBytesBuilder;
use serde::{Deserialize, Serialize};

use crate::{parser::refs::NON_DIGIT_RE, util::get_parent_dir_name};

use super::{
    raw_locations::RawModuleLocation,
    refs::{DF_ENCODING, RAW_TOKEN_RE},
};

// Struct for info about a raw module
#[derive(Serialize, Deserialize)]
pub struct ModuleInfoFile {
    identifier: String,
    location: RawModuleLocation,
    parent_directory: String,
    pub numeric_version: u32,
    pub displayed_version: String,
    pub earliest_compatible_numeric_version: u32,
    pub earliest_compatible_displayed_version: String,
    pub author: String,
    pub name: String,
    pub description: String,
}

impl ModuleInfoFile {
    pub fn empty() -> Self {
        Self {
            identifier: String::new(),
            location: RawModuleLocation::Unknown,
            parent_directory: String::new(),
            numeric_version: 0,
            displayed_version: String::new(),
            earliest_compatible_numeric_version: 0,
            earliest_compatible_displayed_version: String::new(),
            author: String::new(),
            name: String::new(),
            description: String::new(),
        }
    }
    pub fn new(id: &str, location: RawModuleLocation, parent_directory: &str) -> Self {
        Self {
            identifier: id.to_string(),
            location,
            parent_directory: parent_directory.to_owned(),
            numeric_version: 0,
            displayed_version: "0".to_string(),
            earliest_compatible_numeric_version: 0,
            earliest_compatible_displayed_version: "0".to_string(),
            author: String::new(),
            name: String::new(),
            description: String::new(),
        }
    }
    pub fn from_raw_file_path<P: AsRef<Path>>(full_path: &P) -> Self {
        // Take the full path for the raw file and navigate up to the parent directory
        // e.g from `data/vanilla/vanilla_creatures/objects/creature_standard.txt` to `data/vanilla/vanilla_creatures`
        // Then run parse on `data/vanilla/vanilla_creatures/info.txt`
        let parent_directory = full_path
            .as_ref()
            .parent()
            .unwrap_or(Path::new(""))
            .parent()
            .unwrap_or(Path::new(""))
            .to_string_lossy()
            .to_string();
        let info_file_path = Path::new(parent_directory.as_str()).join("info.txt");
        log::info!("ModuleInfoFile - Parsing info.txt for {:?}", info_file_path);
        Self::parse(&info_file_path)
    }
    #[allow(clippy::too_many_lines)]
    pub fn parse<P: AsRef<Path>>(info_file_path: &P) -> ModuleInfoFile {
        let parent_dir = get_parent_dir_name(info_file_path);
        let location = RawModuleLocation::from_info_text_file_path(info_file_path);

        let file = match File::open(info_file_path) {
            Ok(f) => f,
            Err(e) => {
                log::error!(
                    "DFInfoFile - Error opening raw file for parsing in \"{}\"\n{:?}",
                    parent_dir,
                    e
                );
                return ModuleInfoFile::empty();
            }
        };

        let decoding_reader = DecodeReaderBytesBuilder::new()
            .encoding(Some(*DF_ENCODING))
            .build(file);
        let reader = BufReader::new(decoding_reader);

        // info.txt details
        let mut caller = String::from("DFInfoFile");
        let mut info_file_data: ModuleInfoFile = ModuleInfoFile::new("", location, &parent_dir);

        for (index, line) in reader.lines().enumerate() {
            if line.is_err() {
                log::error!("{} - Error processing {:?}:{}", caller, parent_dir, index);
                continue;
            }
            let line = match line {
                Ok(l) => l,
                Err(e) => {
                    log::error!("{} - Line-reading error\n{:?}", caller, e);
                    continue;
                }
            };
            for cap in RAW_TOKEN_RE.captures_iter(&line) {
                let captured_key = match cap.get(2) {
                    Some(v) => v.as_str(),
                    _ => {
                        continue;
                    }
                };
                let captured_value = match cap.get(3) {
                    Some(v) => v.as_str(),
                    _ => {
                        continue;
                    }
                };

                log::trace!(
                    "{} - Key: {} Value: {}",
                    caller,
                    captured_key,
                    captured_value
                );

                match captured_key {
                    // SECTION FOR MATCHING info.txt DATA
                    "ID" => {
                        // the [ID:identifier] tag should be the top of the info.txt file
                        info_file_data = ModuleInfoFile::new(captured_value, location, &parent_dir);
                        caller = format!("DFInfoFile ({})", &captured_value);
                    }
                    "NUMERIC_VERSION" => match captured_value.parse() {
                        Ok(n) => info_file_data.numeric_version = n,
                        Err(_e) => {
                            log::warn!(
                                "{} - 'NUMERIC_VERSION' should be integer {}",
                                caller,
                                parent_dir
                            );
                            // match on \D to replace any non-digit characters with empty string
                            let digits_only =
                                NON_DIGIT_RE.replace_all(captured_value, "").to_string();
                            match digits_only.parse() {
                                Ok(n) => info_file_data.numeric_version = n,
                                Err(_e) => {
                                    log::error!(
                                        "{} - Unable to parse any numbers from {}",
                                        caller,
                                        digits_only
                                    );
                                }
                            }
                        }
                    },
                    "EARLIEST_COMPATIBLE_NUMERIC_VERSION" => match captured_value.parse() {
                        Ok(n) => info_file_data.earliest_compatible_numeric_version = n,
                        Err(_e) => {
                            log::warn!(
                                "{} - 'EARLIEST_COMPATIBLE_NUMERIC_VERSION' should be integer {}",
                                caller,
                                parent_dir
                            );
                            // match on \D to replace any non-digit characters with empty string
                            let digits_only =
                                NON_DIGIT_RE.replace_all(captured_value, "").to_string();
                            match digits_only.parse() {
                                Ok(n) => info_file_data.earliest_compatible_numeric_version = n,
                                Err(_e) => {
                                    log::error!(
                                        "{} - Unable to parse any numbers from {}",
                                        caller,
                                        digits_only
                                    );
                                }
                            }
                        }
                    },
                    "DISPLAYED_VERSION" => {
                        info_file_data.displayed_version = String::from(captured_value);
                        caller = format!(
                            "DFInfoFile ({}@v{})",
                            info_file_data.get_identifier(),
                            &captured_value
                        );
                    }
                    "EARLIEST_COMPATIBLE_DISPLAYED_VERSION" => {
                        info_file_data.earliest_compatible_displayed_version =
                            String::from(captured_value);
                    }
                    "AUTHOR" => {
                        info_file_data.author = String::from(captured_value);
                    }
                    "NAME" => {
                        info_file_data.name = String::from(captured_value);
                    }
                    "DESCRIPTION" => {
                        info_file_data.description = String::from(captured_value);
                    }
                    &_ => (),
                }
            }
        }

        // Do some final checks to confirm that the name is set. Specifically in "Dark Ages V - War & Mythos" the
        // [name] Token in the info.txt is written incorrectly as "[name]X" instead of [name:X]
        if info_file_data.name.is_empty() || info_file_data.name.is_empty() {
            info_file_data.name = info_file_data.get_identifier();
        }

        // Check for 'unknown' identifier and try to provide any extra info
        if info_file_data.get_identifier() == "unknown" {
            log::error!(
                "Failure parsing proper info from {}",
                info_file_path.as_ref().display()
            );
        }

        info_file_data
    }

    pub fn get_identifier(&self) -> String {
        String::from(&self.identifier)
    }
    pub fn get_location(&self) -> RawModuleLocation {
        self.location
    }
    pub fn get_description(&self) -> String {
        String::from(&self.description)
    }
    pub fn get_name(&self) -> String {
        String::from(&self.name)
    }
    pub fn get_version(&self) -> String {
        String::from(&self.displayed_version)
    }
    pub fn get_parent_directory(&self) -> String {
        String::from(&self.parent_directory)
    }
}
