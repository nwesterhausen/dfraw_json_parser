use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use encoding_rs_io::DecodeReaderBytesBuilder;

use crate::parser::{
    mod_info_file::ModuleInfoFile,
    object_types::{ObjectType, OBJECT_TOKENS},
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
    let mut current_object: &ObjectType = &ObjectType::Unknown;
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
                "OBJECT" => {
                    if !OBJECT_TOKENS.contains_key(captured_value) {
                        // We don't know what this object is, so we can't parse it.
                        // We should log this as an error.
                        log::error!(
                            "{} - Unknown object type: {}",
                            caller,
                            captured_value.to_uppercase()
                        );
                        return created_raws;
                    }
                    // We have an object token we understand (or should)
                    // So we can save it as the type for the file.
                    current_object = OBJECT_TOKENS.get(captured_value).unwrap().clone();
                }
                "CREATURE" | "SELECT_CREATURE" => {
                    if started {
                        // We've already started a raw, so we need to finish it.
                        // This is a new creature, so we need to finish the old one.
                    } else {
                        // We haven't started a creature yet, so we need to start one.
                        started = true;
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
                    } else {
                        started = true;
                    }
                }
                _ => {}
            }
        }
    }

    // Return the created raws.
    created_raws
}
