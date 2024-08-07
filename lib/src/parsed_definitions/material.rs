//! A module to handle the parsing of material definitions from the raws.

use tracing::warn;

use crate::{
    color::Color,
    default_checks,
    material_mechanics::MaterialMechanics,
    raw_definitions::{
        CREATURE_EFFECT_TOKENS, FUEL_TYPE_TOKENS, MATERIAL_PROPERTY_TOKENS, MATERIAL_TYPE_TOKENS,
        MATERIAL_USAGE_TOKENS, SYNDROME_TOKENS,
    },
    state_names::StateNames,
    syndrome::Syndrome,
    tags::{FuelTypeTag, MaterialPropertyTag, MaterialTypeTag, MaterialUsageTag},
    temperatures::Temperatures,
    tile::Tile,
    traits::{searchable::clean_search_vec, Searchable},
};

/// A struct representing a material
#[allow(clippy::module_name_repetitions)]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    /// The type of the material is also the trigger to start tracking a material
    #[serde(skip_serializing_if = "Option::is_none")]
    material_type: Option<MaterialTypeTag>,
    /// The material might have a name, but its more likely that there is only an identifier to
    /// refer to another creature/plant/reaction, which are listed elsewhere.
    /// If there is no name provided, then it is a special hardcoded case, e.g. magma or green glass.
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// For the coal tag, it specifies the type of fuel that can be used. It will never be None.
    #[serde(skip_serializing_if = "Option::is_none")]
    fuel_type: Option<FuelTypeTag>,
    /// Linked creature identifier (and then `material_name` might be "skin", like for "`CREATURE_MAT:DWARF:SKIN`")
    #[serde(skip_serializing_if = "Option::is_none")]
    creature_identifier: Option<String>,
    /// Linked plant identifier (and then `material_name` might be "leaf", like for "`PLANT_MAT:BUSH_QUARRY:LEAF`")
    #[serde(skip_serializing_if = "Option::is_none")]
    plant_identifier: Option<String>,
    /// If a material is defined within a creature itself, it will use `LOCAL_CREATURE_MAT` tag, which implies
    /// that the material is only used by that creature. This is also true for plants and `LOCAL_PLANT_MAT`.
    // skip if false
    #[serde(skip_serializing_if = "Option::is_none")]
    is_local_material: Option<bool>,
    /// Within a reaction, there can be special material definitions. Todo: Figure this out.
    #[serde(skip_serializing_if = "Option::is_none")]
    reagent_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reaction_product_identifier: Option<String>,
    /// If material is defined from a template, we need a way to refer to that
    #[serde(skip_serializing_if = "Option::is_none")]
    template_identifier: Option<String>,

    /// Usage tags
    #[serde(skip_serializing_if = "Option::is_none")]
    usage: Option<Vec<MaterialUsageTag>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,

    #[serde(skip_serializing_if = "Option::is_none")]
    state_names: Option<StateNames>,

    #[serde(skip_serializing_if = "Option::is_none")]
    state_adjectives: Option<StateNames>,

    #[serde(skip_serializing_if = "Option::is_none")]
    state_colors: Option<StateNames>,

    #[serde(skip_serializing_if = "Option::is_none")]
    temperatures: Option<Temperatures>,

    /// Catch-all for remaining tags we identify but don't do anything with... yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    properties: Option<Vec<String>>,

    // Syndromes attached to materials..
    #[serde(skip_serializing_if = "Option::is_none")]
    syndromes: Option<Vec<Syndrome>>,
    // Material Mechanical Properties
    #[serde(skip_serializing_if = "Option::is_none")]
    mechanical_properties: Option<MaterialMechanics>,
    // Technically, the material mechanics wouldn't apply to liquid or gaseous forms
    #[serde(skip_serializing_if = "Option::is_none")]
    liquid_density: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    molar_mass: Option<i32>,

    // Colors
    #[serde(skip_serializing_if = "Option::is_none")]
    build_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_color: Option<Color>,

    // Display
    #[serde(skip_serializing_if = "Option::is_none")]
    tile: Option<Tile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    item_symbol: Option<String>,
}

