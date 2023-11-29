use crate::parser::helpers::serializer_helper;
use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
<<<<<<< HEAD:lib/src/parser/simple_structs/temperatures.rs
<<<<<<< HEAD:lib/src/parser/simple_structs/temperatures.rs
/// The temperature properties of a material
=======
/// This struct describes the temperatures of a material. The way temperatures are defined in the raws
/// is by many individual tokens, each of which describes a different temperature-related property. This
/// struct is used to group all of those tokens together.
>>>>>>> 6f58260 (docs: add doc comments):src/parser/temperature.rs
=======
/// This struct describes the temperatures of a material. The way temperatures are defined in the raws
/// is by many individual tokens, each of which describes a different temperature-related property. This
/// struct is used to group all of those tokens together.
>>>>>>> c2812957821240fff30b78553e73f23e904207e2:src/parser/temperature.rs
pub struct Temperatures {
    /// This determines how long it takes the material to heat up or cool down.
    /// A material with a high specific heat capacity will hold more heat and affect its surroundings more
    /// before cooling down or heating up to equilibrium. The input for this token is not temperature,
    /// but rather the specific heat capacity of the material.
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    specific_heat: u32,
    /// This is the temperature at which the material will catch fire.
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    ignition_point: u32,
    /// This is the temperature at which a liquid material will freeze, or a solid material will melt.
    /// In Dwarf Fortress the melting point and freezing point coincide exactly; this is contrary to many
    /// real-life materials, which can be supercooled.
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    melting_point: u32,
    /// This is the temperature at which the material will boil or condense. Water boils at 10180 °U
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    boiling_point: u32,
    /// This is the temperature above which the material will begin to take heat damage.
    /// Burning items without a heat damage point (or with an exceptionally high one) will take damage very slowly,
    /// causing them to burn for a very long time (9 months and 16.8 days) before disappearing.
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    heat_damage_point: u32,
    /// This is the temperature below which the material will begin to take frost damage.
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    cold_damage_point: u32,
    /// A material's temperature can be forced to always be a certain value via the `MAT_FIXED_TEMP`
    /// material definition token. The only standard material which uses this is nether-cap wood,
    /// whose temperature is always at the melting point of water. If a material's temperature is fixed
    /// to between its cold damage point and its heat damage point, then items made from that material
    /// will never suffer cold/heat damage. This makes nether-caps fire-safe and magma-safe despite being a type of wood.
    #[serde(skip_serializing_if = "serializer_helper::is_zero")]
    material_fixed_temperature: u32,
}

impl Temperatures {
    /// Check if the `Temperatures` struct is empty/default.
    ///
    /// Returns:
    ///
    /// * `bool`: True if the `Temperatures` struct is empty, false otherwise.
    pub fn is_empty(&self) -> bool {
        self.specific_heat == 0
            && self.ignition_point == 0
            && self.melting_point == 0
            && self.boiling_point == 0
            && self.heat_damage_point == 0
            && self.cold_damage_point == 0
            && self.material_fixed_temperature == 0
    }
    /// Set the `specific_heat` field.
    ///
    /// Arguments:
    ///
    /// * `value`: The value to set the `specific_heat` field to.
    pub fn set_specific_heat(&mut self, value: u32) {
        self.specific_heat = value;
    }
    /// Set the `ignition_point` field.
    ///
    /// Arguments:
    ///
    /// * `value`: The value to set the `ignition_point` field to.
    pub fn set_ignition_point(&mut self, value: u32) {
        self.ignition_point = value;
    }
    /// Set the `melting_point` field.
    ///
    /// Arguments:
    ///
    /// * `value`: The value to set the `melting_point` field to.
    pub fn set_melting_point(&mut self, value: u32) {
        self.melting_point = value;
    }
    /// Set the `boiling_point` field.
    ///
    /// Arguments:
    ///
    /// * `value`: The value to set the `boiling_point` field to.
    pub fn set_boiling_point(&mut self, value: u32) {
        self.boiling_point = value;
    }
    /// Set the `heat_damage_point` field.
    ///
    /// Arguments:
    ///
    /// * `value`: The value to set the `heat_damage_point` field to.
    pub fn set_heat_damage_point(&mut self, value: u32) {
        self.heat_damage_point = value;
    }
    /// Set the `cold_damage_point` field.
    ///
    /// Arguments:
    ///
    /// * `value`: The value to set the `cold_damage_point` field to.
    pub fn set_cold_damage_point(&mut self, value: u32) {
        self.cold_damage_point = value;
    }
    /// Set the `material_fixed_temperature` field.
    ///
    /// Arguments:
    ///
    /// * `value`: The value to set the `material_fixed_temperature` field to.
    pub fn set_material_fixed_temperature(&mut self, value: u32) {
        self.material_fixed_temperature = value;
    }
}
