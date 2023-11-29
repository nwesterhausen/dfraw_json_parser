use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::parser::{
    clean_search_vec,
    creature_effect::TOKEN_MAP as CREATURE_EFFECT_TOKENS,
    material::phf_table::MATERIAL_PROPERTY_TOKENS,
    serializer_helper,
    syndrome::{Syndrome, TOKEN_MAP as SYNDROME_TOKEN_MAP},
    Color, MaterialMechanics, Searchable, StateName, Temperatures, Tile,
};

use super::{
    phf_table::{FUEL_TYPE_TOKENS, MATERIAL_TYPE_TOKENS, MATERIAL_USAGE_TOKENS},
    tokens::{FuelType, MaterialProperty, MaterialType, MaterialUsage},
};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    /// The type of the material is also the trigger to start tracking a material
    #[serde(skip_serializing_if = "MaterialType::is_default")]
    material_type: MaterialType,
    /// The material might have a name, but its more likely that there is only an identifier to
    /// refer to another creature/plant/reaction, which are listed elsewhere.
    /// If there is no name provided, then it is a special hardcoded case, e.g. magma or green glass.
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,
    /// For the coal tag, it specifies the type of fuel that can be used. It will never be None.
    #[serde(skip_serializing_if = "FuelType::is_default")]
    fuel_type: FuelType,
    /// Linked creature identifier (and then material_name might be "skin", like for "CREATURE_MAT:DWARF:SKIN")
    #[serde(skip_serializing_if = "String::is_empty")]
    creature_identifier: String,
    /// Linked plant identifier (and then material_name might be "leaf", like for "PLANT_MAT:BUSH_QUARRY:LEAF")
    #[serde(skip_serializing_if = "String::is_empty")]
    plant_identifier: String,
    /// If a material is defined within a creature itself, it will use LOCAL_CREATURE_MAT tag, which implies
    /// that the material is only used by that creature. This is also true for plants and LOCAL_PLANT_MAT.
    #[serde(skip_serializing_if = "serializer_helper::is_false")]
    is_local_material: bool,
    /// Within a reaction, there can be special material definitions. Todo: Figure this out.
    #[serde(skip_serializing_if = "String::is_empty")]
    reagent_identifier: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    reaction_product_identifier: String,
    /// If material is defined from a template, we need a way to refer to that
    #[serde(skip_serializing_if = "String::is_empty")]
    template_identifier: String,

    /// Usage tags
    #[serde(skip_serializing_if = "Vec::is_empty")]
    usage: Vec<MaterialUsage>,

    #[serde(skip_serializing_if = "serializer_helper::is_one")]
    value: u32,

    #[serde(skip_serializing_if = "Color::is_default")]
    color: Color,

    #[serde(skip_serializing_if = "StateName::is_empty")]
    state_names: StateName,

    #[serde(skip_serializing_if = "StateName::is_empty")]
    state_adjectives: StateName,

    #[serde(skip_serializing_if = "StateName::is_empty")]
    state_colors: StateName,

    #[serde(skip_serializing_if = "Temperatures::is_empty")]
    temperatures: Temperatures,

    /// Catch-all for remaining tags we identify but don't do anything with... yet.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    properties: Vec<String>,

    // Syndromes attached to materials..
    #[serde(skip_serializing_if = "Vec::is_empty")]
    syndromes: Vec<Syndrome>,
    // Material Mechanical Properties
    #[serde(skip_serializing_if = "MaterialMechanics::is_empty")]
    mechanical_properties: MaterialMechanics,
    // Technically, the material mechanics wouldn't apply to liquid or gaseous forms
    #[serde(skip_serializing_if = "serializer_helper::is_zero_i32")]
    liquid_density: i32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_i32")]
    molar_mass: i32,

    // Colors
    #[serde(skip_serializing_if = "Color::is_default")]
    build_color: Color,
    #[serde(skip_serializing_if = "Color::is_default")]
    display_color: Color,

    // Display
    #[serde(skip_serializing_if = "Tile::is_default")]
    tile: Tile,
    #[serde(skip_serializing_if = "String::is_empty")]
    item_symbol: String,
}

