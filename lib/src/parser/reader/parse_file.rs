use encoding_rs_io::DecodeReaderBytesBuilder;
use std::{
    io::{BufRead, BufReader},
    path::Path,
};
use tracing::{debug, error, trace};

use crate::{
    creature_variation::CreatureVariation,
    options::ParserOptions,
    parser::{
        creature::Creature,
        entity::Entity,
        graphics::{Graphic, GraphicTypeToken, TilePage, GRAPHIC_TYPE_TOKEN_MAP},
        inorganic::Inorganic,
        material_template::MaterialTemplate,
        module_info_file::ModuleInfoFile,
        plant::Plant,
        select_creature::SelectCreature,
        DF_ENCODING, PARSABLE_OBJECT_TYPES, RAW_TOKEN_RE, {ObjectType, OBJECT_TOKEN_MAP},
        {RawMetadata, RawObject},
    },
    util::try_get_file,
    ParserError,
};

use super::header::read_raw_file_type;

pub fn parse_raw_file<P: AsRef<Path>>(
    raw_file_path: &P,
    options: &ParserOptions,
) -> Result<Vec<Box<dyn RawObject>>, ParserError> {
    let mod_info_file = match ModuleInfoFile::from_raw_file_path(raw_file_path) {
        Ok(m) => m,
        Err(e) => {
            error!(
                "parse_raw_file: Unable to get module info file for {}\n{:?}",
                raw_file_path.as_ref().display(),
                e
            );
            ModuleInfoFile::empty()
        }
    };

    parse_raw_file_with_info(raw_file_path, &mod_info_file, options)
}

#[allow(clippy::too_many_lines)]
pub fn parse_raw_file_with_info<P: AsRef<Path>>(
    raw_file_path: &P,
    mod_info_file: &ModuleInfoFile,
    options: &ParserOptions,
) -> Result<Vec<Box<dyn RawObject>>, ParserError> {
    let mut created_raws: Vec<Box<dyn RawObject>> = Vec::new();

    let file = try_get_file(raw_file_path)?;

    let decoding_reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(*DF_ENCODING))
        .build(file);
    let reader = BufReader::new(decoding_reader);
    let mut started = false;
    let mut raw_filename = String::new();

    let mut temp_creature = Creature::empty();
    let mut temp_select_creature = SelectCreature::empty();
    let mut temp_plant = Plant::empty();
    let mut temp_inorganic = Inorganic::empty();
    let mut temp_graphic = Graphic::empty();
    let mut temp_material_template = MaterialTemplate::empty();
    let mut temp_entity = Entity::empty();
    let mut temp_creature_variation = CreatureVariation::empty();

    let mut last_parsed_type = ObjectType::Unknown;
    let mut last_graphic_type = GraphicTypeToken::Unknown;
    let mut temp_tile_page = TilePage::empty();

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
    if !options.raws_to_parse.contains(&object_type) {
        debug!(
            "parse_raw_file_with_info: Quitting early because object type {:?} is not included in options!",
            object_type
        );
        return Ok(Vec::new());
    }

    // If the type of object is not in our known_list, we should quit here
    if !PARSABLE_OBJECT_TYPES.contains(&&object_type) {
        debug!(
            "parse_raw_file_with_info: Quitting early because object type {:?} is not parsable!",
            object_type
        );
        return Ok(Vec::new());
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
                "CREATURE" => {
                    // The entity object has a CREATURE tag
                    if started && last_parsed_type == ObjectType::Entity {
                        // We need to let the entity parse this tag.
                        temp_entity.parse_tag(captured_key, captured_value);
                        // leave before adding a fake new creature
                        continue;
                    }

                    if started
                        && (last_parsed_type == ObjectType::Creature
                            || last_parsed_type == ObjectType::CreatureCaste)
                    {
                        // We need to add the creature to the list.
                        created_raws.push(Box::new(temp_creature.clone()));
                    } else {
                        started = true;
                    }
                    // We haven't started a creature yet, so we need to start one.
                    temp_creature = Creature::new(captured_value, &raw_metadata.clone());
                    last_parsed_type = ObjectType::Creature;
                }
                "SELECT_CREATURE" => {
                    if started && last_parsed_type == ObjectType::SelectCreature {
                        // We need to add the creature to the list.
                        created_raws.push(Box::new(temp_select_creature.clone()));
                    } else {
                        started = true;
                    }
                    // We haven't started a creature yet, so we need to start one.
                    temp_select_creature =
                        SelectCreature::new(captured_value, &raw_metadata.clone());
                    last_parsed_type = ObjectType::SelectCreature;
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
                "CASTE" => {
                    if object_type != ObjectType::Creature
                        && object_type != ObjectType::Entity
                        && object_type != ObjectType::Graphics
                    {
                        // Currently unhandled outside of these configurations.
                        continue;
                    }
                    // Starting a new caste (in creature), so we can just add a caste to the last creature we started.
                    if started
                        && (last_parsed_type == ObjectType::CreatureCaste
                            || last_parsed_type == ObjectType::Creature)
                    {
                        // We have a creature, so we can add a caste to it.
                        // First we have to cast the dyn RawObject to a DFCreature.
                        temp_creature.add_caste(captured_value);
                    }
                    last_parsed_type = ObjectType::CreatureCaste;
                }
                "SELECT_CASTE" => {
                    // Starting a new caste (in creature), so we can just add a caste to the last creature we started.
                    if started {
                        // We have a creature, so we can add a caste to it.
                        // First we have to cast the dyn RawObject to a DFCreature.
                        temp_creature.select_caste(captured_value);
                    }
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
                    last_graphic_type = *GRAPHIC_TYPE_TOKEN_MAP
                        .get(captured_key)
                        .unwrap_or(&GraphicTypeToken::Unknown);

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
                _ => {
                    // This should be a tag for the current object.
                    // We should check if we have a current object, and if we do, we should add the tag to it.
                    // If we haven't started yet, we should do nothing.
                    if started {
                        match last_parsed_type {
                            ObjectType::Creature | ObjectType::CreatureCaste => {
                                // We have a creature, so we can add a tag to it.
                                temp_creature.parse_tag(captured_key, captured_value);
                            }
                            ObjectType::SelectCreature => {
                                // We have a creature, so we can add a tag to it.
                                temp_select_creature.parse_tag(captured_key, captured_value);
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
                                if temp_graphic.get_graphic_type() == GraphicTypeToken::Tile {
                                    // Update graphic type (every line should have a graphic type tag)
                                    last_graphic_type = *GRAPHIC_TYPE_TOKEN_MAP
                                        .get(captured_key)
                                        .unwrap_or(&GraphicTypeToken::Unknown);
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
        if !temp_creature.is_empty() {
            created_raws.push(Box::new(temp_creature.clone()));
        }
        if !temp_select_creature.is_empty() {
            created_raws.push(Box::new(temp_select_creature.clone()));
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

    Ok(created_raws)
}
