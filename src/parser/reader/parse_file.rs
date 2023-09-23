use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use encoding_rs_io::DecodeReaderBytesBuilder;

use crate::parser::{
    creature::raw::DFCreature,
    mod_info_file::ModuleInfoFile,
    object_types::{ObjectType, OBJECT_TOKENS},
    raws::{RawMetadata, RawObject},
    refs::{DF_ENCODING, RAW_TOKEN_RE},
};

use super::header::read_raw_file_type;

pub fn parse_raw_file<P: AsRef<Path>>(raw_file_path: &P) -> Vec<Box<dyn RawObject>> {
    let mod_info_file = ModuleInfoFile::from_raw_file_path(raw_file_path);

    parse_raw_file_with_info(raw_file_path, &mod_info_file)
}

#[allow(clippy::too_many_lines)]
pub fn parse_raw_file_with_info<P: AsRef<Path>>(
    raw_file_path: &P,
    mod_info_file: &ModuleInfoFile,
) -> Vec<Box<dyn RawObject>> {
    let caller = "Parse Raw (Generically)";
    let mut created_raws: Vec<Box<dyn RawObject>> = Vec::new();

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
    let mut started = false;
    let mut raw_filename = String::new();
    let mut temp_creature = DFCreature::empty();

    // Metadata
    let object_type = read_raw_file_type(raw_file_path);
    let mut raw_metadata = RawMetadata::new(
        mod_info_file,
        &object_type,
        raw_filename.as_str(),
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
                mod_info_file,
                &object_type,
                raw_filename.as_str(),
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
                            "{} - Unknown object type: {} Raw: {}",
                            caller,
                            captured_value.to_uppercase(),
                            raw_filename
                        );
                        return created_raws;
                    }
                    // Check of object_type matches the captured_value as ObjectType.
                    // If it doesn't, we should log this as an error.
                    if &object_type != OBJECT_TOKENS.get(captured_value).unwrap() {
                        log::error!(
                            "{} - Object type mismatch: {} != {}",
                            caller,
                            object_type,
                            captured_value.to_uppercase()
                        );
                        return created_raws;
                    }
                }
                "CREATURE" | "SELECT_CREATURE" => {
                    if started {
                        // We need to add the creature to the list.
                        created_raws.push(Box::new(temp_creature.clone()));
                    }
                    // We haven't started a creature yet, so we need to start one.
                    started = true;
                    temp_creature = DFCreature::new(captured_value, raw_metadata.clone());
                }
                "CASTE" => {
                    // Starting a new caste (in creature), so we can just add a caste to the last creature we started.
                    if started {
                        // We have a creature, so we can add a caste to it.
                        // First we have to cast the dyn RawObject to a DFCreature.
                        temp_creature.add_caste(captured_value);
                    }
                }
                "SELECT_CASTE" => {
                    // Starting a new caste (in creature), so we can just add a caste to the last creature we started.
                    if started {
                        // We have a creature, so we can add a caste to it.
                        // First we have to cast the dyn RawObject to a DFCreature.
                        temp_creature.select_caste(captured_value);
                    }
                }
                "INORGANIC" | "SELECT_INORGANIC" => {
                    if started {
                        // We've already started a raw, so we need to finish it.
                        // This is a new creature, so we need to finish the old one.
                    } else {
                        started = true;
                    }
                }
                _ => {
                    // This should be a tag for the current object.
                    // We should check if we have a current object, and if we do, we should add the tag to it.
                    // If we haven't started yet, we should do nothing.
                    if started {
                        match object_type {
                            ObjectType::Creature => {
                                // We have a creature, so we can add a tag to it.
                                // First we have to cast the dyn RawObject to a DFCreature.
                                temp_creature.parse_tag(captured_key, captured_value);
                            }
                            ObjectType::Inorganic => {
                                log::info!("Pretend to parse inorganics....");
                            }
                            _ => {
                                // We don't have a known raw yet. So do nothing.
                            }
                        }
                    }
                }
            }
        }
    }
    if started {
        // If we did indeed start capture, we need to complete the final raw by adding it to the list
        match object_type {
            ObjectType::Creature => created_raws.push(Box::new(temp_creature.clone())),
            ObjectType::Inorganic => log::info!("Pretend to parse inorganics...."),
            _ => {}
        }
    }

    // Return the created raws.
    created_raws
}
