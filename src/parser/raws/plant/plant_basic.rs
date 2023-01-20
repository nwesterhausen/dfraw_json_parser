use std::collections::HashMap;

use crate::parser::raws::{info_txt::DFInfoFile, names::Name, DFRawCommon, RawObjectKind};

impl super::DFPlant {
    pub fn new(raw: &str, id: &str, info_text: &DFInfoFile) -> Self {
        Self {
            raw_header: DFRawCommon::from(id, raw, info_text, RawObjectKind::Plant),
            // Boolean Flags
            tags: Vec::new(),

            // integers
            frequency: 50, //Defaults to 50 if not specified
            cluster_size: 0,

            biomes: Vec::new(),
            name: Name::new(""),

            pref_string: Vec::new(),
            value: 0,
            underground_depth: [0, 0],
            growth_duration: 0,
            growth_names: HashMap::new(),

            // Simple materials
            materials_vec: Vec::new(),

            // Simple reactions..
            reactions: Vec::new(),
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
        if !self.get_raw_header().overwrites_raw.is_empty() {
            return format!("Overwrite {}", self.get_raw_header().overwrites_raw);
        }
        self.name.to_string_vec()[0].to_string()
    }
}
