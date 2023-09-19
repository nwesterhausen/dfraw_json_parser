use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use encoding_rs_io::DecodeReaderBytesBuilder;

use crate::parser::{
    mod_info_file::ModuleInfoFile,
    raw_object_kind::RawObjectKind,
    raws::{DFRaw, RawMetadata, RawProperties},
    refs::{DF_ENCODING, RAW_TOKEN_RE},
    tags::DFTag,
};

use super::header::read_raw_file_type;

#[allow(clippy::too_many_lines)]
pub fn parse_raw_file<P: AsRef<Path>>(raw_file_path: &P) -> Vec<DFRaw> {
    let caller = "Parse Raw (Generically)";
    let mut created_raws: Vec<DFRaw> = Vec::new();

    let file = match File::open(raw_file_path) {
        Ok(f) => f,
        Err(e) => {
            log::error!("{} - Error opening raw file for parsing!\n{:?}", caller, e);
            return created_raws;
        }
    };

    let decoding_reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(*DF_ENCODING))
        .build(file);
    let reader = BufReader::new(decoding_reader);

    // Mutable Temp Vars
    let mut current_object: DFRaw = DFRaw::empty();
    let mut started = false;
    let mut gathered_tags: Vec<DFTag> = Vec::new();
    let mut castes_temp: HashMap<String, RawProperties> = HashMap::new();
    let mut current_caste: String = String::new();
    let mut raw_filename = String::new();

    // Metadata
    let mod_info_file = ModuleInfoFile::from_raw_file_path(raw_file_path);
    let raw_variant = read_raw_file_type(raw_file_path);
    let mut raw_metadata = RawMetadata::new(
        &mod_info_file,
        &raw_variant,
        &raw_filename.as_str(),
        &raw_file_path,
    );

    for (index, line) in reader.lines().enumerate() {
        if line.is_err() {
            log::error!(
                "{} - Error processing {}:{}",
                caller,
                raw_file_path.as_ref().display(),
                index
            );
            continue;
        }
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                log::error!("{} - Line-reading error\n{:?}", caller, e);
                continue;
            }
        };

        if index == 0 {
            raw_filename = String::from(&line);
            raw_metadata = RawMetadata::new(
                &mod_info_file,
                &raw_variant,
                &raw_filename.as_str(),
                &raw_file_path,
            );
            continue;
        }
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
                "OBJECT" => {}
                "CREATURE" | "SELECT_CREATURE" => {
                    if started {
                        // We've already started a raw, so we need to finish it.
                        // This is a new creature, so we need to finish the old one.

                        // Flush the gathered tags to the current object.
                        current_object.set_tags(gathered_tags);

                        // Flush the castes to the current object.
                        current_object.set_castes(castes_temp);

                        // Add the current object to the list of raws.
                        created_raws.push(current_object);
                    } else {
                        // We haven't started a creature yet, so we need to start one.
                        started = true;
                    }
                    // Creating a creature raw
                    current_object =
                        DFRaw::new(captured_value, RawObjectKind::Creature, &raw_metadata);
                    // Reset the tags
                    gathered_tags = Vec::new();
                    // Reset the castes
                    castes_temp = HashMap::new();
                    current_caste = String::new();

                    // Add an extra tag to help indicate SELECT_CREATURE
                    if captured_key == "SELECT_CREATURE" {
                        current_object
                            .add_other_property(format!("SELECT_CREATURE:{}", captured_value));
                    }
                }
                "CASTE" => {
                    // We're starting a new caste, so just update the current caste key.
                    current_caste = String::from(captured_value);
                }
                "INORGANIC" | "SELECT_INORGANIC" => {
                    if started {
                        // We've already started a raw, so we need to finish it.
                        // This is a new creature, so we need to finish the old one.

                        // Flush the gathered tags to the current object.
                        current_object.set_tags(gathered_tags);

                        // Flush the castes to the current object.
                        current_object.set_castes(castes_temp);

                        // Add the current object to the list of raws.
                        created_raws.push(current_object);
                    } else {
                        started = true;
                    }

                    // Creating an inorganic raw
                    current_object =
                        DFRaw::new(captured_value, RawObjectKind::Inorganic, &raw_metadata);

                    // Reset the tags
                    gathered_tags = Vec::new();
                    // Reset the castes
                    castes_temp = HashMap::new();

                    // Add an extra tag to help indicate SELECT_INORGANIC
                    if captured_key == "SELECT_INORGANIC" {
                        current_object
                            .add_other_property(format!("SELECT_INORGANIC:{}", captured_value));
                    }
                }
                _ => {
                    if started {
                        current_object
                            .add_other_property(format!("{}:{}", captured_key, captured_value));
                    }
                }
            }
        }
    }

    // We've reached the end of the file, so we need to finish the last object.
    // Flush the gathered tags to the current object.
    current_object.set_tags(gathered_tags);
    // Flush the castes to current object
    current_object.set_castes(castes_temp);
    // Add the current object to the list of raws.
    created_raws.push(current_object);

    // Return the created raws.
    created_raws
}
