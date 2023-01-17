use encoding_rs_io::DecodeReaderBytesBuilder;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::parser::raws::info::DFInfoFile;
use crate::parser::raws::{biomes, material, names, plant, tags};
use crate::parser::reader::RawObjectKind;
use crate::parser::refs::{DF_ENCODING, RAW_TOKEN_RE};

#[allow(clippy::too_many_lines)]
pub fn parse<P: AsRef<Path>>(input_path: &P, info_text: &DFInfoFile) -> Vec<plant::DFPlant> {
    let caller = "Parse Plant Raw";
    let mut results: Vec<plant::DFPlant> = Vec::new();

    let file = match File::open(input_path) {
        Ok(f) => f,
        Err(e) => {
            log::error!("{} - Error opening raw file for parsing!\n{:?}", caller, e);
            return results;
        }
    };

    let decoding_reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(*DF_ENCODING))
        .build(file);
    let reader = BufReader::new(decoding_reader);

    // let mut creatures = 0;
    let mut raw_filename = String::new();
    let mut current_object = RawObjectKind::None;
    let mut started = false;
    let mut plant_temp = plant::DFPlant::new("None", "None", info_text);

    let mut material_tags: Vec<tags::MaterialTag> = Vec::new();
    let mut plant_tags: Vec<tags::PlantTag> = Vec::new();
    let mut temp_material_vec: Vec<material::SimpleMaterial> = Vec::new();
    let mut temp_plant_growth = plant::Growth::None;

    let mut material_temp = material::SimpleMaterial::empty();

    for (index, line) in reader.lines().enumerate() {
        if line.is_err() {
            log::error!(
                "{} - Error processing {}:{}",
                caller,
                input_path.as_ref().display(),
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
            continue;
        }

        for cap in RAW_TOKEN_RE.captures_iter(&line) {
            log::trace!("{} - Key: {} Value: {}", caller, &cap[2], &cap[3]);
            match &cap[2] {
                "OBJECT" => match &cap[3] {
                    "PLANT" => {
                        // Discovered raws for plants.
                        current_object = RawObjectKind::Plant;
                    }
                    &_ => {
                        log::debug!("{} - Wrong type of raw ({})", caller, &cap[3]);
                        return Vec::new();
                        // current_object = RawObjectKind::None;
                    }
                },
                "PLANT" | "SELECT_PLANT" => {
                    // We are starting a creature object capture
                    if let RawObjectKind::Plant = current_object {
                        if started {
                            // If we already *were* capturing a creature, export it.
                            //1. Save caste tags
                            material_temp.tags = material_tags.clone();
                            //2. Save caste
                            temp_material_vec.push(material_temp.clone());
                            //3. Save creature tags
                            plant_temp.tags = plant_tags.clone();
                            //4. Save tamp_castes to creature
                            plant_temp.materials_vec = temp_material_vec.clone();
                            //5. Save creature
                            results.push(plant_temp);
                        } else {
                            started = true;
                        }
                        //Reset all temp values
                        log::debug!("Starting new plant {}", &cap[3]);
                        //1. Make new creature from [CREATURE:<NAME>]
                        plant_temp = plant::DFPlant::new(&raw_filename, &cap[3], info_text);
                        //2. Make new caste
                        material_temp = material::SimpleMaterial::empty();
                        //3. Reset/empty caste tags
                        material_tags = Vec::new();
                        //4. Reset/empty creature tags
                        plant_tags = Vec::new();
                        //5. Reset/empty caste vector
                        temp_material_vec = Vec::new();

                        // Apply overwrites_raw if this is a SELECT tag
                        if cap[2].eq("SELECT_PLANT") {
                            plant_temp.set_overwrites_raw(&cap[3]);
                        }
                    }
                }
                "CUT_USE_MATERIAL_TEMPLATE" => {
                    // We will have to add one of these for each tag we support cutting..
                    plant_temp.push_cut_tag(&cap[2], &cap[3]);
                }
                "USE_MATERIAL_TEMPLATE" => {
                    //1. Save caste tags
                    material_tags.extend(material_temp.tags);
                    material_temp.tags = material_tags.clone();
                    //2. Save caste
                    temp_material_vec.push(material_temp.clone());

                    // Split the value into a descriptor and template
                    let split = cap[3].split(':').collect::<Vec<&str>>();
                    if split.len() != 2 {
                        log::error!("Unable to build from material template {}", &cap[3]);
                        // When we can't do anything about the template, just use empty one
                        material_temp = material::SimpleMaterial::empty();
                        material_tags = Vec::new();
                        continue;
                    }

                    log::debug!("Found defined template {} {}", &split[0], &split[1]);
                    //3. Make new caste from [CASTE:<NAME>]
                    material_temp = material::SimpleMaterial::new(split[0], split[1]);
                    //4. Reset/empty caste tags
                    // ~~material_tags = Vec::new();~~
                    //5. Get material template to add (known) template tags
                    material_tags = Vec::clone(&material::tags_from_template(split[1]));
                }
                "BIOME" => {
                    if let Some(biome_name) = biomes::BIOMES.get(&cap[3]) {
                        plant_temp.biomes.push((*biome_name).to_string());
                    } else {
                        log::warn!(
                            "BIOME:{} is not a valid token (in {}); Will add it 'as-is' to biome list",
                            &cap[3],
                            plant_temp.get_raw_header().get_identifier()
                        );
                        plant_temp.biomes.push(String::from(&cap[3]));
                    }
                }
                "GROWTH" => match &cap[3] {
                    "LEAVES" => temp_plant_growth = plant::Growth::Leaves,
                    "FLOWERS" => temp_plant_growth = plant::Growth::Flowers,
                    "FRUIT" => temp_plant_growth = plant::Growth::Fruit,
                    "SPATHES" => temp_plant_growth = plant::Growth::Spathes,
                    "NUT" => temp_plant_growth = plant::Growth::Nut,
                    "SEED_CATKINS" => temp_plant_growth = plant::Growth::SeedCatkins,
                    "POLLEN_CATKINS" => temp_plant_growth = plant::Growth::PollenCatkins,
                    "CONE" => temp_plant_growth = plant::Growth::Cone,
                    "SEED_CONE" => temp_plant_growth = plant::Growth::SeedCone,
                    "POLLEN_CONE" => temp_plant_growth = plant::Growth::PollenCone,
                    "POD" => temp_plant_growth = plant::Growth::Pod,
                    _ => {
                        log::debug!("Un-matched plant growth token '{}'", &cap[3]);
                    }
                },
                "GROWTH_NAME" => {
                    plant_temp
                        .growth_names
                        .insert(temp_plant_growth, names::SingPlurName::new(&cap[3]));
                }
                "ALL_NAMES" => {
                    plant_temp.name.set_all(&cap[3]);
                }
                "NAME" => {
                    plant_temp.name.set_singular(&cap[3]);
                }
                "NAME_PLURAL" => {
                    plant_temp.name.set_plural(&cap[3]);
                }
                "ADJ" => {
                    plant_temp.name.set_adjective(&cap[3]);
                }
                "PREFSTRING" => {
                    plant_temp.pref_string.push(String::from(&cap[3]));
                }
                "FREQUENCY" => match cap[3].parse() {
                    Ok(n) => plant_temp.frequency = n,
                    Err(e) => log::error!(
                        "{}:FREQUENCY parsing error\n{:?}",
                        plant_temp.get_raw_header().get_identifier(),
                        e
                    ),
                },
                "CLUSTERSIZE" => match cap[3].parse() {
                    Ok(n) => plant_temp.cluster_size = n,
                    Err(e) => log::error!(
                        "{}:CLUSTERSIZE parsing error\n{:?}",
                        plant_temp.get_raw_header().get_identifier(),
                        e
                    ),
                },
                "GROWDUR" => match cap[3].parse() {
                    Ok(n) => plant_temp.growth_duration = n,
                    Err(e) => log::error!(
                        "{}:GROWDUR parsing error\n{:?}",
                        plant_temp.get_raw_header().get_identifier(),
                        e
                    ),
                },
                "VALUE" => match cap[3].parse() {
                    Ok(n) => plant_temp.value = n,
                    Err(e) => log::error!(
                        "{}:VALUE parsing error\n{:?}",
                        plant_temp.get_raw_header().get_identifier(),
                        e
                    ),
                },
                "MATERIAL_VALUE" => match cap[3].parse() {
                    Ok(n) => material_temp.material_value = n,
                    Err(e) => log::error!(
                        "{}:{:?}:MATERIAL_VALUE parsing error\n{:?}",
                        plant_temp.get_raw_header().get_identifier(),
                        material_temp.material_type,
                        e
                    ),
                },
                "EDIBLE_VERMIN" => {
                    material_tags.push(tags::MaterialTag::EdibleVermin);
                }
                "EDIBLE_RAW" => {
                    material_tags.push(tags::MaterialTag::EdibleRaw);
                }
                "EDIBLE_COOKED" => {
                    material_tags.push(tags::MaterialTag::EdibleCooked);
                }
                "STATE_NAME" => {
                    material_temp.state_name.set_from_tag(&cap[3]);
                }
                "STATE_ADJ" => {
                    material_temp.state_adj.set_from_tag(&cap[3]);
                }
                "STATE_NAME_ADJ" => {
                    material_temp.state_name.set_from_tag(&cap[3]);
                    material_temp.state_adj.set_from_tag(&cap[3]);
                }
                "STATE_COLOR" => {
                    material_temp.state_color.set_from_tag(&cap[3]);
                }
                "MILL" => {
                    plant_temp.reactions.push(String::from(&cap[3]));
                }
                &_ => (),
            }
        }
    }

    if let RawObjectKind::Plant = current_object {
        // If we already *were* capturing a plant, export it.
        //1. Save caste tags
        material_tags.extend(material_temp.tags);
        material_temp.tags = material_tags.clone();
        //2. Save caste
        temp_material_vec.push(material_temp.clone());
        //3. Save creature tags
        plant_temp.tags = plant_tags;
        //4. Save tamp_castes to creature
        plant_temp.materials_vec = temp_material_vec.clone();
        //5. Save creature
        results.push(plant_temp);
    }
    log::info!(
        "{} plants defined in {} ({} {} in {:?})",
        results.len(),
        &raw_filename,
        info_text.get_identifier(),
        info_text.displayed_version,
        info_text.get_location(),
    );
    results
}