impl Material {
    /// Create a new material
    ///
    /// # Returns
    ///
    /// A new material
    #[must_use]
    pub fn new() -> Self {
        Self {
            value: Some(1),
            ..Self::default()
        }
    }
    /// This may not be correct. This should be for `[USE_MATERIAL:XX:XX]` but couldn't find an example for Plant.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the material
    ///
    /// # Returns
    ///
    /// A new material
    #[must_use]
    pub fn use_material_from_value(value: &str) -> Self {
        // Start defining a new material with a name and properties of another local material
        let mut split = value.split(':');

        let material_name = split.next().unwrap_or_default();
        let parent_material_name = split.next().unwrap_or_default();

        Self {
            name: Some(String::from(material_name)),
            template_identifier: Some(String::from(parent_material_name)),
            is_local_material: Some(true),
            ..Self::new()
        }
    }
    /// Create a new material from a template
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the material
    ///
    /// # Returns
    ///
    /// A new material
    #[must_use]
    pub fn use_material_template_from_value(value: &str) -> Self {
        // Start defining a new material with a name and properties of another local material
        let mut split = value.split(':');

        let material_name = split.next().unwrap_or_default();
        let template_material_name = split.next().unwrap_or_default();

        Self {
            name: Some(String::from(material_name)),
            template_identifier: Some(String::from(template_material_name)),
            is_local_material: Some(true),
            ..Self::new()
        }
    }
    /// Create a new material from a basic material
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the material
    ///
    /// # Returns
    ///
    /// A new material
    #[must_use]
    pub fn basic_material_from_value(value: &str) -> Self {
        Self::from_value(value)
    }
    /// Create a new material from a material
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the material
    ///
    /// # Returns
    ///
    /// A new material
    #[must_use]
    pub fn from_value(value: &str) -> Self {
        // Value is a string like "`CREATURE_MAT:DWARF:SKIN`" or "`INORGANIC`" or "`STONE:MARBLE`" or "`LOCAL_PLANT_MAT:LEAF`"
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
            return Self::new();
        };

