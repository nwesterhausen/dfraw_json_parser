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
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
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
            if let Some(s) = material_type.split(':').next() {
                template_type = s;
            } else {
                log::warn!("Unable to handle template '{}'", material_type);
                return SimpleMaterial::empty();
            }
        }

        let material_type = SimpleMaterialType::from_template_tag(template_type);

        if material_type.eq(&SimpleMaterialType::None) {
            log::debug!("Un-matched material type: '{}'", template_type);
        }

        let mut simple_material = Self::empty();
        simple_material.material_type = material_type;

        simple_material.complete_initialization_from_template(template);

        simple_material
    }
    #[allow(clippy::too_many_lines)]
    fn complete_initialization_from_template(&mut self, template: &str) {
        match template {
            "PLANT_ALCOHOL_TEMPLATE" => {
                self.tags.push(tags::MaterialTag::AlcoholPlant);

                self.syndromes.push(String::from("inebriation"));

                self.temperatures.specific_heat = 2440;

                self.state_color = StateName::from("brown", "brown", "brown");
                self.state_name = StateName::from("alcohol", "frozen alcohol", "boiling alcohol");
                self.state_adj = StateName::from("alcohol", "frozen alcohol", "boiling alcohol");
            }
            "PLANT_POWDER_TEMPLATE" => {
                self.tags.push(tags::MaterialTag::PowderMiscPlant);

                self.temperatures.specific_heat = 800;

                self.state_color = StateName::from("white", "black", "black");
                self.state_name = StateName::from("plant powder", "none", "none");
                self.state_adj = StateName::from("plant powder", "none", "none");
            }
            "PLANT_EXTRACT_TEMPLATE" => {
                self.tags.push(tags::MaterialTag::LiquidMiscPlant);
                self.tags.push(tags::MaterialTag::Rots);

                self.temperatures.specific_heat = 4181;

                self.state_color = StateName::from("white", "white", "white");
                self.state_name = StateName::from("frozen extract", "extract", "boiling extract");
                self.state_adj = StateName::from("frozen extract", "extract", "boiling extract");
            }
            "PLANT_OIL_TEMPLATE" => {
                self.tags.push(tags::MaterialTag::LiquidMiscPlant);

                self.temperatures.specific_heat = 1820;

                self.state_color = StateName::from("yellow", "yellow", "yellow");
                self.state_name = StateName::from(
                    "frozen vegetable oil",
                    "vegetable oil",
                    "boiling vegetable oil",
                );
                self.state_adj = StateName::from(
                    "frozen vegetable oil",
                    "vegetable oil",
                    "boiling vegetable oil",
                );
            }
            "PLANT_SOAP_TEMPLATE" => {
                self.tags.push(tags::MaterialTag::Soap);

                self.temperatures.specific_heat = 800;

                self.state_color = StateName::from("cream", "cream", "cream");
                self.state_name = StateName::from("soap", "melted soap", "n/a");
                self.state_adj = StateName::from("soap", "melted soap", "n/a");
            }
            "SEED_TEMPLATE" => {
                self.tags.push(tags::MaterialTag::SeedMaterial);
                self.tags.push(tags::MaterialTag::DoNotCleanGlob);

                self.temperatures.specific_heat = 800;

                self.state_color = StateName::from("brown", "brown", "brown");
                self.state_name = StateName::from("seed", "none", "none");
                self.state_adj = StateName::from("seed", "none", "none");
            }
            "LEAF_TEMPLATE" => {
                self.tags.push(tags::MaterialTag::Rots);

                self.temperatures.specific_heat = 800;

                self.state_color = StateName::from("green", "black", "black");
                self.state_name = StateName::from("leaf", "none", "none");
                self.state_adj = StateName::from("leaf", "none", "none");
            }
            "FRUIT_TEMPLATE" => {
                self.tags.push(tags::MaterialTag::Rots);

                self.temperatures.specific_heat = 800;

                self.state_color = StateName::from("green", "black", "black");
                self.state_name = StateName::from("fruit", "none", "none");
                self.state_adj = StateName::from("fruit", "none", "none");
            }
            "MUSHROOM_TEMPLATE" => {
                self.tags.push(tags::MaterialTag::Rots);

                self.temperatures.specific_heat = 800;

                self.state_color = StateName::from("green", "black", "black");
                self.state_name = StateName::from("mushroom", "none", "none");
                self.state_adj = StateName::from("mushroom", "none", "none");
            }
            "FLOWER_TEMPLATE" => {
                self.tags.push(tags::MaterialTag::Rots);

                self.temperatures.specific_heat = 800;

                self.state_color = StateName::from("cream", "black", "black");
                self.state_name = StateName::from("flower", "none", "none");
                self.state_adj = StateName::from("flower", "none", "none");
            }
            "THREAD_PLANT_TEMPLATE" => {
                self.tags.push(tags::MaterialTag::ThreadPlant);
                self.tags.push(tags::MaterialTag::ItemsSoft);

                self.temperatures.specific_heat = 420;

                self.state_color = StateName::from("gray", "black", "black");
                self.state_name = StateName::from("fiber", "none", "none");
                self.state_adj = StateName::from("fiber", "none", "none");
            }
            "STRUCTURAL_PLANT_TEMPLATE" => {
                self.tags.push(tags::MaterialTag::Rots);
                self.tags.push(tags::MaterialTag::StructuralPlantMaterial);

                self.temperatures.specific_heat = 3000;

                self.state_color = StateName::from("brown", "brown", "brown");
                self.state_name = StateName::from("plant", "none", "none");
                self.state_adj = StateName::from("plant", "none", "none");
            }
            "WOOD_TEMPLATE" => {
                self.tags.push(tags::MaterialTag::ItemsHard);
                self.tags.push(tags::MaterialTag::ItemsWeapon);
                self.tags.push(tags::MaterialTag::ItemsWeaponRanged);
                self.tags.push(tags::MaterialTag::ItemsAmmo);
                self.tags.push(tags::MaterialTag::ItemsArmor);
                self.tags.push(tags::MaterialTag::ItemsSiegeEngine);
                self.tags.push(tags::MaterialTag::Wood);

                self.temperatures.specific_heat = 420;
                self.temperatures.heat_damage_point = 10250;
                self.temperatures.cold_damage_point = 9900;

                self.state_color = StateName::from("brown", "brown", "brown");
                self.state_name = StateName::from("wood", "n/a", "n/a");
                self.state_adj = StateName::from("wooden", "n/a", "n/a");
            }
            "FEATHER_TEMPLATE" => {
                self.tags.push(tags::MaterialTag::ImpliesAnimalKill);
                self.tags.push(tags::MaterialTag::Feather);

                self.temperatures.specific_heat = 420;

                self.state_color = StateName::from("gray", "gray", "gray");
                self.state_name = StateName::from("feather", "n/a", "n/a");
                self.state_adj = StateName::from("feather", "n/a", "n/a");
            }
            "METAL_TEMPLATE" => {
                self.tags.push(tags::MaterialTag::IsMetal);

                self.temperatures.specific_heat = 450;
                self.temperatures.melting_point = 12768;
                self.temperatures.boiling_point = 15150;

                self.state_color = StateName::from("gray", "red", "red");
                self.state_name = StateName::from("metal", "molten metal", "boiling metal");
                self.state_adj = StateName::from("metal", "molten metal", "boiling metal");
            }
            "STONE_TEMPLATE" => {
                self.tags.push(tags::MaterialTag::ItemsHard);
                self.tags.push(tags::MaterialTag::ItemsQuern);

                self.temperatures.specific_heat = 800;
                self.temperatures.melting_point = 11500;
                self.temperatures.boiling_point = 14000;

                self.state_color = StateName::from("gray", "orange", "orange");
                self.state_name = StateName::from("metal", "magma", "boiling magma");
                self.state_adj = StateName::from("metal", "magma", "boiling magma");
            }
            _ => (),
        }
    }

    /// For reactions and custom buildings using the `[MAGMA_BUILD_SAFE]` token, only a material which is solid and
    /// stable at the temperature 12000 °U (i.e. `MELTING_POINT`/`BOILING_POINT`/`IGNITE_POINT`/`HEATDAM_POINT` greater
    /// than 12000 and `COLDDAM_POINT` less than 12000) is considered magma-safe.
    pub fn is_magma_safe(&self) -> bool {
        // Check melting point
        if self.temperatures.melting_point > 0 && self.temperatures.melting_point < 12_000 {
            return false;
        }
        // Check boiling point
        if self.temperatures.boiling_point > 0 && self.temperatures.boiling_point < 12_000 {
            return false;
        }
        // Check ignition point
        if self.temperatures.ignition_point > 0 && self.temperatures.ignition_point < 12_000 {
            return false;
        }
        // Check heat damage point
        if self.temperatures.heat_damage_point > 0 && self.temperatures.heat_damage_point < 12_000 {
            return false;
        }
        // Check cold damage point
        if self.temperatures.cold_damage_point > 12_000 {
            return false;
        }

        true
    }
}

/// Takes a material template (as a string) and returns a vector of `MaterialTags`
///
/// Arguments:
///
/// * `template_type`: The material template to match
///
/// Returns:
///
/// A vector of `MaterialTag` enum values.
pub fn tags_from_template(template_type: &str) -> Vec<tags::MaterialTag> {
    let mut template_tags = Vec::new();

    if "PLANT_ALCOHOL_TEMPLATE" == template_type {
        template_tags.push(tags::MaterialTag::AlcoholPlant);
    }

    template_tags
}

impl SimpleMaterialType {
    /// Given a string and returns a `SimpleMaterialType`
    ///
    /// Arguments:
    ///
    /// * `template_type`: &str
    ///
    /// Returns:
    ///
    /// A `SimpleMaterialType` for given `template_type` or `SimpleMaterialType::None`.
    pub fn from_template_tag(template_type: &str) -> Self {
        match template_type {
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
        }
    }
}
