use std::collections::HashMap;

use crate::parser::raws::{
    info_txt::DFInfoFile,
    names::{Name, SingPlurName},
    tags, DFRawCommon, RawObjectKind,
};

impl super::DFCreature {
    pub fn new(raw: &str, id: &str, info_text: &DFInfoFile) -> Self {
        Self {
            raw_header: DFRawCommon::from(id, raw, info_text, RawObjectKind::Creature),

            // Boolean Flags
            tags: Vec::new(),

            // integers
            frequency: 50, //Defaults to 50 if not specified

            // [min, max] ranges
            cluster_number: [1, 1],    //Defaults to 1:1 if not specified.
            population_number: [1, 1], //default 1:1
            underground_depth: [0, 0], //default 0:0 (aboveground)

            // strings
            general_baby_name: SingPlurName::new(""),
            general_child_name: SingPlurName::new(""),
            name: Name::new(""),

            // Arrays
            biomes: Vec::new(),
            pref_string: Vec::new(),

            // sub definitions
            castes: Vec::new(),

            // copy_from
            copy_tags_from: Vec::new(), // vec of creature identifiers
        }
    }
    pub fn get_raw_header(&self) -> &DFRawCommon {
        &self.raw_header
    }
    pub fn set_overwrites_raw(&mut self, raw_name: &str) {
        self.raw_header.overwrites_raw = String::from(raw_name);
    }
    pub fn push_cut_tag(&mut self, tag0: &str, tag1: &str) {
        self.raw_header.push_cut_tag(tag0, tag1);
    }
    pub fn get_general_name(&self) -> String {
        self.name
            .to_string_vec()
            .first()
            .unwrap_or(&String::new())
            .to_string()
    }
    pub fn get_names_by_caste(&self) -> HashMap<String, Vec<String>> {
        let mut names_map: HashMap<String, Vec<String>> = HashMap::new();
        names_map.insert("SPECIES".to_string(), self.name.to_string_vec());
        if !self
            .general_baby_name
            .to_string_vec()
            .first()
            .unwrap_or(&String::new())
            .is_empty()
        {
            names_map.insert(
                "baby_SPECIES".to_string(),
                self.general_baby_name.to_string_vec(),
            );
        }
        if !self
            .general_child_name
            .to_string_vec()
            .first()
            .unwrap_or(&String::new())
            .is_empty()
        {
            names_map.insert(
                "child_SPECIES".to_string(),
                self.general_child_name.to_string_vec(),
            );
        }
        for self_caste in &self.castes {
            if !self_caste
                .baby_name
                .to_string_vec()
                .first()
                .unwrap_or(&String::new())
                .is_empty()
            {
                names_map.insert(
                    format!("baby_{}", self_caste.name),
                    self_caste.baby_name.to_string_vec(),
                );
            }
            if !self_caste
                .child_name
                .to_string_vec()
                .first()
                .unwrap_or(&String::new())
                .is_empty()
            {
                names_map.insert(
                    format!("child_{}", self_caste.name),
                    self_caste.child_name.to_string_vec(),
                );
            }
            if !self_caste
                .caste_name
                .to_string_vec()
                .first()
                .unwrap_or(&String::new())
                .is_empty()
            {
                names_map.insert(
                    self_caste.name.to_string(),
                    self_caste.caste_name.to_string_vec(),
                );
            }
        }
        names_map
    }
    pub fn get_description_by_caste(&self) -> HashMap<String, String> {
        let mut descriptions: HashMap<String, String> = HashMap::new();
        for self_caste in &self.castes {
            descriptions.insert(
                String::from(&self_caste.name),
                String::from(&self_caste.description),
            );
        }
        descriptions
    }
    pub fn get_max_ages_by_caste(&self) -> HashMap<String, [u16; 2]> {
        let mut max_ages: HashMap<String, [u16; 2]> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.max_age[0] != self_caste.max_age[1] && self_caste.max_age[1] != 0 {
                max_ages.insert(String::from(&self_caste.name), self_caste.max_age);
            }
        }
        max_ages
    }
    pub fn get_clutch_sizes_by_caste(&self) -> HashMap<String, [u16; 2]> {
        let mut clutch_sizes: HashMap<String, [u16; 2]> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.clutch_size[0] != self_caste.clutch_size[1]
                && self_caste.clutch_size[1] != 0
            {
                clutch_sizes.insert(String::from(&self_caste.name), self_caste.clutch_size);
            }
        }
        clutch_sizes
    }
    pub fn get_body_sizes_by_caste(&self) -> HashMap<String, Vec<tags::DFBodySize>> {
        let mut body_sizes: HashMap<String, Vec<tags::DFBodySize>> = HashMap::new();
        for self_caste in &self.castes {
            let caste_body_sizes = Vec::clone(&self_caste.body_size);
            body_sizes.insert(String::from(&self_caste.name), caste_body_sizes);
        }
        body_sizes
    }
    pub fn get_milkable_by_caste(&self) -> HashMap<String, tags::DFMilkable> {
        let mut milkable: HashMap<String, tags::DFMilkable> = HashMap::new();
        for self_caste in &self.castes {
            if !self_caste.milkable.is_empty() {
                milkable.insert(String::from(&self_caste.name), self_caste.milkable.clone());
            }
        }
        milkable
    }
    pub fn get_child_ages_by_caste(&self) -> HashMap<String, u32> {
        let mut child_ages: HashMap<String, u32> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.baby != 0 {
                child_ages.insert(String::from(&self_caste.name), self_caste.baby);
            }
        }
        child_ages
    }
    pub fn get_grown_at_ages_by_caste(&self) -> HashMap<String, u32> {
        let mut grown_at_ages: HashMap<String, u32> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.child != 0 {
                grown_at_ages.insert(String::from(&self_caste.name), self_caste.child);
            }
        }
        grown_at_ages
    }
    pub fn get_difficulty_by_caste(&self) -> HashMap<String, u32> {
        let mut difficulty: HashMap<String, u32> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.child != 0 {
                difficulty.insert(String::from(&self_caste.name), self_caste.difficulty);
            }
        }
        difficulty
    }
    pub fn get_grass_trample_by_caste(&self) -> HashMap<String, u8> {
        let mut grass_trample: HashMap<String, u8> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.grass_trample != 0 {
                grass_trample.insert(String::from(&self_caste.name), self_caste.grass_trample);
            }
        }
        grass_trample
    }
    pub fn get_grazer_by_caste(&self) -> HashMap<String, u32> {
        let mut grazer: HashMap<String, u32> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.grazer != 0 {
                grazer.insert(String::from(&self_caste.name), self_caste.grazer);
            }
            if self_caste.tags.contains(&tags::CasteTag::StandardGrazer) {
                if let Some(body_size) = self_caste.body_size.last() {
                    let graze_value: f64 =
                        20_000.0 * 100.0 * (f64::powf(f64::from(body_size.size_cm3() / 10), -0.75));
                    let graze_int = format!("{}", graze_value.round());

                    match graze_int.as_str().parse::<u32>() {
                        Ok(n) => {
                            if n < 150 {
                                grazer.insert(String::from(&self_caste.name), 150);
                            } else {
                                grazer.insert(String::from(&self_caste.name), n);
                            }
                        }
                        Err(e) => {
                            log::warn!(
                                "{}:Unable to create GRAZER value from StandardGrazer",
                                &self.raw_header.identifier
                            );
                            log::warn!("{:?}", e);
                        }
                    }
                }
            }
        }
        grazer
    }
    pub fn get_low_light_vision_by_caste(&self) -> HashMap<String, u32> {
        let mut low_light_vision: HashMap<String, u32> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.low_light_vision != 0 {
                low_light_vision
                    .insert(String::from(&self_caste.name), self_caste.low_light_vision);
            }
        }
        low_light_vision
    }
    pub fn get_egg_sizes_by_caste(&self) -> HashMap<String, u32> {
        let mut values: HashMap<String, u32> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.tags.contains(&tags::CasteTag::LaysEggs) {
                values.insert(String::from(&self_caste.name), self_caste.egg_size);
            }
        }
        values
    }
    pub fn get_pet_value_by_caste(&self) -> HashMap<String, u16> {
        let mut pet_values: HashMap<String, u16> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.pet_value > 0 {
                pet_values.insert(String::from(&self_caste.name), self_caste.pet_value);
            }
        }
        pet_values
    }
    pub fn get_pop_ratio_by_caste(&self) -> HashMap<String, u16> {
        let mut pop_ratio: HashMap<String, u16> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.pop_ratio > 0 {
                pop_ratio.insert(String::from(&self_caste.name), self_caste.pop_ratio);
            }
        }
        pop_ratio
    }
    pub fn get_intelligence_by_caste(&self) -> HashMap<String, [bool; 2]> {
        let mut intelligence: HashMap<String, [bool; 2]> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.tags.contains(&tags::CasteTag::Intelligent) {
                intelligence.insert(String::from(&self_caste.name), [true, true]);
            }
            if self_caste.tags.contains(&tags::CasteTag::CanLearn)
                || self_caste.tags.contains(&tags::CasteTag::CanSpeak)
            {
                intelligence.insert(
                    String::from(&self_caste.name),
                    [
                        self_caste.tags.contains(&tags::CasteTag::CanLearn),
                        self_caste.tags.contains(&tags::CasteTag::CanSpeak),
                    ],
                );
            }
        }
        // Handle the case when the creature is not intelligent
        if intelligence.is_empty() {
            intelligence.insert(String::from("ALL"), [false, false]);
        }
        intelligence
    }
    pub fn get_gnawer_by_caste(&self) -> HashMap<String, bool> {
        let mut gnawer: HashMap<String, bool> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.tags.contains(&tags::CasteTag::Gnawer) {
                gnawer.insert(
                    String::from(&self_caste.name),
                    self_caste.tags.contains(&tags::CasteTag::Gnawer),
                );
            }
        }
        gnawer
    }
    pub fn get_flier_by_caste(&self) -> HashMap<String, bool> {
        let mut flier: HashMap<String, bool> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.tags.contains(&tags::CasteTag::Flier) {
                flier.insert(
                    String::from(&self_caste.name),
                    self_caste.tags.contains(&tags::CasteTag::Flier),
                );
            }
        }
        flier
    }
    pub fn get_trainable_by_caste(&self) -> HashMap<String, u8> {
        let mut trainable: HashMap<String, u8> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.trainable > 0 {
                trainable.insert(String::from(&self_caste.name), self_caste.trainable);
            }
        }
        trainable
    }
    pub fn get_active_time_by_caste(&self) -> HashMap<String, u8> {
        let mut active_time: HashMap<String, u8> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.active_time > 0 {
                active_time.insert(String::from(&self_caste.name), self_caste.active_time);
            }
        }
        active_time
    }
    pub fn get_inactive_season_by_caste(&self) -> HashMap<String, u8> {
        let mut no_season: HashMap<String, u8> = HashMap::new();
        for self_caste in &self.castes {
            if self_caste.no_season > 0 {
                no_season.insert(String::from(&self_caste.name), self_caste.no_season);
            }
        }
        no_season
    }
    pub fn get_creature_class_by_caste(&self) -> HashMap<String, Vec<String>> {
        let mut creature_class: HashMap<String, Vec<String>> = HashMap::new();
        for self_caste in &self.castes {
            if !self_caste.creature_class.is_empty() {
                creature_class.insert(
                    String::from(&self_caste.name),
                    Vec::clone(&self_caste.creature_class),
                );
            }
        }
        creature_class
    }
    pub fn get_caste_tags(&self) -> HashMap<String, Vec<tags::CasteTag>> {
        let mut tags: HashMap<String, Vec<tags::CasteTag>> = HashMap::new();
        for self_caste in &self.castes {
            tags.insert(String::from(&self_caste.name), Vec::clone(&self_caste.tags));
        }
        tags
    }
}