        // If there is only one part, then it is a special hardcoded material, like magma or water.
        if split_len == 1 {
            return Self {
                material_type: Some(material_type.clone()),
                ..Self::new()
            };
        }
        // If there are more than one parts, we can use a match and drill down further.
        // Use the phf_table to get the type of the material and then match from there.
        match material_type {
            MaterialTypeTag::Inorganic | MaterialTypeTag::Stone | MaterialTypeTag::Metal => {
                let material_name = split.next().unwrap_or_default();
                Self {
                    material_type: Some(material_type.clone()),
                    name: Some(String::from(material_name)),
                    ..Self::new()
                }
            }
            MaterialTypeTag::Coal => {
                let material_key = split.next().unwrap_or_default();
                let Some(fuel_type) = FUEL_TYPE_TOKENS.get(material_key) else {
                    warn!(
                        "Material::from_value() was provided a value with an invalid fuel type: {}",
                        value
                    );
                    return Self {
                        material_type: Some(material_type.clone()),
                        ..Self::new()
                    };
                };
                Self {
                    material_type: Some(material_type.clone()),
                    fuel_type: Some(fuel_type.clone()),
                    ..Self::new()
                }
            }
            MaterialTypeTag::LocalCreatureMaterial | MaterialTypeTag::LocalPlantMaterial => {
                let material_name = split.next().unwrap_or_default();
                Self {
                    material_type: Some(material_type.clone()),
                    name: Some(String::from(material_name)),
                    is_local_material: Some(true),
                    ..Self::new()
                }
            }
            MaterialTypeTag::CreatureMaterial => {
                let creature_identifier = split.next().unwrap_or_default();
                let material_name = split.next().unwrap_or_default();
                Self {
                    material_type: Some(material_type.clone()),
                    creature_identifier: Some(String::from(creature_identifier)),
                    name: Some(String::from(material_name)),
                    ..Self::new()
                }
            }
            MaterialTypeTag::PlantMaterial => {
                let plant_identifier = split.next().unwrap_or_default();
                let material_name = split.next().unwrap_or_default();
                Self {
                    material_type: Some(material_type.clone()),
                    plant_identifier: Some(String::from(plant_identifier)),
                    name: Some(String::from(material_name)),
                    ..Self::new()
                }
            }
            MaterialTypeTag::GetMaterialFromReagent => {
                let reagent_identifier = split.next().unwrap_or_default();
                let reaction_product_identifier = split.next().unwrap_or_default();
                Self {
                    material_type: Some(material_type.clone()),
                    reagent_identifier: Some(String::from(reagent_identifier)),
                    reaction_product_identifier: Some(String::from(reaction_product_identifier)),
                    ..Self::new()
                }
            }
            _ => {
                warn!(
                    "Material::from_value() was provided a value with an invalid material type: {}",
                    value
                );
                Self::new()
            }
        }
    }
    /// Parses a tag and value into the material
    ///
    /// # Arguments
    ///
    /// * `key` - The tag of the material
    /// * `value` - The value of the material
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
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
                MaterialPropertyTag::MaterialValue => {
                    self.value = Some(value.parse::<u32>().unwrap_or(1));
                }
                MaterialPropertyTag::StateNameAdjective => {
                    if self.state_names.is_none() {
                        self.state_names = Some(StateNames::default());
                    }
                    if let Some(state_names) = self.state_names.as_mut() {
                        state_names.add_from_value(value);
                    }
                    if self.state_adjectives.is_none() {
                        self.state_adjectives = Some(StateNames::default());
                    }
                    if let Some(state_adjectives) = self.state_adjectives.as_mut() {
                        state_adjectives.add_from_value(value);
                    }
                }
                // Names and Adjectives
                MaterialPropertyTag::StateName => {
                    if self.state_names.is_none() {
                        self.state_names = Some(StateNames::default());
                    }
                    if let Some(state_names) = self.state_names.as_mut() {
                        state_names.add_from_value(value);
                    }
                }
                MaterialPropertyTag::StateAdjective => {
                    if self.state_adjectives.is_none() {
                        self.state_adjectives = Some(StateNames::default());
                    }
                    if let Some(state_adjectives) = self.state_adjectives.as_mut() {
                        state_adjectives.add_from_value(value);
                    }
                }
                MaterialPropertyTag::StateColor => {
                    if self.state_colors.is_none() {
                        self.state_colors = Some(StateNames::default());
                    }
                    if let Some(state_colors) = self.state_colors.as_mut() {
                        state_colors.add_from_value(value);
                    }
                }
                MaterialPropertyTag::BasicColor => self.color = Some(Color::from_value(value)),
                // Temperatures
                MaterialPropertyTag::SpecificHeat => {
                    if self.temperatures.is_none() {
                        self.temperatures = Some(Temperatures::default());
                    }
                    if let Some(temperatures) = self.temperatures.as_mut() {
                        temperatures.update_specific_heat(value.parse::<u32>().unwrap_or(0));
                    }
                }
                MaterialPropertyTag::IgnitionPoint => {
                    if self.temperatures.is_none() {
                        self.temperatures = Some(Temperatures::default());
                    }
                    if let Some(temperatures) = self.temperatures.as_mut() {
                        temperatures.update_ignition_point(value.parse::<u32>().unwrap_or(0));
                    }
                }
                MaterialPropertyTag::MeltingPoint => {
                    if self.temperatures.is_none() {
                        self.temperatures = Some(Temperatures::default());
                    }
                    if let Some(temperatures) = self.temperatures.as_mut() {
                        temperatures.update_melting_point(value.parse::<u32>().unwrap_or(0));
                    }
                }
                MaterialPropertyTag::BoilingPoint => {
                    if self.temperatures.is_none() {
                        self.temperatures = Some(Temperatures::default());
                    }
                    if let Some(temperatures) = self.temperatures.as_mut() {
                        temperatures.update_boiling_point(value.parse::<u32>().unwrap_or(0));
                    }
                }
                MaterialPropertyTag::HeatDamagePoint => {
                    if self.temperatures.is_none() {
                        self.temperatures = Some(Temperatures::default());
                    }
                    if let Some(temperatures) = self.temperatures.as_mut() {
                        temperatures.update_heat_damage_point(value.parse::<u32>().unwrap_or(0));
                    }
                }
                MaterialPropertyTag::ColdDamagePoint => {
                    if self.temperatures.is_none() {
                        self.temperatures = Some(Temperatures::default());
                    }
                    if let Some(temperatures) = self.temperatures.as_mut() {
                        temperatures.update_cold_damage_point(value.parse::<u32>().unwrap_or(0));
                    }
                }
                MaterialPropertyTag::MaterialFixedTemperature => {
                    if self.temperatures.is_none() {
                        self.temperatures = Some(Temperatures::default());
                    }
                    if let Some(temperatures) = self.temperatures.as_mut() {
                        temperatures
                            .update_material_fixed_temperature(value.parse::<u32>().unwrap_or(0));
                    }
                }
                // Syndrome
                MaterialPropertyTag::Syndrome => {
                    let syndrome = Syndrome::new();
                    if let Some(syndromes) = self.syndromes.as_mut() {
                        syndromes.push(syndrome);
                    } else {
                        self.syndromes = Some(vec![syndrome]);
                    }
                }
                // Material Mechanics..
                MaterialPropertyTag::ImpactYield
                | MaterialPropertyTag::ImpactFracture
                | MaterialPropertyTag::ImpactElasticity
                | MaterialPropertyTag::CompressiveYield
                | MaterialPropertyTag::CompressiveFracture
                | MaterialPropertyTag::CompressiveElasticity
                | MaterialPropertyTag::TensileYield
                | MaterialPropertyTag::TensileFracture
                | MaterialPropertyTag::TensileElasticity
                | MaterialPropertyTag::TorsionYield
                | MaterialPropertyTag::TorsionFracture
                | MaterialPropertyTag::TorsionElasticity
                | MaterialPropertyTag::ShearYield
                | MaterialPropertyTag::ShearFracture
                | MaterialPropertyTag::ShearElasticity
                | MaterialPropertyTag::BendingYield
                | MaterialPropertyTag::BendingFracture
                | MaterialPropertyTag::BendingElasticity
                | MaterialPropertyTag::MaxEdge
                | MaterialPropertyTag::SolidDensity => {
                    if self.mechanical_properties.is_none() {
                        self.mechanical_properties = Some(MaterialMechanics::new());
                    }
                    if let Some(mechanical_properties) = self.mechanical_properties.as_mut() {
                        mechanical_properties.parse_tag(tag, value);
                    }
                }
                // Liquid and Gas
                MaterialPropertyTag::LiquidDensity => {
                    self.liquid_density = Some(value.parse::<i32>().unwrap_or(0));
                }
                MaterialPropertyTag::MolarMass => {
                    self.molar_mass = Some(value.parse::<i32>().unwrap_or(0));
                }
                // Template
                MaterialPropertyTag::UseMaterialTemplate => {
                    self.template_identifier = Some(String::from(value));
                }
                // Colors
                MaterialPropertyTag::BuildColor => {
                    self.build_color = Some(Color::from_value(value));
                }
                MaterialPropertyTag::DisplayColor => {
                    self.display_color = Some(Color::from_value(value));
                }

                MaterialPropertyTag::Tile => {
                    if self.tile.is_none() {
                        self.tile = Some(Tile::default());
                    }
                    if let Some(tile) = self.tile.as_mut() {
                        tile.set_character(value);
                    }
                }
                MaterialPropertyTag::TileColor => {
                    if self.tile.is_none() {
                        self.tile = Some(Tile::default());
                    }
                    if let Some(tile) = self.tile.as_mut() {
                        tile.set_color(value);
                    }
                }

                MaterialPropertyTag::MaterialReactionProduct => {
                    self.reaction_product_identifier = Some(String::from(value));
                }

                MaterialPropertyTag::ItemSymbol => {
                    self.item_symbol = Some(String::from(value));
                }

                // Catch-all
                _ => {
                    if self.properties.is_none() {
                        self.properties = Some(Vec::new());
                    }
                    if let Some(properties) = self.properties.as_mut() {
                        properties.push(format!("{key}:{value}"));
                    }
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
            if self.usage.is_none() {
                self.usage = Some(Vec::new());
            }
            if let Some(self_usage) = self.usage.as_mut() {
                self_usage.push(usage.clone());
            }
            return;
        }

        // Materials can have syndromes attached and syndromes have creature effects attached.
        if SYNDROME_TOKENS.contains_key(key)
            || CREATURE_EFFECT_TOKENS.contains_key(key)
            || key == "CE"
        {
            if let Some(syndromes) = self.syndromes.as_mut() {
                // We need to add the tag to the last syndrome added (all syndromes start with SYNDROME key)
                if let Some(syndrome) = syndromes.last_mut() {
                    syndrome.parse_tag(key, value);
                    return;
                }
            }
        }

        warn!(
            "Material::parse_tag() was provided a key that was not recognized: {}",
            key
        );
    }

    /// Function to "clean" the raw. This is used to remove any empty list or strings,
    /// and to remove any default values. By "removing" it means setting the value to None.
    ///
    /// This also will remove the metadata if `is_metadata_hidden` is true.
    ///
    /// Steps for all "Option" fields:
    /// - Set any metadata to None if `is_metadata_hidden` is true.
    /// - Set any empty string to None.
    /// - Set any empty list to None.
    /// - Set any default values to None.
    ///
    /// # Returns
    ///
    /// A new material with all empty or default values removed.
    #[allow(clippy::too_many_lines, clippy::cognitive_complexity)]
    #[must_use]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        if let Some(material_type) = &cleaned.material_type {
            if material_type.is_default() {
                cleaned.material_type = None;
            }
        }
        if let Some(name) = &cleaned.name {
            if name.is_empty() {
                cleaned.name = None;
            }
        }
        if let Some(fuel_type) = &cleaned.fuel_type {
            if fuel_type.is_default() {
                cleaned.fuel_type = None;
            }
        }
        if let Some(creature_identifier) = &cleaned.creature_identifier {
            if creature_identifier.is_empty() {
                cleaned.creature_identifier = None;
            }
        }
        if let Some(plant_identifier) = &cleaned.plant_identifier {
            if plant_identifier.is_empty() {
                cleaned.plant_identifier = None;
            }
        }
        if let Some(is_local_material) = &cleaned.is_local_material {
            if !is_local_material {
                cleaned.is_local_material = None;
            }
        }
        if let Some(reagent_identifier) = &cleaned.reagent_identifier {
            if reagent_identifier.is_empty() {
                cleaned.reagent_identifier = None;
            }
        }
        if let Some(reaction_product_identifier) = &cleaned.reaction_product_identifier {
            if reaction_product_identifier.is_empty() {
                cleaned.reaction_product_identifier = None;
            }
        }
        if let Some(template_identifier) = &cleaned.template_identifier {
            if template_identifier.is_empty() {
                cleaned.template_identifier = None;
            }
        }
        if let Some(usage) = &cleaned.usage {
            if usage.is_empty() {
                cleaned.usage = None;
            }
        }
        if default_checks::is_one_u32(cleaned.value) {
            cleaned.value = None;
        }
        if let Some(color) = &cleaned.color {
            if color.is_default() {
                cleaned.color = None;
            }
        }
        if let Some(state_names) = &cleaned.state_names {
            if state_names.is_empty() {
                cleaned.state_names = None;
            }
        }
        if let Some(state_adjectives) = &cleaned.state_adjectives {
            if state_adjectives.is_empty() {
                cleaned.state_adjectives = None;
            }
        }
        if let Some(state_colors) = &cleaned.state_colors {
            if state_colors.is_empty() {
                cleaned.state_colors = None;
            }
        }
        if let Some(temperatures) = &cleaned.temperatures {
            if temperatures.is_empty() {
                cleaned.temperatures = None;
            }
        }
        if let Some(properties) = &cleaned.properties {
            if properties.is_empty() {
                cleaned.properties = None;
            }
        }
        if let Some(syndromes) = &cleaned.syndromes {
            let mut cleaned_syndromes = Vec::new();
            for syndrome in syndromes {
                cleaned_syndromes.push(syndrome.cleaned());
            }
            if cleaned_syndromes.is_empty() {
                cleaned.syndromes = None;
            } else {
                cleaned.syndromes = Some(cleaned_syndromes);
            }
        }
        if let Some(mechanical_properties) = &cleaned.mechanical_properties {
            if mechanical_properties.is_empty() {
                cleaned.mechanical_properties = None;
            }
        }
        if default_checks::is_zero_i32(cleaned.liquid_density) {
            cleaned.liquid_density = None;
        }
        if default_checks::is_zero_i32(cleaned.molar_mass) {
            cleaned.molar_mass = None;
        }
        if let Some(build_color) = &cleaned.build_color {
            if build_color.is_default() {
                cleaned.build_color = None;
            }
        }
        if let Some(display_color) = &cleaned.display_color {
            if display_color.is_default() {
                cleaned.display_color = None;
            }
        }
        if let Some(tile) = &cleaned.tile {
            if tile.is_default() {
                cleaned.tile = None;
            }
        }
        if let Some(item_symbol) = &cleaned.item_symbol {
            if item_symbol.is_empty() {
                cleaned.item_symbol = None;
            }
        }

        cleaned
    }
}

impl Searchable for Material {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        // Name
        if let Some(name) = &self.name {
            vec.push(name.to_string());
        }
        // Material Type
        if let Some(material_type) = &self.material_type {
            vec.push(material_type.to_string());
        }
        // State descriptions
        if let Some(state_names) = &self.state_names {
            vec.extend(state_names.as_vec());
        }
        if let Some(state_adjectives) = &self.state_adjectives {
            vec.extend(state_adjectives.as_vec());
        }
        if let Some(state_colors) = &self.state_colors {
            vec.extend(state_colors.as_vec());
        }

        // Syndromes
        if let Some(syndromes) = &self.syndromes {
            vec.extend(syndromes.iter().flat_map(Searchable::get_search_vec));
        }

        // Reaction Classes (products)
        if let Some(reaction_product_identifier) = &self.reaction_product_identifier {
            vec.push(reaction_product_identifier.clone());
        }
        // Properties
        if let Some(properties) = &self.properties {
            vec.extend(properties.iter().map(std::string::ToString::to_string));
        }
        // Usage
        if let Some(usage) = &self.usage {
            vec.extend(usage.iter().map(std::string::ToString::to_string));
        }
        clean_search_vec(vec.as_slice())
    }
}
