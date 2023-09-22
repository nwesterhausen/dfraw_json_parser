use serde::{Deserialize, Serialize};

use crate::parser::{
    names::{Name, SingPlurName},
    reader::ranges::parse_min_max_range,
};

use super::{
    phf_table::{CASTE_TOKENS, CREATURE_TOKENS},
    tokens::{CasteTag, CreatureTag},
};

#[derive(Serialize, Deserialize, Debug)]
struct DFCreature {
    castes: Vec<DFCaste>,
    tags: Vec<CreatureTag>,
    biomes: Vec<String>,
    general_child_name: SingPlurName,
    general_baby_name: SingPlurName,
    name: Name,
    pref_strings: Vec<String>,
    population_number: [u16; 2],
    underground_depth: [u16; 2],
    frequency: u16,
}

#[derive(Serialize, Deserialize, Debug)]
struct DFCaste {
    identifier: String,
    description: String,
    tags: Vec<CasteTag>,
    pet_value: i32,
}

impl DFCreature {
    fn new() -> DFCreature {
        DFCreature {
            castes: vec![DFCaste::new("general")],
            tags: Vec::new(),
            name: Name::empty(),
            biomes: Vec::new(),
            general_child_name: SingPlurName::empty(),
            general_baby_name: SingPlurName::empty(),
            pref_strings: Vec::new(),
            population_number: [0, 0],
            underground_depth: [0, 0],
            frequency: 0,
        }
    }
    // Add a new caste
    fn add_caste(&mut self, name: &str) {
        self.castes.push(DFCaste::new(name));
    }
    // Move specified caste (by name) to end of case list
    fn select_caste(&mut self, name: &str) {
        let mut index = 0;
        for (i, caste) in self.castes.iter().enumerate() {
            if caste.identifier.eq(name) {
                index = i;
                break;
            }
        }
        let caste = self.castes.remove(index);
        self.castes.push(caste);
    }
    fn parse_tag(&mut self, key: &str, value: &str) {
        if CASTE_TOKENS.contains_key(key) && !self.castes.is_empty() {
            self.castes.last_mut().unwrap().parse_tag(key, value);
        }
        if !CREATURE_TOKENS.contains_key(key) {
            log::warn!("CreatureParsing: Unknown tag {} with value {}", key, value);
            return;
        }

        let Some(tag) = CREATURE_TOKENS.get(key) else {
            log::warn!(
                "CreatureParsing: called `Option::unwrap()` on a `None` value for prsmed creature tag: {}",
                key
            );
            return;
        };

        match tag {
            CreatureTag::Biome => {
                self.biomes.push(String::from(value));
            }
            CreatureTag::Name => {
                self.name = Name::new(value);
            }
            CreatureTag::GeneralBabyName => {
                self.general_baby_name = SingPlurName::new(value);
            }
            CreatureTag::GeneralChildName => {
                self.general_child_name = SingPlurName::new(value);
            }
            CreatureTag::PrefString => {
                self.pref_strings.push(String::from(value));
            }
            CreatureTag::PopulationNumber => {
                self.population_number = parse_min_max_range(value).unwrap_or([0, 0]);
            }
            CreatureTag::Frequency => {
                self.frequency = value.parse::<u16>().unwrap_or(0);
            }
            CreatureTag::UndergroundDepth => {
                self.underground_depth = parse_min_max_range(value).unwrap_or([0, 0]);
            }
            _ => {
                self.tags.push(tag.clone());
            }
        }
    }
}

impl DFCaste {
    fn new(name: &str) -> DFCaste {
        DFCaste {
            identifier: String::from(name),
            description: String::new(),
            tags: Vec::new(),
            pet_value: 0,
        }
    }
    pub fn parse_tag(&mut self, key: &str, value: &str) {
        let Some(tag) = CASTE_TOKENS.get(key) else {
            log::warn!(
                "CreatureParsing: called `Option::unwrap()` on a `None` value for prsmed caste tag: {}",
                key
            );
            return;
        };

        // If value is empty, add the tag to the last caste
        if value.is_empty() {
            self.tags.push(tag.clone());
            return;
        }

        // Todo: Handle special tag cases where value matters.
        if tag == &CasteTag::Description {
            self.description = String::from(value);
        }
        if tag == &CasteTag::PetValue {
            self.pet_value = value.parse::<i32>().unwrap_or(0);
        }
    }
}
