use crate::{
    constants::DF_ENCODING,
    creature_variation::CreatureVariation,
    entity::Entity,
    graphic::Graphic,
    inorganic::Inorganic,
    material_template::MaterialTemplate,
    metadata::{ObjectType, ParserOptions, RawMetadata, RawModuleLocation, OBJECT_TOKEN_MAP},
    plant::Plant,
    raw_definitions::GRAPHIC_TYPE_TOKENS,
    reader::{unprocessed_raw::UnprocessedRaw, PARSABLE_OBJECT_TYPES},
    regex::RAW_TOKEN_RE,
    tags::{GraphicTypeTag, ModificationTag},
    tile_page::TilePage,
    traits::RawObject,
    utilities::try_get_file,
    InfoFile, ParserError,
};
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::{
    io::{BufRead, BufReader},
    path::Path,
};
use tracing::{debug, error, trace, warn};

use super::{parse_result::FileParseResult, read_raw_file_type};

/// Parse a raw file into a list of parsed raws and a list of unprocessed raws.
///
/// This function performs the following steps:
///
/// 1. Parse the raw file into a list of parsed raws.
/// 2. Filter the parsed raws into a list of unprocessed raws.
/// 3. Return the parsed raws and unprocessed raws.
///
/// The unprocessed raws need to be resolved so that they can become parsed raws. This is done
/// by calling `resolve` on an `UnprocessedRaw` object. That requires the entirety of the parsed
/// raws to be passed in, so that it can find the raws it needs to resolve against.
///
/// # Arguments
///
/// * `raw_file_path` - The path to the raw file to parse.
/// * `options` - The parser options to use when parsing the raw file.
///
/// # Returns
///
/// * `Result<FileParseResult, ParserError>` - The results of parsing the raw file.
///
/// # Errors
///
/// * `ParserError::InvalidRawFile` - If the raw file is invalid.
/// * `ParserError::IOError` - If there is an error reading the raw file.
/// * `ParserError::InfoFileError` - If there is an error reading the module info file.
#[allow(dead_code)]
pub fn parse_raw_file<P: AsRef<Path>>(
    raw_file_path: &P,
    options: &ParserOptions,
) -> Result<FileParseResult, ParserError> {
    let mod_info_file = match InfoFile::from_raw_file_path(raw_file_path) {
        Ok(m) => m,
        Err(e) => {
            warn!("parse_raw_file: Using an empty InfoFile because of error parsing the file");
            debug!("{e:?}");
            InfoFile::new(
                raw_file_path
                    .as_ref()
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or(""),
                RawModuleLocation::Unknown,
                "none",
            )
        }
    };

    parse_raw_file_with_info(raw_file_path, &mod_info_file, options)
}

