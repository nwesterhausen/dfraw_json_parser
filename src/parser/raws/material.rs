use crate::parser::raws::tags::{self};
use crate::parser::raws::{names::StateName, temperatures::Temperatures};
use serde::{Deserialize, Serialize};

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
    Metal,
    Stone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleMaterial {
    #[serde(rename = "type")]
    pub material_type: SimpleMaterialType,
    #[serde(rename = "names")]
    pub state_name: StateName,
    #[serde(rename = "adjectives")]
    pub state_adj: StateName,
    #[serde(rename = "value")]
    pub material_value: u32,
    pub tags: Vec<tags::MaterialTag>,
    #[serde(rename = "colors")]
    pub state_color: StateName,
    pub temperatures: Temperatures,
    pub syndromes: Vec<String>,
    pub reaction_classes: Vec<String>,
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
            temperatures: Temperatures::new(),
            syndromes: Vec::new(),
            reaction_classes: Vec::new(),
        }
    }
    pub fn new(material_type: &str, template: &str) -> Self {
        let mut template_type = template;
        if !template_type.eq("METAL_TEMPLATE") || !template_type.eq("STONE_TEMPLATE") {
            match material_type.split(":").next() {
                Some(s) => template_type = s,
                _ => {
                    log::warn!("Unable to handle template '{}'", material_type);
                    return SimpleMaterial::empty();
                }
            }
        }

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
            "METAL_TEMPLATE" => SimpleMaterialType::Metal,
            "STONE_TEMPLATE" => SimpleMaterialType::Stone,
            _ => SimpleMaterialType::None,
        };

        if material_type.eq(&SimpleMaterialType::None) {
            log::debug!("Un-matched material type: '{}'", template_type);
        }

        let mut tags_vec: Vec<tags::MaterialTag> = Vec::new();
        let mut syndrome_vec: Vec<String> = Vec::new();
        let reaction_classes_vec: Vec<String> = Vec::new();
        let mut temperatures: Temperatures = Temperatures::new();

        match template {
            "PLANT_ALCOHOL_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::AlcoholPlant);

                syndrome_vec.push(String::from("inebriation"));

                temperatures.specific_heat = 2440;

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("brown", "brown", "brown"),
                    state_name: StateName::from("alcohol", "frozen alcohol", "boiling alcohol"),
                    state_adj: StateName::from("alcohol", "frozen alcohol", "boiling alcohol"),
                    tags: tags_vec,
                    temperatures: temperatures,
                    syndromes: syndrome_vec,
                    reaction_classes: reaction_classes_vec,
                };
            }
            "PLANT_POWDER_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::PowderMiscPlant);

                temperatures.specific_heat = 800;

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("white", "black", "black"),
                    state_name: StateName::from("plant powder", "none", "none"),
                    state_adj: StateName::from("plant powder", "none", "none"),
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    temperatures: temperatures,
                    reaction_classes: reaction_classes_vec,
                };
            }
            "PLANT_EXTRACT_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::LiquidMiscPlant);
                tags_vec.push(tags::MaterialTag::Rots);

                temperatures.specific_heat = 4181;

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("white", "white", "white"),
                    state_name: StateName::from("frozen extract", "extract", "boiling extract"),
                    state_adj: StateName::from("frozen extract", "extract", "boiling extract"),
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    temperatures: temperatures,
                    reaction_classes: reaction_classes_vec,
                };
            }
            "PLANT_OIL_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::LiquidMiscPlant);

                temperatures.specific_heat = 1820;

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
                    temperatures: temperatures,
                    syndromes: syndrome_vec,
                    reaction_classes: reaction_classes_vec,
                };
            }
            "PLANT_SOAP_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::Soap);

                temperatures.specific_heat = 800;

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("cream", "cream", "cream"),
                    state_name: StateName::from("soap", "melted soap", "n/a"),
                    state_adj: StateName::from("soap", "melted soap", "n/a"),
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    temperatures: temperatures,
                    reaction_classes: reaction_classes_vec,
                };
            }
            "SEED_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::SeedMaterial);
                tags_vec.push(tags::MaterialTag::DoNotCleanGlob);

                temperatures.specific_heat = 800;

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("brown", "brown", "brown"),
                    state_name: StateName::from("seed", "none", "none"),
                    state_adj: StateName::from("seed", "none", "none"),
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    temperatures: temperatures,
                    reaction_classes: reaction_classes_vec,
                };
            }
            "LEAF_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::Rots);

                temperatures.specific_heat = 800;

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("green", "black", "black"),
                    state_name: StateName::from("leaf", "none", "none"),
                    state_adj: StateName::from("leaf", "none", "none"),
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    temperatures: temperatures,
                    reaction_classes: reaction_classes_vec,
                };
            }
            "FRUIT_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::Rots);

                temperatures.specific_heat = 800;

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("green", "black", "black"),
                    state_name: StateName::from("fruit", "none", "none"),
                    state_adj: StateName::from("fruit", "none", "none"),
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    temperatures: temperatures,
                    reaction_classes: reaction_classes_vec,
                };
            }
            "MUSHROOM_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::Rots);

                temperatures.specific_heat = 800;

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("green", "black", "black"),
                    state_name: StateName::from("mushroom", "none", "none"),
                    state_adj: StateName::from("mushroom", "none", "none"),
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    temperatures: temperatures,
                    reaction_classes: reaction_classes_vec,
                };
            }
            "FLOWER_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::Rots);

                temperatures.specific_heat = 800;

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("cream", "black", "black"),
                    state_name: StateName::from("flower", "none", "none"),
                    state_adj: StateName::from("flower", "none", "none"),
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    temperatures: temperatures,
                    reaction_classes: reaction_classes_vec,
                };
            }
            "THREAD_PLANT_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::ThreadPlant);
                tags_vec.push(tags::MaterialTag::ItemsSoft);

                temperatures.specific_heat = 420;

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("gray", "black", "black"),
                    state_name: StateName::from("fiber", "none", "none"),
                    state_adj: StateName::from("fiber", "none", "none"),
                    tags: tags_vec,
                    temperatures: temperatures,
                    syndromes: syndrome_vec,
                    reaction_classes: reaction_classes_vec,
                };
            }
            "STRUCTURAL_PLANT_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::Rots);
                tags_vec.push(tags::MaterialTag::StructuralPlantMaterial);

                temperatures.specific_heat = 3000;

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("brown", "brown", "brown"),
                    state_name: StateName::from("plant", "none", "none"),
                    state_adj: StateName::from("plant", "none", "none"),
                    tags: tags_vec,
                    temperatures: temperatures,
                    syndromes: syndrome_vec,
                    reaction_classes: reaction_classes_vec,
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

                temperatures.specific_heat = 420;
                temperatures.heat_damage_point = 10250;
                temperatures.cold_damage_point = 9900;

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("brown", "brown", "brown"),
                    state_name: StateName::from("wood", "n/a", "n/a"),
                    state_adj: StateName::from("wooden", "n/a", "n/a"),
                    tags: tags_vec,
                    temperatures: temperatures,
                    syndromes: syndrome_vec,
                    reaction_classes: reaction_classes_vec,
                };
            }
            "FEATHER_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::ImpliesAnimalKill);
                tags_vec.push(tags::MaterialTag::Feather);

                temperatures.specific_heat = 420;

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("gray", "gray", "gray"),
                    state_name: StateName::from("feather", "n/a", "n/a"),
                    state_adj: StateName::from("feather", "n/a", "n/a"),
                    tags: tags_vec,
                    temperatures: temperatures,
                    syndromes: syndrome_vec,
                    reaction_classes: reaction_classes_vec,
                };
            }
            "METAL_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::IsMetal);

                temperatures.specific_heat = 450;
                temperatures.melting_point = 12768;
                temperatures.boiling_point = 15150;

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("gray", "red", "red"),
                    state_name: StateName::from("metal", "molten metal", "boiling metal"),
                    state_adj: StateName::from("metal", "molten metal", "boiling metal"),
                    temperatures: temperatures,
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    reaction_classes: reaction_classes_vec,
                };
            }
            "STONE_TEMPLATE" => {
                tags_vec.push(tags::MaterialTag::ItemsHard);
                tags_vec.push(tags::MaterialTag::ItemsQuern);

                temperatures.specific_heat = 800;
                temperatures.melting_point = 11500;
                temperatures.boiling_point = 14000;

                return Self {
                    material_type: material_type,
                    material_value: 1,
                    state_color: StateName::from("gray", "orange", "orange"),
                    state_name: StateName::from("metal", "magma", "boiling magma"),
                    state_adj: StateName::from("metal", "magma", "boiling magma"),
                    temperatures: temperatures,
                    tags: tags_vec,
                    syndromes: syndrome_vec,
                    reaction_classes: reaction_classes_vec,
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
                temperatures: temperatures,
                reaction_classes: reaction_classes_vec,
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
