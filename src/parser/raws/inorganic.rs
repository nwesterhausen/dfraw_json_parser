use crate::parser::raws::{
    info::DFInfoFile,
    tags::{self},
};

use super::{environment, roll_chance};
use super::{material, DFRawCommon};

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct DFInorganic {
    // Common Raw file Things
    raw_header: DFRawCommon,
    pub tags: Vec<tags::InorganicTag>,

    // Basic Tokens
    pub material: material::SimpleMaterial,
    pub environments: Vec<environment::Environment>,
    pub environments_specific: Vec<environment::Environment>,

    pub metal_ores: Vec<roll_chance::RollChance>,
    pub thread_metals: Vec<roll_chance::RollChance>,
}

impl DFInorganic {
    pub fn new(raw: &str, id: &str, info_text: &DFInfoFile) -> Self {
        Self {
            raw_header: DFRawCommon::from(id, raw, info_text, super::RawObjectKind::Inorganic),
            // Boolean Flags
            tags: Vec::new(),

            material: material::SimpleMaterial::empty(),
            environments: Vec::new(),
            environments_specific: Vec::new(),
            metal_ores: Vec::new(),
            thread_metals: Vec::new(),
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
        if self.material.state_name.get_solid().is_empty() {
            return titlecase::titlecase(&self.get_raw_header().identifier);
        }
        String::from(self.material.state_name.get_solid())
    }
    pub fn is_magma_safe(&self) -> bool {
        self.material.is_magma_safe()
    }
}