/// Parse a raw file into a list of parsed raws and a list of unprocessed raws.
///
/// # Arguments
///
/// * `raw_file_path` - The path to the raw file to parse.
/// * `mod_info_file` - The module info file for the raw file.
/// * `options` - The parser options to use when parsing the raw file.
///
/// # Returns
///
/// * `Result<FileParseResult, ParserError>` - The results of parsing the raw file.
///
/// # Errors
///
/// * `ParserError::InvalidRawFile` - If the raw file is invalid.
/// * `ParserError::IOError` - If there is an error reading the raw file.
/// * `ParserError::InfoFileError` - If there is an error reading the module info file.
#[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
#[allow(dead_code)]
pub fn parse_raw_file_with_info<P: AsRef<Path>>(
    raw_file_path: &P,
    mod_info_file: &InfoFile,
    options: &ParserOptions,
) -> Result<FileParseResult, ParserError> {
    let mut created_raws: Vec<Box<dyn RawObject>> = Vec::new();
    let mut unprocessed_raws: Vec<UnprocessedRaw> = Vec::new();

    let file = try_get_file(raw_file_path)?;

    let decoding_reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(*DF_ENCODING))
        .build(file);
    let reader = BufReader::new(decoding_reader);
    let mut started = false;
    let mut raw_filename = String::new();

    let mut temp_plant = Plant::empty();
    let mut temp_inorganic = Inorganic::empty();
    let mut temp_graphic = Graphic::empty();
    let mut temp_material_template = MaterialTemplate::empty();
    let mut temp_entity = Entity::empty();
    let mut temp_creature_variation = CreatureVariation::empty();
    let mut temp_unprocessed_raw = UnprocessedRaw::default();

    let mut last_parsed_type = ObjectType::Unknown;
    let mut last_graphic_type = GraphicTypeTag::Unknown;
    let mut temp_tile_page = TilePage::empty();
    let mut current_modification = ModificationTag::MainRawBody { raws: Vec::new() };

    // Metadata
    let object_type = read_raw_file_type(raw_file_path)?;
    let mut raw_metadata = RawMetadata::new(
        mod_info_file,
        &object_type,
        raw_filename.as_str(),
        &raw_file_path,
        options.attach_metadata_to_raws,
    );

    // If we aren't supposed to parse this type, we should quit here
    if !options.object_types_to_parse.contains(&object_type) {
        debug!(
            "parse_raw_file_with_info: Quitting early because object type {:?} is not included in options!",
            object_type
        );
        return Ok(FileParseResult {
            parsed_raws: Vec::new(),
            unprocessed_raws: Vec::new(),
        });
    }

    // If the type of object is not in our known_list, we should quit here
    if !PARSABLE_OBJECT_TYPES.contains(&&object_type) {
        debug!(
            "parse_raw_file_with_info: Quitting early because object type {:?} is not parsable!",
            object_type
        );
        return Ok(FileParseResult {
            parsed_raws: Vec::new(),
            unprocessed_raws: Vec::new(),
        });
    }

    for (index, line) in reader.lines().enumerate() {
        if line.is_err() {
            error!(
                "parse_raw_file_with_info: Error processing {}:{}",
                raw_file_path.as_ref().display(),
                index
            );
            continue;
        }
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                error!("parse_raw_file_with_info: Line-reading error\n{:?}", e);
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
                options.attach_metadata_to_raws,
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

            trace!(
                "parse_raw_file_with_info: Key: {} Value: {}",
                captured_key,
                captured_value
            );

            match captured_key {
                "OBJECT" => {
                    if !OBJECT_TOKEN_MAP.contains_key(captured_value) {
                        // We don't know what this object is, so we can't parse it.
                        // We should log this as an error.
                        error!(
                            "parse_raw_file_with_info: Unknown object type: {} Raw: {}",
                            captured_value.to_uppercase(),
                            raw_filename
                        );
                        return Err(ParserError::InvalidRawFile(format!(
                            "Unknown object type: {}",
                            captured_value.to_uppercase()
                        )));
                    }
                    // Check of object_type matches the captured_value as ObjectType.
                    // If it doesn't, we should log this as an error.
                    if &object_type
                        != OBJECT_TOKEN_MAP
                            .get(captured_value)
                            .unwrap_or(&ObjectType::Unknown)
                    {
                        error!(
                            "parse_raw_file_with_info: Object type mismatch: {} != {}",
                            object_type,
                            captured_value.to_uppercase()
                        );
                        return Err(ParserError::InvalidRawFile(format!(
                            "Object type mismatch: {} != {}",
                            object_type,
                            captured_value.to_uppercase()
                        )));
                    }
                }
                "CREATURE" | "SELECT_CREATURE" => {
                    // The entity object has a CREATURE tag
                    if started && last_parsed_type == ObjectType::Entity {
                        // We need to let the entity parse this tag.
                        temp_entity.parse_tag(captured_key, captured_value);
                        // leave before adding a fake new creature
                        continue;
                    }

                    if started
                        && (last_parsed_type == ObjectType::Creature
                            || last_parsed_type == ObjectType::CreatureCaste
                            || last_parsed_type == ObjectType::SelectCreature)
                    {
                        temp_unprocessed_raw.add_modification(current_modification.clone());
                        // We need to add the creature to the list of unprocessed raws.
                        unprocessed_raws.push(temp_unprocessed_raw.clone());
                    } else {
                        started = true;
                    }
                    // We haven't started a creature yet, so we need to start one.
                    temp_unprocessed_raw =
                        UnprocessedRaw::new(&ObjectType::Creature, &raw_metadata, captured_value);
                    current_modification = ModificationTag::MainRawBody { raws: Vec::new() };
                    last_parsed_type = ObjectType::Creature;
                }
                "CREATURE_VARIATION" => {
                    if started && last_parsed_type == ObjectType::CreatureVariation {
                        // We need to add the creature to the list.
                        created_raws.push(Box::new(temp_creature_variation.clone()));
                    } else {
                        started = true;
                    }
                    // We haven't started a creature variation yet, so we need to start one.
                    temp_creature_variation =
                        CreatureVariation::new(captured_value, &raw_metadata.clone());
                    last_parsed_type = ObjectType::CreatureVariation;
                }
                "CASTE" | "SELECT_CASTE" => {
                    if object_type != ObjectType::Creature
                        && object_type != ObjectType::Entity
                        && object_type != ObjectType::Graphics
                    {
                        // Currently unhandled outside of these configurations.
                        continue;
                    }
                    if started && object_type == ObjectType::Entity {
                        // We need to let the entity parse this tag.
                        temp_entity.parse_tag(captured_key, captured_value);
                        // leave before adding a fake new creature
                        continue;
                    }
                    // Starting a new caste (in creature), so we can just add a caste to the last creature we started.
                    current_modification.add_raw(format!("{captured_key}:{captured_value}"));

                    last_parsed_type = ObjectType::CreatureCaste;
                }
                "PLANT" => {
                    // Starting a new plant, so we can just add a plant to the list.
                    if started {
                        // We need to add the plant to the list.
                        created_raws.push(Box::new(temp_plant.clone()));
                    } else {
                        started = true;
                    }
                    // We haven't started a plant yet, so we need to start one.
                    temp_plant = Plant::new(captured_value, &raw_metadata.clone());
                    last_parsed_type = ObjectType::Plant;
                }
                "INORGANIC" | "SELECT_INORGANIC" => {
                    if started {
                        // We've already started a raw, so we need to finish it.
                        // This is a new creature, so we need to finish the old one.
                        created_raws.push(Box::new(temp_inorganic.clone()));
                    } else {
                        started = true;
                    }
                    temp_inorganic = Inorganic::new(captured_value, &raw_metadata.clone());
                    last_parsed_type = ObjectType::Inorganic;
                }
                "MATERIAL_TEMPLATE" => {
                    // Starting a new material template, so we can just add a material template to the list.
                    if started {
                        // We need to add the material template to the list.
                        created_raws.push(Box::new(temp_material_template.clone()));
                    } else {
                        started = true;
                    }
                    // We haven't started a material template yet, so we need to start one.
                    temp_material_template =
                        MaterialTemplate::new(captured_value, &raw_metadata.clone());
                    last_parsed_type = ObjectType::MaterialTemplate;
                }
                "CREATURE_GRAPHICS"
                | "CREATURE_CASTE_GRAPHICS"
                | "TILE_GRAPHICS"
                | "PLANT_GRAPHICS" => {
                    // Starting a new graphic, so we can just add a graphic to the list.
                    if started {
                        // We need to add the graphic to the list.
                        created_raws.push(Box::new(temp_graphic.clone()));
                    } else {
                        started = true;
                    }
                    // We haven't started a graphic yet, so we need to start one.

                    last_parsed_type = ObjectType::Graphics;
                    last_graphic_type = *GRAPHIC_TYPE_TOKENS
                        .get(captured_key)
                        .unwrap_or(&GraphicTypeTag::Unknown);

                    temp_graphic =
                        Graphic::new(captured_value, &raw_metadata.clone(), last_graphic_type);
                }
                "TILE_PAGE" => {
                    if started {
                        // We need to add the tile page to the list.
                        created_raws.push(Box::new(temp_tile_page.clone()));
                    } else {
                        started = true;
                    }
                    // We haven't started a tile page yet, so we need to start one.
                    temp_tile_page = TilePage::new(captured_value, &raw_metadata.clone());
                    last_parsed_type = ObjectType::TilePage;
                }
                "ENTITY" => {
                    // Starting a new entity, so we can just add an entity to the list.
                    if started {
                        // We need to add the entity to the list.
                        created_raws.push(Box::new(temp_entity.clone()));
                    } else {
                        started = true;
                    }
                    // We haven't started an entity yet, so we need to start one.
                    temp_entity = Entity::new(captured_value, &raw_metadata.clone());
                    last_parsed_type = ObjectType::Entity;
                }
                "GO_TO_END" => {
                    debug!("began tracking AddToEnding modification");
                    // Push the current modification to the unprocessed raw
                    temp_unprocessed_raw.add_modification(current_modification.clone());
                    // Update the current modification to be an AddToEnding
                    current_modification = ModificationTag::AddToEnding { raws: Vec::new() };
                }
                "GO_TO_START" => {
                    debug!("began tracking AddToBeginning modification");
                    // Push the current modification to the unprocessed raw
                    temp_unprocessed_raw.add_modification(current_modification.clone());
                    // Update the current modification to be an AddToBeginning
                    current_modification = ModificationTag::AddToBeginning { raws: Vec::new() };
                }
                "GO_TO_TAG" => {
                    debug!("began tracking AddBeforeTag:{captured_value} modification");
                    // Push the current modification to the unprocessed raw
                    temp_unprocessed_raw.add_modification(current_modification.clone());
                    // Update the current modification to be an AddBeforeTag
                    current_modification = ModificationTag::AddBeforeTag {
                        tag: captured_value.to_string(),
                        raws: Vec::new(),
                    };
                }
                "COPY_TAGS_FROM" => {
                    debug!("began tracking CopyTagsFrom:{captured_value} modification");
                    // Push the CopyTagsFrom modification to the unprocessed raw
                    temp_unprocessed_raw.add_modification(ModificationTag::CopyTagsFrom {
                        identifier: captured_value.to_string(),
                    });
                }
                "APPLY_CREATURE_VARIATION" => {
                    debug!("began tracking ApplyCreatureVariation:{captured_value} modification");
                    // Push the ApplyCreatureVariation modification to the unprocessed raw
                    temp_unprocessed_raw.add_modification(
                        ModificationTag::ApplyCreatureVariation {
                            identifier: captured_value.to_string(),
                        },
                    );
                }
                _ => {
                    // This should be a tag for the current object.
                    // We should check if we have a current object, and if we do, we should add the tag to it.
                    // If we haven't started yet, we should do nothing.
                    if started {
                        match last_parsed_type {
                            ObjectType::Creature
                            | ObjectType::CreatureCaste
                            | ObjectType::SelectCreature => {
                                // We have a creature, so we can add a tag to it. We need to determine which section
                                // of the creature we are adding to.
                                current_modification
                                    .add_raw(format!("{captured_key}:{captured_value}"));
                            }
                            ObjectType::CreatureVariation => {
                                // We have a creature variation, so we can add a tag to it.
                                temp_creature_variation.parse_tag(captured_key, captured_value);
                            }
                            ObjectType::Plant => {
                                // We have a plant, so we can add a tag to it.
                                temp_plant.parse_tag(captured_key, captured_value);
                            }
                            ObjectType::Inorganic => {
                                // We have an inorganic, so we can add a tag to it.
                                temp_inorganic.parse_tag(captured_key, captured_value);
                            }
                            ObjectType::MaterialTemplate => {
                                // We have a material template, so we can add a tag to it.
                                temp_material_template.parse_tag(captured_key, captured_value);
                            }
                            ObjectType::Graphics => {
                                // We have a graphic, so we can add a tag to it.
                                if temp_graphic.get_graphic_type() == GraphicTypeTag::Tile {
                                    // Update graphic type (every line should have a graphic type tag)
                                    last_graphic_type = *GRAPHIC_TYPE_TOKENS
                                        .get(captured_key)
                                        .unwrap_or(&GraphicTypeTag::Unknown);
                                }

                                temp_graphic.parse_sprite_from_tag(
                                    captured_key,
                                    captured_value,
                                    last_graphic_type,
                                );
                            }
                            ObjectType::TilePage => {
                                // We have a tile page, so we can add a tag to it.
                                temp_tile_page.parse_tag(captured_key, captured_value);
                            }
                            ObjectType::Entity => {
                                // We have an entity, so we can add a tag to it.
                                temp_entity.parse_tag(captured_key, captured_value);
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
        if !temp_unprocessed_raw.is_empty() {
            temp_unprocessed_raw.add_modification(current_modification.clone());
            unprocessed_raws.push(temp_unprocessed_raw.clone());
        }
        if !temp_plant.is_empty() {
            created_raws.push(Box::new(temp_plant.clone()));
        }
        if !temp_inorganic.is_empty() {
            created_raws.push(Box::new(temp_inorganic.clone()));
        }
        if !temp_material_template.is_empty() {
            created_raws.push(Box::new(temp_material_template.clone()));
        }
        if !temp_graphic.is_empty() {
            created_raws.push(Box::new(temp_graphic.clone()));
        }
        if !temp_tile_page.is_empty() {
            created_raws.push(Box::new(temp_tile_page.clone()));
        }
        if !temp_entity.is_empty() {
            created_raws.push(Box::new(temp_entity.clone()));
        }
        if !temp_creature_variation.is_empty() {
            created_raws.push(Box::new(temp_creature_variation.clone()));
        }
    }

    debug!(
        "parse_raw_file_with_info: Parsed {} raws from {}",
        created_raws.len(),
        raw_filename
    );

    Ok(FileParseResult {
        parsed_raws: created_raws,
        unprocessed_raws,
    })
}