impl Material {
    pub fn new() -> Material {
        Material {
            value: 1,
            ..Material::default()
        }
    }
    /// This may not be correct. This should be for `\[USE_MATERIAL:XX:XX\]` but couldn't find an example for Plant.
    pub fn use_material_from_value(value: &str) -> Material {
        // Start defining a new material with a name and properties of another local material
        let mut split = value.split(':');

        let material_name = split.next().unwrap_or_default();
        let parent_material_name = split.next().unwrap_or_default();

        Material {
            name: String::from(material_name),
            template_identifier: String::from(parent_material_name),
            is_local_material: true,
            ..Material::new()
        }
    }
    pub fn use_material_template_from_value(value: &str) -> Material {
        // Start defining a new material with a name and properties of another local material
        let mut split = value.split(':');

        let material_name = split.next().unwrap_or_default();
        let template_material_name = split.next().unwrap_or_default();

        Material {
            name: String::from(material_name),
            template_identifier: String::from(template_material_name),
            is_local_material: true,
            ..Material::new()
        }
    }
    pub fn basic_material_from_value(value: &str) -> Material {
        Material::from_value(value)
    }
    pub fn from_value(value: &str) -> Material {
        // Value is a string like "CREATURE_MAT:DWARF:SKIN" or "INORGANIC" or "STONE:MARBLE" or "LOCAL_PLANT_MAT:LEAF"
        // It's possible that the number of parts to the value str is 1, 2, or 3.
        let mut split = value.split(':');
        let split_len = split.clone().count();

        // The first part is always the material type, so we can get that first.
        let material_type = split.next().unwrap_or_default();
        let Some(material_type) = MATERIAL_TYPE_TOKENS.get(material_type) else {
            warn!(
                "Material::from_value() was provided a value with an invalid material type: {}",
                value
            );
            return Material::new();
        };

        // If there is only one part, then it is a special hardcoded material, like magma or water.
        if split_len == 1 {
            return Material {
                material_type: material_type.clone(),
                ..Material::new()
            };
        }
        // If there are more than one parts, we can use a match and drill down further.
        // Use the phf_table to get the type of the material and then match from there.
        match material_type {
            MaterialType::Inorganic | MaterialType::Stone | MaterialType::Metal => {
                let material_name = split.next().unwrap_or_default();
                Material {
                    material_type: material_type.clone(),
                    name: String::from(material_name),
                    ..Material::new()
                }
            }
            MaterialType::Coal => {
                let material_key = split.next().unwrap_or_default();
                let Some(fuel_type) = FUEL_TYPE_TOKENS.get(material_key) else {
                    warn!(
                        "Material::from_value() was provided a value with an invalid fuel type: {}",
                        value
                    );
                    return Material {
                        material_type: material_type.clone(),
                        ..Material::new()
                    };
                };
                Material {
                    material_type: material_type.clone(),
                    fuel_type: fuel_type.clone(),
                    ..Material::new()
                }
            }
            MaterialType::LocalCreatureMaterial | MaterialType::LocalPlantMaterial => {
                let material_name = split.next().unwrap_or_default();
                Material {
                    material_type: material_type.clone(),
                    name: String::from(material_name),
                    is_local_material: true,
                    ..Material::new()
                }
            }
            MaterialType::CreatureMaterial => {
                let creature_identifier = split.next().unwrap_or_default();
                let material_name = split.next().unwrap_or_default();
                Material {
                    material_type: material_type.clone(),
                    creature_identifier: String::from(creature_identifier),
                    name: String::from(material_name),
                    ..Material::new()
                }
            }
            MaterialType::PlantMaterial => {
                let plant_identifier = split.next().unwrap_or_default();
                let material_name = split.next().unwrap_or_default();
                Material {
                    material_type: material_type.clone(),
                    plant_identifier: String::from(plant_identifier),
                    name: String::from(material_name),
                    ..Material::new()
                }
            }
            MaterialType::GetMaterialFromReagent => {
                let reagent_identifier = split.next().unwrap_or_default();
                let reaction_product_identifier = split.next().unwrap_or_default();
                Material {
                    material_type: material_type.clone(),
                    reagent_identifier: String::from(reagent_identifier),
                    reaction_product_identifier: String::from(reaction_product_identifier),
                    ..Material::new()
                }
            }
            _ => {
                warn!(
                    "Material::from_value() was provided a value with an invalid material type: {}",
                    value
                );
                Material::new()
            }
        }
    }
    #[allow(clippy::too_many_lines)]
    pub fn parse_tag(&mut self, key: &str, value: &str) {
        // Determine if the key is a Property or Usage tag
        if MATERIAL_PROPERTY_TOKENS.contains_key(key) {
            // Parse key as a property token, then pass the value to the property (or add a generic tag)
            let Some(tag) = MATERIAL_PROPERTY_TOKENS.get(key) else {
                warn!(
                    "Material::parse_tag() was provided a key with an invalid property token: {}",
                    key
                );
                return;
            };

            match tag {
                MaterialProperty::MaterialValue => {
                    self.value = value.parse::<u32>().unwrap_or(1);
                }
                MaterialProperty::StateNameAdjective => {
                    self.state_names.add_from_value(value);
                    self.state_adjectives.add_from_value(value);
                }
                // Names and Adjectives
                MaterialProperty::StateName => self.state_names.add_from_value(value),
                MaterialProperty::StateAdjective => self.state_adjectives.add_from_value(value),
                MaterialProperty::StateColor => self.state_colors.add_from_value(value),
                MaterialProperty::BasicColor => self.color = Color::from_value(value),
                // Temperatures
                MaterialProperty::SpecificHeat => self
                    .temperatures
                    .update_specific_heat(value.parse::<u32>().unwrap_or(0)),
                MaterialProperty::IgnitionPoint => self
                    .temperatures
                    .update_ignition_point(value.parse::<u32>().unwrap_or(0)),
                MaterialProperty::MeltingPoint => self
                    .temperatures
                    .update_melting_point(value.parse::<u32>().unwrap_or(0)),
                MaterialProperty::BoilingPoint => self
                    .temperatures
                    .update_boiling_point(value.parse::<u32>().unwrap_or(0)),
                MaterialProperty::HeatDamagePoint => self
                    .temperatures
                    .update_heat_damage_point(value.parse::<u32>().unwrap_or(0)),
                MaterialProperty::ColdDamagePoint => self
                    .temperatures
                    .update_cold_damage_point(value.parse::<u32>().unwrap_or(0)),
                MaterialProperty::MaterialFixedTemperature => self
                    .temperatures
                    .update_material_fixed_temperature(value.parse::<u32>().unwrap_or(0)),
                // Syndrome
                MaterialProperty::Syndrome => {
                    let syndrome = Syndrome::new();
                    self.syndromes.push(syndrome);
                }
                // Material Mechanics..
                MaterialProperty::ImpactYield
                | MaterialProperty::ImpactFracture
                | MaterialProperty::ImpactElasticity
                | MaterialProperty::CompressiveYield
                | MaterialProperty::CompressiveFracture
                | MaterialProperty::CompressiveElasticity
                | MaterialProperty::TensileYield
                | MaterialProperty::TensileFracture
                | MaterialProperty::TensileElasticity
                | MaterialProperty::TorsionYield
                | MaterialProperty::TorsionFracture
                | MaterialProperty::TorsionElasticity
                | MaterialProperty::ShearYield
                | MaterialProperty::ShearFracture
                | MaterialProperty::ShearElasticity
                | MaterialProperty::BendingYield
                | MaterialProperty::BendingFracture
                | MaterialProperty::BendingElasticity
                | MaterialProperty::MaxEdge
                | MaterialProperty::SolidDensity => {
                    self.mechanical_properties.parse_tag(tag, value);
                }
                // Liquid and Gas
                MaterialProperty::LiquidDensity => {
                    self.liquid_density = value.parse::<i32>().unwrap_or(0);
                }
                MaterialProperty::MolarMass => {
                    self.molar_mass = value.parse::<i32>().unwrap_or(0);
                }
                // Template
                MaterialProperty::UseMaterialTemplate => {
                    self.template_identifier = String::from(value);
                }
                // Colors
                MaterialProperty::BuildColor => self.build_color = Color::from_value(value),
                MaterialProperty::DisplayColor => self.display_color = Color::from_value(value),

                MaterialProperty::Tile => {
                    self.tile.set_character(value);
                }
                MaterialProperty::TileColor => {
                    self.tile.set_color(value);
                }

                MaterialProperty::MaterialReactionProduct => {
                    self.reaction_product_identifier = String::from(value);
                }

                MaterialProperty::ItemSymbol => {
                    self.item_symbol = String::from(value);
                }

                // Catch-all
                _ => {
                    self.properties.push(format!("{key}:{value}"));
                }
            }

            return;
        }

        if MATERIAL_USAGE_TOKENS.contains_key(key) {
            let Some(usage) = MATERIAL_USAGE_TOKENS.get(key) else {
                warn!(
                    "Material::parse_tag() was provided a key with an invalid usage token: {}",
                    key
                );
                return;
            };
            self.usage.push(usage.clone());
            return;
        }

        // Materials can have syndromes attached and syndromes have creature effects attached.
        if SYNDROME_TOKEN_MAP.contains_key(key)
            || CREATURE_EFFECT_TOKENS.contains_key(key)
            || key == "CE"
        {
            // We need to add the tag to the last syndrome added (all syndromes start with SYNDROME key)
            if let Some(syndrome) = self.syndromes.last_mut() {
                syndrome.parse_tag(key, value);
                return;
            }
        }

        warn!(
            "Material::parse_tag() was provided a key that was not recognized: {}",
            key
        );
    }
}

impl Searchable for Material {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        // Name
        vec.push(self.name.clone());
        // Material Type
        vec.push(self.material_type.to_string());
        // State descriptions
        vec.extend(self.state_names.as_vec());
        vec.extend(self.state_adjectives.as_vec());
        vec.extend(self.state_colors.as_vec());

        // Tags
        vec.extend(
            self.usage
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>(),
        );

        // Syndromes
        vec.extend(
            self.syndromes
                .iter()
                .flat_map(Searchable::get_search_vec)
                .collect::<Vec<String>>(),
        );

        // Reaction Classes (products)
        vec.push(self.reaction_product_identifier.clone());
        // Properties
        vec.extend(self.properties.clone());
        // Usage
        vec.extend(
            self.usage
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<String>>(),
        );

        clean_search_vec(vec.as_slice())
    }
}
