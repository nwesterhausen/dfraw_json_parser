use encoding_rs_io::DecodeReaderBytesBuilder;
use regex::Regex;
use slug::slugify;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::raws::{creature, names, biomes};

enum RawObjectKind {
    Creature,
    None,
}

pub fn parse_file(input_path: String) -> Vec<creature::DFCreature> {
    let re = Regex::new(r"(\[(?P<key>[^\[:]+):?(?P<value>[^\]\[]*)])").unwrap();
    let enc = encoding_rs::Encoding::for_label("latin1".as_bytes());

    let file = File::open(&input_path).unwrap();
    let decoding_reader = DecodeReaderBytesBuilder::new().encoding(enc).build(file);
    let reader = BufReader::new(decoding_reader);

    // let mut creatures = 0;
    let mut raw_filename = String::new();
    let mut current_object = RawObjectKind::None;
    let mut started = false;
    let mut creature_temp = creature::DFCreature::new("None", "None");
    let mut empty_caste =
        creature::DFCreatureCaste::new("none");
    let mut caste_temp = &mut empty_caste;

    let mut results: Vec<creature::DFCreature> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        if line.is_err() {
            eprintln!("Error processing {}:{}", &input_path, index);
            continue;
        }
        let line = line.unwrap();
        if index == 0 {
            raw_filename = String::from(&line);
            continue;
        }
        for cap in re.captures_iter(&line) {
            // println!("Key: {} Value: {}", &cap[2], &cap[3])
            match &cap[2] {
                "OBJECT" => match &cap[3] {
                    "CREATURE" => {
                        // println!("Discovered raws for creatures.");
                        current_object = RawObjectKind::Creature;
                    }
                    &_ => {
                        println!("No support right now for OBJECT:{}", &cap[3]);
                        return results;
                        // current_object = RawObjectKind::None;
                    }
                },
                "CREATURE" => {
                    // We are starting a creature object capture
                    match current_object {
                        RawObjectKind::Creature => {
                            if started {
                                // If we already *were* capturing a creature, export it.
                                // Reset the temp values !!Todo
                                //println!("{:#?}", creature_temp);
                                // writeln!(stream, "{},", to_string(&creature_temp).unwrap())
                                //  .expect("Unable to write creature info to out.json.");
                                results.push(creature_temp);
                            } else {
                                started = true;
                            }
                            creature_temp = creature::DFCreature::new(&raw_filename, &cap[3]);
                            creature_temp
                                .castes
                                .push(creature::DFCreatureCaste::new("EVERY"));
                            caste_temp = creature_temp.castes.last_mut().unwrap();
                        }
                        RawObjectKind::None => (),
                    }
                    continue;
                }
                "BIOME" => {
                    match biomes::BIOMES.get(&cap[3]) {
                        Some(biome_name) => creature_temp.biomes.push(biome_name.to_string()),
                        None => println!("{} is not in biome dictionary!", &cap[3])
                    }
                }
                "BODY_SIZE" => {
                    let split = cap[3].split(":").collect::<Vec<&str>>();
                    match split.len() {
                        3 => {
                            caste_temp.body_size.push(creature::DFBodySize::new(
                                split[0].parse().expect("Bad year argument for body size"),
                                split[1].parse().expect("Bad days argument for body size"),
                                split[2].parse().expect("Bad size argument for body size")
                            ))
                        }
                        _ => ()
                    }
                }
                "PREFSTRING" => {
                    creature_temp.pref_string.push(String::from(&cap[3]));
                }
                "NAME" => {
                    creature_temp.name = names::Name::new(String::from(&cap[3]));
                    continue;
                }
                "CASTE" => {
                    creature_temp
                        .castes
                        .push(creature::DFCreatureCaste::new(&cap[3]));
                    caste_temp = creature_temp.castes.last_mut().unwrap();
                    continue;
                }
                "CASTE_NAME" => {
                    caste_temp.caste_name = names::Name::new(String::from(&cap[3]));
                    continue;
                }
                "GENERAL_CHILD_NAME" => {
                    creature_temp.general_child_name =
                        names::SingPlurName::new(String::from(&cap[3]));
                    continue;
                }
                "LAYS_EGGS" => {
                    caste_temp.lays_eggs = true;
                    continue;
                }
                "EGG_SIZE" => {
                    caste_temp.egg_size = cap[3].parse().expect("EGG_SIZE should be an integer");
                    continue;
                }
                "CLUTCH_SIZE" => {
                    let split = cap[3].split(":").collect::<Vec<&str>>();
                    caste_temp.clutch_size[0] = split[0]
                        .parse()
                        .expect("CLUTCH_SIZE min should be an integer");
                    caste_temp.clutch_size[1] = split[1]
                        .parse()
                        .expect("CLUTCH_SIZE max should be an integer");
                    continue;
                }
                "DESCRIPTION" => {
                    caste_temp.description = String::from(&cap[3]);
                    continue;
                }
                "MAXAGE" => {
                    let split = cap[3].split(":").collect::<Vec<&str>>();
                    caste_temp.max_age[0] =
                        split[0].parse().expect("MAXAGE min should be an integer");
                    caste_temp.max_age[1] =
                        split[1].parse().expect("MAXAGE max should be an integer");
                    continue;
                }
                "COPY_TAGS_FROM" => {
                    creature_temp.copy_tags_from.push(format!(
                        "{}-{}-{}",
                        raw_filename,
                        "CREATURE",
                        slugify(&cap[3])
                    ));
                    continue;
                }
                "ALL_ACTIVE" => {
                    caste_temp.active_time = caste_temp.active_time | creature::ACTIVE_DIURNAL | creature::ACTIVE_NOCTURNAL | creature::ACTIVE_CREPUSCULAR;
                }
                "DIURNAL" => {
                    caste_temp.active_time = caste_temp.active_time | creature::ACTIVE_DIURNAL;
                }
                "CREPUSCULAR" => {
                    caste_temp.active_time = caste_temp.active_time | creature::ACTIVE_CREPUSCULAR;
                }
                "MATUTINAL" => {
                    caste_temp.active_time = caste_temp.active_time | creature::ACTIVE_MATUTINAL;
                }
                "VESPERTINE" => {
                    caste_temp.active_time = caste_temp.active_time | creature::ACTIVE_VESPERTINE;
                }
                "NOCTURNAL" => {
                    caste_temp.active_time = caste_temp.active_time | creature::ACTIVE_NOCTURNAL;
                }
                "AMBUSHPREDATOR" => {
                    caste_temp.ambush_predator = true;
                }
                "AMPHIBIOUS" => {
                    caste_temp.amphibious = true;
                }
                "CURIOUSBEAST_EATER" => {
                    caste_temp.curious_beast = caste_temp.curious_beast | creature::CURIOUS_EATER;
                }
                "CURIOUSBEAST_GUZZLER" => {
                    caste_temp.curious_beast = caste_temp.curious_beast | creature::CURIOUS_GUZZLER;
                }
                "CURIOUSBEAST_ITEM" => {
                    caste_temp.curious_beast = caste_temp.curious_beast | creature::CURIOUS_ITEM;
                }
                "NO_SPRING" => {
                    caste_temp.no_season = caste_temp.no_season | creature::NO_SPRING;
                }
                "NO_SUMMER" => {
                    caste_temp.no_season = caste_temp.no_season | creature::NO_SUMMER;
                }
                "NO_AUTUMN" => {
                    caste_temp.no_season = caste_temp.no_season | creature::NO_FALL;
                }
                "NO_WINTER" => {
                    caste_temp.no_season = caste_temp.no_season | creature::NO_WINTER;
                }
                "TRAINABLE_HUNTING" => {
                    caste_temp.trainable = caste_temp.trainable | creature::TRAINABLE_HUNTING;
                }
                "TRAINABLE_WAR" => {
                    caste_temp.trainable = caste_temp.trainable | creature::TRAINABLE_WAR;
                }
                "TRAINABLE" => {
                    caste_temp.trainable = caste_temp.trainable | creature::TRAINABLE_WAR | creature::TRAINABLE_HUNTING;
                }
                &_ => (),
            }
        }
    }
    match current_object {
        RawObjectKind::Creature => {
            // If we already *were* capturing a creature, export it.
            // println!("Finished capturing creature, now finished");
            // Reset the temp values !!Todo
            //println!("{:#?}", creature_temp);
            results.push(creature_temp);
        }
        RawObjectKind::None => (),
    }
    // println!("{} creatures defined in {}", creatures, &raw_filename);
    results
}
