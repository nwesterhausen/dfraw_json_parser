use std::collections::HashMap;

use crate::parser::raws::{
    info::DFInfoFile,
    names::Name,
    tags::{self},
};
use crate::parser::reader::RawObjectKind;
use serde::{Deserialize, Serialize};
use slug::slugify;

use super::names::{SingPlurName, StateName};

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
    pub growth_duration: u32,
    pub growth_names: HashMap<PlantGrowth, SingPlurName>,

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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PlantGrowth {
    None,
    Leaves,
    Spathes,
    Fruit,
    Flowers,
    Nut,
    SeedCatkins,
    PollenCatkins,
    Cone,
    SeedCone,
    PollenCone,
    Feathers,
    Eggs,
    Pod,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DFPlantSeed {
    pub name: Name,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
    Structural,
    Mushroom,
    Wood,
    Fruit,
    Extract,
    Thread,
    Egg,
    Feather,
    Nut,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleMaterial {
    pub material_type: SimpleMaterialType,
    pub state_name: StateName,
    pub state_adj: StateName,
    pub material_value: u32,
    pub tags: Vec<tags::MaterialTag>,
    pub state_color: StateName,
    pub specific_heat: u32,
    pub syndromes: Vec<String>,
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
            growth_duration: 0,
            growth_names: HashMap::new(),

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
            state_name: StateName::new(),
            state_adj: StateName::new(),
            state_color: StateName::new(),
            tags: Vec::new(),
            specific_heat: 0,
            syndromes: Vec::new(),
        }
    }
    pub fn new(material_type: &str, template: &str) -> Self {
        let Some(template_type) = material_type.split(":").next() else {
            log::warn!("Unable to handle template '{}'", material_type);
            return SimpleMaterial::empty();
        };

        let material_type = match template_type {
            "OIL" => SimpleMaterialType::Oil,
            "SOAP" => SimpleMaterialType::Soap,
            "MILL" => SimpleMaterialType::Powder,
            "LEAF" => SimpleMaterialType::Leaf,
            "SEED" => SimpleMaterialType::Seed,
            "DRINK" => SimpleMaterialType::DrinkPlant,
            "FLOWER" => SimpleMaterialType::Flower,
            "STRUCTURAL" => SimpleMaterialType::Structural,
            "MUSHROOM" => SimpleMaterialType::Mushroom,
            "WOOD" => SimpleMaterialType::Wood,
            "FRUIT" => SimpleMaterialType::Fruit,
            "EXTRACT" => SimpleMaterialType::Extract,
            "THREAD" => SimpleMaterialType::Thread,
            "EGG" => SimpleMaterialType::Egg,
            "FEATHER" => SimpleMaterialType::Feather,
            "NUT" => SimpleMaterialType::Nut,
            _ => SimpleMaterialType::None,
        };

        if material_type.eq(&SimpleMaterialType::None) {
            log::debug!("Un-matched material type: '{}'", template_type);
        }

        let mut tags_vec: Vec<tags::MaterialTag> = Vec::new();
        let mut syndrome_vec: Vec<String> = Vec::new();

        match template {
            "PLANT_ALCOHOL_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::AlcoholPlant);

                syndrome_vec.push(String::from("inebriation"));

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("brown", "brown", "brown"),
                    state_name: StateName::from("alcohol", "frozen alcohol", "boiling alcohol"),
                    state_adj: StateName::from("alcohol", "frozen alcohol", "boiling alcohol"),
                    tags: tags_vec,
                    specific_heat: 2440,
                    syndromes: syndrome_vec,
                };
            }
            "PLANT_POWDER_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::PowderMiscPlant);

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("white", "black", "black"),
                    state_name: StateName::from("plant powder", "none", "none"),
                    state_adj: StateName::from("plant powder", "none", "none"),
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    specific_heat: 800,
                };
            }
            "PLANT_EXTRACT_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::LiquidMiscPlant);
                tags_vec.push(tags::MaterialTag::Rots);

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("white", "white", "white"),
                    state_name: StateName::from("frozen extract", "extract", "boiling extract"),
                    state_adj: StateName::from("frozen extract", "extract", "boiling extract"),
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    specific_heat: 4181,
                };
            }
            "PLANT_OIL_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::LiquidMiscPlant);

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("yellow", "yellow", "yellow"),
                    state_name: StateName::from(
                        "frozen vegetable oil",
                        "vegetable oil",
                        "boiling vegetable oil",
                    ),
                    state_adj: StateName::from(
                        "frozen vegetable oil",
                        "vegetable oil",
                        "boiling vegetable oil",
                    ),
                    tags: tags_vec,
                    specific_heat: 1820,
                    syndromes: syndrome_vec,
                };
            }
            "PLANT_SOAP_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::Soap);

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("cream", "cream", "cream"),
                    state_name: StateName::from("soap", "melted soap", "n/a"),
                    state_adj: StateName::from("soap", "melted soap", "n/a"),
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    specific_heat: 800,
                };
            }
            "SEED_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::SeedMaterial);
                tags_vec.push(tags::MaterialTag::DoNotCleanGlob);

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("brown", "brown", "brown"),
                    state_name: StateName::from("seed", "none", "none"),
                    state_adj: StateName::from("seed", "none", "none"),
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    specific_heat: 800,
                };
            }
            "LEAF_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::Rots);

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("green", "black", "black"),
                    state_name: StateName::from("leaf", "none", "none"),
                    state_adj: StateName::from("leaf", "none", "none"),
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    specific_heat: 800,
                };
            }
            "FRUIT_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::Rots);

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("green", "black", "black"),
                    state_name: StateName::from("fruit", "none", "none"),
                    state_adj: StateName::from("fruit", "none", "none"),
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    specific_heat: 800,
                };
            }
            "MUSHROOM_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::Rots);

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("green", "black", "black"),
                    state_name: StateName::from("mushroom", "none", "none"),
                    state_adj: StateName::from("mushroom", "none", "none"),
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    specific_heat: 800,
                };
            }
            "FLOWER_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::Rots);

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("cream", "black", "black"),
                    state_name: StateName::from("flower", "none", "none"),
                    state_adj: StateName::from("flower", "none", "none"),
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    specific_heat: 800,
                };
            }
            "THREAD_PLANT_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::ThreadPlant);
                tags_vec.push(tags::MaterialTag::ItemsSoft);

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("gray", "black", "black"),
                    state_name: StateName::from("fiber", "none", "none"),
                    state_adj: StateName::from("fiber", "none", "none"),
                    tags: tags_vec,
                    specific_heat: 420,
                    syndromes: syndrome_vec,
                };
            }
            "STRUCTURAL_PLANT_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::Rots);
                tags_vec.push(tags::MaterialTag::StructuralPlantMaterial);

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("brown", "brown", "brown"),
                    state_name: StateName::from("plant", "none", "none"),
                    state_adj: StateName::from("plant", "none", "none"),
                    tags: tags_vec,
                    specific_heat: 3000,
                    syndromes: syndrome_vec,
                };
            }
            "WOOD_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::ItemsHard);
                tags_vec.push(tags::MaterialTag::ItemsWeapon);
                tags_vec.push(tags::MaterialTag::ItemsWeaponRanged);
                tags_vec.push(tags::MaterialTag::ItemsAmmo);
                tags_vec.push(tags::MaterialTag::ItemsArmor);
                tags_vec.push(tags::MaterialTag::ItemsSiegeEngine);
                tags_vec.push(tags::MaterialTag::Wood);

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("brown", "brown", "brown"),
                    state_name: StateName::from("wood", "n/a", "n/a"),
                    state_adj: StateName::from("wooden", "n/a", "n/a"),
                    tags: tags_vec,
                    specific_heat: 420,
                    syndromes: syndrome_vec,
                };
            }
            "FEATHER_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::ImpliesAnimalKill);
                tags_vec.push(tags::MaterialTag::Feather);

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("gray", "gray", "gray"),
                    state_name: StateName::from("feather", "n/a", "n/a"),
                    state_adj: StateName::from("feather", "n/a", "n/a"),
                    tags: tags_vec,
                    specific_heat: 420,
                    syndromes: syndrome_vec,
                };
            }
            _ => Self {
                material_type: material_type,
                material_value: 0,
                state_name: StateName::new(),
                state_adj: StateName::new(),
                state_color: StateName::new(),
                tags: tags_vec,
                syndromes: syndrome_vec,
                specific_heat: 800,
            },
        }
    }
}

pub fn material_tags_from_template(template_type: &str) -> Vec<tags::MaterialTag> {
    let mut template_tags = Vec::new();

    match template_type {
        "PLANT_ALCOHOL_TEMPLATE" => {
            template_tags.push(tags::MaterialTag::AlcoholPlant);
        }
        _ => (),
    }

    template_tags
}
