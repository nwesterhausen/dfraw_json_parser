use encoding_rs_io::DecodeReaderBytesBuilder;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::parser::raws::environment::Environment;
use crate::parser::raws::info_txt::DFInfoFile;
use crate::parser::raws::roll_chance::RollChance;
use crate::parser::raws::{inorganic, material, roll_chance, tags, RawObjectKind};
use crate::parser::refs::{DF_ENCODING, RAW_TOKEN_RE};

impl super::DFInorganic {
    #[allow(clippy::too_many_lines)]
    pub fn parse<P: AsRef<Path>>(
        input_path: &P,
        info_text: &DFInfoFile,
    ) -> Vec<inorganic::DFInorganic> {
        let caller = "Parse Inorganic Raw";
        let mut results: Vec<inorganic::DFInorganic> = Vec::new();

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
        let mut inorganic_temp = inorganic::DFInorganic::new("None", "None", info_text);

        let mut material_tags: Vec<tags::MaterialTag> = Vec::new();
        let mut material_temp = material::SimpleMaterial::empty();
        let mut environments_temp: Vec<Environment> = Vec::new();
        let mut environments_spec_temp: Vec<Environment> = Vec::new();
        let mut inorganic_tags: Vec<tags::InorganicTag> = Vec::new();
        let mut metal_ores: Vec<roll_chance::RollChance> = Vec::new();
        let mut metal_threads: Vec<roll_chance::RollChance> = Vec::new();

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
                    "OBJECT" => match captured_value {
                        "INORGANIC" => {
                            // Discovered raws for plants.
                            current_object = RawObjectKind::Inorganic;
                        }
                        &_ => {
                            log::debug!("{} - Wrong type of raw ({})", caller, captured_value);
                            return Vec::new();
                            // current_object = RawObjectKind::None;
                        }
                    },
                    "INORGANIC" | "SELECT_INORGANIC" => {
                        // We are starting a creature object capture
                        if let RawObjectKind::Inorganic = current_object {
                            if started {
                                // If we already *were* capturing, export it.
                                //1. Save material tags
                                material_temp.tags = material_tags;
                                //2a. Save inorganic environment
                                inorganic_temp.environments = environments_temp;
                                inorganic_temp.environments_specific = environments_spec_temp;
                                //2b. Save inorganic metal produced
                                inorganic_temp.metal_ores = metal_ores;
                                inorganic_temp.thread_metals = metal_threads;
                                //3. Save creature tags
                                inorganic_temp.tags = inorganic_tags;
                                //2. Save material&cap\[3\]
                                inorganic_temp.material = material_temp;
                                //5. Save creature
                                results.push(inorganic_temp);
                            } else {
                                started = true;
                            }
                            //Reset all temp values
                            log::debug!("Starting new inorganic {}", captured_value);
                            //1. Make new inorganic from [INORGANIC:<NAME>]
                            inorganic_temp = inorganic::DFInorganic::new(
                                &raw_filename,
                                captured_value,
                                info_text,
                            );
                            //2. Make new material
                            material_temp = material::SimpleMaterial::empty();
                            //3. Reset/empty caste tags
                            material_tags = Vec::new();
                            environments_temp = Vec::new();
                            environments_spec_temp = Vec::new();
                            inorganic_tags = Vec::new();
                            metal_ores = Vec::new();
                            metal_threads = Vec::new();

                            // Apply overwrites_raw if this is a SELECT tag
                            if captured_key.eq("SELECT_INORGANIC") {
                                inorganic_temp.set_overwrites_raw(captured_value);
                            }
                        }
                    }
                    "USE_MATERIAL_TEMPLATE" => {
                        // As far as I know, inorganics have a single material template.

                        log::debug!("Found defined template {}", captured_value);
                        //3. Make new caste from [CASTE:<NAME>]
                        material_temp =
                            material::SimpleMaterial::new(captured_value, captured_value);
                        //4. Reset/empty caste tags
                        // ~~material_tags = Vec::new();~~
                        environments_temp = Vec::new();
                        //5. Get material template to add (known) template tags
                        material_tags = Vec::clone(&material::tags_from_template(captured_value));
                    }
                    "CUT_USE_MATERIAL_TEMPLATE" => {
                        // We will have to add one of these for each tag we support cutting..
                        inorganic_temp.push_cut_tag(captured_key, captured_value);
                    }
                    "PREFSTRING" => {
                        log::warn!(
                            "THERE INDEED WERE PREF STRING FOR {}: {}",
                            inorganic_temp.get_raw_header().get_object_id(),
                            captured_value
                        );
                    }
                    "REACTION_CLASS" => {
                        material_temp
                            .reaction_classes
                            .push(String::from(captured_value));
                    }
                    "MATERIAL_VALUE" => match captured_value.parse() {
                        Ok(n) => material_temp.material_value = n,
                        Err(e) => log::error!(
                            "{}:{:?}:MATERIAL_VALUE parsing error\n{:?}",
                            inorganic_temp.get_raw_header().get_object_id(),
                            material_temp.material_type,
                            e
                        ),
                    },
                    "STATE_NAME" => {
                        material_temp.state_name.set_from_tag(captured_value);
                    }
                    "STATE_ADJ" => {
                        material_temp.state_adj.set_from_tag(captured_value);
                    }
                    "STATE_NAME_ADJ" => {
                        material_temp.state_name.set_from_tag(captured_value);
                        material_temp.state_adj.set_from_tag(captured_value);
                    }
                    "STATE_COLOR" => {
                        material_temp.state_color.set_from_tag(captured_value);
                    }

                    "NO_STONE_STOCKPILE" => {
                        material_tags.push(tags::MaterialTag::NoStoneStockpile);
                    }
                    "DISPLAY_UNGLAZED" => {
                        material_tags.push(tags::MaterialTag::DisplayUnglazed);
                    }
                    "IS_STONE" => {
                        material_tags.push(tags::MaterialTag::IsStone);
                    }
                    "IS_CERAMIC" => {
                        material_tags.push(tags::MaterialTag::IsCeramic);
                    }
                    "IS_METAL" => {
                        material_tags.push(tags::MaterialTag::IsMetal);
                    }
                    "ITEMS_WEAPON" => {
                        material_tags.push(tags::MaterialTag::ItemsWeapon);
                    }
                    "ITEMS_WEAPON_RANGED" => {
                        material_tags.push(tags::MaterialTag::ItemsWeaponRanged);
                    }
                    "ITEMS_AMMO" => {
                        material_tags.push(tags::MaterialTag::ItemsAmmo);
                    }
                    "ITEMS_DIGGER" => {
                        material_tags.push(tags::MaterialTag::ItemsDigger);
                    }
                    "ITEMS_ARMOR" => {
                        material_tags.push(tags::MaterialTag::ItemsArmor);
                    }
                    "ITEMS_ANVIL" => {
                        material_tags.push(tags::MaterialTag::ItemsAnvil);
                    }
                    "ITEMS_HARD" => {
                        material_tags.push(tags::MaterialTag::ItemsHard);
                    }
                    "ITEMS_METAL" => {
                        material_tags.push(tags::MaterialTag::ItemsMetal);
                    }
                    "ITEMS_BARRED" => {
                        material_tags.push(tags::MaterialTag::ItemsBarred);
                    }
                    "ITEMS_SCALED" => {
                        material_tags.push(tags::MaterialTag::ItemsScaled);
                    }
                    "SEDIMENTARY" => {
                        inorganic_tags.push(tags::InorganicTag::Sedimentary);
                    }
                    "SEDIMENTARY_OCEAN_SHALLOW" => {
                        inorganic_tags.push(tags::InorganicTag::SedimentaryOceanShallow);
                    }
                    "AQUIFER" => {
                        inorganic_tags.push(tags::InorganicTag::Aquifer);
                    }
                    "SEDIMENTARY_OCEAN_DEEP" => {
                        inorganic_tags.push(tags::InorganicTag::SedimentaryOceanDeep);
                    }
                    "IGNEOUS_INTRUSIVE" => {
                        inorganic_tags.push(tags::InorganicTag::IgneousIntrusive);
                    }
                    "IGNEOUS_EXTRUSIVE" => {
                        inorganic_tags.push(tags::InorganicTag::IgneousExtrusive);
                    }
                    "METAMORPHIC" => {
                        inorganic_tags.push(tags::InorganicTag::Metamorphic);
                    }
                    "LAVA" => {
                        inorganic_tags.push(tags::InorganicTag::Lava);
                    }
                    "WAFERS" => {
                        inorganic_tags.push(tags::InorganicTag::Wafers);
                    }
                    "DEEP_SPECIAL" => {
                        inorganic_tags.push(tags::InorganicTag::DeepSpecial);
                    }
                    "DEEP_SURFACE" => {
                        inorganic_tags.push(tags::InorganicTag::DeepSurface);
                    }
                    "SPECIAL" => {
                        inorganic_tags.push(tags::InorganicTag::Special);
                    }
                    "GENERATED" => {
                        inorganic_tags.push(tags::InorganicTag::Generated);
                    }
                    "DIVINE" => {
                        inorganic_tags.push(tags::InorganicTag::Divine);
                    }
                    "SOIL" => {
                        inorganic_tags.push(tags::InorganicTag::Soil);
                    }
                    "SOIL_OCEAN" => {
                        inorganic_tags.push(tags::InorganicTag::SoilOcean);
                    }
                    "SOIL_SAND" => {
                        inorganic_tags.push(tags::InorganicTag::SoilSand);
                    }
                    "ENVIRONMENT" => {
                        environments_temp.push(Environment::from_tag(captured_value));
                    }
                    "ENVIRONMENT_SPEC" => {
                        environments_spec_temp.push(Environment::from_tag(captured_value));
                    }
                    "METAL_ORE" => {
                        if let Some(rc) = RollChance::from_tag(captured_value) {
                            metal_ores.push(rc);
                        }
                    }
                    "THREAD_METAL" => {
                        if let Some(rc) = RollChance::from_tag(captured_value) {
                            metal_threads.push(rc);
                        }
                    }
                    "SPEC_HEAT" => {
                        if captured_value.eq("NONE") {
                            material_temp.temperatures.material_fixed_temp = 0;
                            continue;
                        }

                        match captured_value.parse() {
                            Ok(n) => material_temp.temperatures.specific_heat = n,
                            Err(e) => log::error!(
                                "{}:SPEC_HEAT parsing error\n{:?}",
                                inorganic_temp.get_raw_header().get_object_id(),
                                e
                            ),
                        }
                    }
                    "IGNITE_POINT" => {
                        if captured_value.eq("NONE") {
                            material_temp.temperatures.material_fixed_temp = 0;
                            continue;
                        }

                        match captured_value.parse() {
                            Ok(n) => material_temp.temperatures.ignition_point = n,
                            Err(e) => log::error!(
                                "{}:IGNITE_POINT parsing error\n{:?}",
                                inorganic_temp.get_raw_header().get_object_id(),
                                e
                            ),
                        }
                    }
                    "MELTING_POINT" => {
                        if captured_value.eq("NONE") {
                            material_temp.temperatures.material_fixed_temp = 0;
                            continue;
                        }

                        match captured_value.parse() {
                            Ok(n) => material_temp.temperatures.melting_point = n,
                            Err(e) => log::error!(
                                "{}:MELTING_POINT parsing error\n{:?}",
                                inorganic_temp.get_raw_header().get_object_id(),
                                e
                            ),
                        }
                    }
                    "BOILING_POINT" => {
                        if captured_value.eq("NONE") {
                            material_temp.temperatures.material_fixed_temp = 0;
                            continue;
                        }

                        match captured_value.parse() {
                            Ok(n) => material_temp.temperatures.boiling_point = n,
                            Err(e) => log::error!(
                                "{}:BOILING_POINT parsing error\n{:?}",
                                inorganic_temp.get_raw_header().get_object_id(),
                                e
                            ),
                        }
                    }
                    "HEATDAM_POINT" => {
                        if captured_value.eq("NONE") {
                            material_temp.temperatures.material_fixed_temp = 0;
                            continue;
                        }

                        match captured_value.parse() {
                            Ok(n) => material_temp.temperatures.heat_damage_point = n,
                            Err(e) => log::error!(
                                "{}:HEATDAM_POINT parsing error\n{:?}",
                                inorganic_temp.get_raw_header().get_object_id(),
                                e
                            ),
                        }
                    }
                    "COLDDAM_POINT" => {
                        if captured_value.eq("NONE") {
                            material_temp.temperatures.material_fixed_temp = 0;
                            continue;
                        }

                        match captured_value.parse() {
                            Ok(n) => material_temp.temperatures.cold_damage_point = n,
                            Err(e) => log::error!(
                                "{}:COLDDAM_POINT parsing error\n{:?}",
                                inorganic_temp.get_raw_header().get_object_id(),
                                e
                            ),
                        }
                    }
                    "MAT_FIXED_TEMP" => {
                        if captured_value.eq("NONE") {
                            material_temp.temperatures.material_fixed_temp = 0;
                            continue;
                        }

                        match captured_value.parse() {
                            Ok(n) => material_temp.temperatures.material_fixed_temp = n,
                            Err(e) => log::error!(
                                "{}:MAT_FIXED_TEMP parsing error\n{:?}",
                                inorganic_temp.get_raw_header().get_object_id(),
                                e
                            ),
                        }
                    }
                    &_ => (),
                }
            }
        }

        if let RawObjectKind::Inorganic = current_object {
            // If we already *were* capturing, export it.
            //1. Save material tags
            material_tags.extend(material_temp.tags);
            material_temp.tags = material_tags;
            //2a. Save inorganic environment
            inorganic_temp.environments = environments_temp;
            inorganic_temp.environments_specific = environments_spec_temp;
            //2b. Save inorganic metal produced
            inorganic_temp.metal_ores = metal_ores;
            inorganic_temp.thread_metals = metal_threads;
            //3. Save creature tags
            inorganic_temp.tags = inorganic_tags;
            //2. Save material
            inorganic_temp.material = material_temp;
            //5. Save inorganic
            results.push(inorganic_temp);
        }
        log::info!(
            "{} inorganic objects defined in {} ({} {} in {:?})",
            results.len(),
            &raw_filename,
            info_text.get_identifier(),
            info_text.displayed_version,
            info_text.get_location(),
        );
        results
    }
}
