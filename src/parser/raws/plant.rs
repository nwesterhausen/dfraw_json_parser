use crate::parser::raws::{
    info::DFInfoFile,
    names::Name,
    tags::{self},
};
use crate::parser::reader::RawObjectKind;
use serde::{Deserialize, Serialize};
use slug::slugify;

#[derive(Debug)]
pub struct DFPlant {
    // Common Raw file Things
    identifier: String,
    parent_raw: String,
    dfraw_identifier: String,
    dfraw_version: String,
    dfraw_found_in: String,
    dfraw_display: String,
    raw_type: RawObjectKind,
    pub tags: Vec<tags::PlantTag>,

    // Basic Tokens
    pub name: Name,
    pub pref_string: Vec<String>,
    pub value: u32,

    // Environment Tokens
    pub underground_depth: [u32; 2],
    pub frequency: u32,
    pub cluster_size: u32,
    pub biomes: Vec<String>,
    // pub growth: DFPlantGrowth,
    // pub materials: Vec<DFMaterialTemplate>,
    // pub seed: DFPlantSeed,
    // Sub Tags
    pub materials_vec: Vec<SimpleMaterial>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DFPlantGrowth {
    pub name: Name,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DFPlantSeed {
    pub name: Name,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SimpleMaterialType {
    None,
    Oil,
    Soap,
    DrinkPlant,
    DrinkFruit,
    Powder,
    Leaf,
    Flower,
    Seed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleMaterial {
    pub material_type: SimpleMaterialType,
    pub name: Name,
    pub material_value: u32,
    pub tags: Vec<tags::MaterialTag>,
    pub state_color: String,
}

impl DFPlant {
    pub fn new(raw: &str, id: &str, info_text: &DFInfoFile) -> Self {
        Self {
            identifier: String::from(id),
            parent_raw: String::from(raw),
            dfraw_identifier: String::from(info_text.get_identifier()),
            dfraw_version: String::from(info_text.displayed_version.as_str()),
            dfraw_found_in: String::from(info_text.get_sourced_directory()),
            dfraw_display: format!("{} v{}", info_text.name, info_text.displayed_version),
            raw_type: RawObjectKind::Plant,
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

            // Simple materials
            materials_vec: Vec::new(),
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
            "PLANT",
            slugify(self.get_identifier())
        )
    }
    pub fn get_general_name(&self) -> String {
        self.name.to_string_vec()[0].to_string()
    }
}

impl SimpleMaterial {
    pub fn empty() -> Self {
        Self {
            material_type: SimpleMaterialType::None,
            material_value: 0,
            name: Name::new(""),
            state_color: String::new(),
            tags: Vec::new(),
        }
    }
    pub fn new(template: &str) -> Self {
        let Some(template_type) = template.split(":").next() else {
            log::warn!("Unable to handle template '{}'", template);
            return SimpleMaterial::empty();
        };

        let material_type = match template_type {
            "OIL" => SimpleMaterialType::Oil,
            "SOAP" => SimpleMaterialType::Soap,
            "MILL" => SimpleMaterialType::Powder,
            "LEAF" => SimpleMaterialType::Leaf,
            "SEED" => SimpleMaterialType::Seed,
            "DRINK" => SimpleMaterialType::DrinkPlant,
            _ => SimpleMaterialType::None,
        };

        Self {
            material_type: material_type,
            material_value: 0,
            name: Name::new(""),
            state_color: String::new(),
            tags: Vec::new(),
        }
    }
}
