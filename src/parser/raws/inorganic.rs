use crate::parser::raws::{
    info::DFInfoFile,
    tags::{self},
};
use crate::parser::reader::RawObjectKind;
use slug::slugify;

use super::material;
use super::{environment, roll_chance};

#[derive(Debug)]
pub struct DFInorganic {
    // Common Raw file Things
    identifier: String,
    parent_raw: String,
    dfraw_identifier: String,
    dfraw_version: String,
    dfraw_found_in: String,
    dfraw_display: String,
    raw_type: RawObjectKind,
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
            identifier: String::from(id),
            parent_raw: String::from(raw),
            dfraw_identifier: String::from(info_text.get_identifier()),
            dfraw_version: String::from(info_text.displayed_version.as_str()),
            dfraw_found_in: String::from(info_text.get_sourced_directory()),
            dfraw_display: format!("{} v{}", info_text.name, info_text.displayed_version),
            raw_type: RawObjectKind::Inorganic,
            // Boolean Flags
            tags: Vec::new(),

            material: material::SimpleMaterial::empty(),
            environments: Vec::new(),
            environments_specific: Vec::new(),
            metal_ores: Vec::new(),
            thread_metals: Vec::new(),
        }
    }

    pub fn get_identifier(&self) -> String {
        String::from(&self.identifier)
    }
    pub fn get_raw_module(&self) -> String {
        String::from(&self.dfraw_identifier)
    }
    pub fn get_raw_module_version(&self) -> String {
        String::from(&self.dfraw_version)
    }
    pub fn get_dfraw_found_in(&self) -> String {
        String::from(&self.dfraw_found_in)
    }
    pub fn get_dfraw_display(&self) -> String {
        String::from(&self.dfraw_display)
    }
    pub fn get_parent_raw(&self) -> String {
        String::from(&self.parent_raw)
    }
    pub fn get_raw_type(&self) -> String {
        format!("{:?}", self.raw_type)
    }
    pub fn get_object_id(&self) -> String {
        format!(
            "{}-{}-{}",
            self.get_parent_raw(),
            "INORGANIC",
            slugify(self.get_identifier())
        )
    }
    pub fn get_general_name(&self) -> String {
        String::from(self.material.state_name.get_solid())
    }
}
