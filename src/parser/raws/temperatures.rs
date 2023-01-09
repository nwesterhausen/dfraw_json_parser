use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Temperatures {
    #[serde(rename = "specificHeat")]
    pub specific_heat: u32,
    #[serde(rename = "ignitionPoint")]
    pub ignition_point: u32,
    #[serde(rename = "meltingPoint")]
    pub melting_point: u32,
    #[serde(rename = "boilingPoint")]
    pub boiling_point: u32,
    #[serde(rename = "heatDamagePoint")]
    pub heat_damage_point: u32,
    #[serde(rename = "coldDamagePoint")]
    pub cold_damage_point: u32,
    #[serde(rename = "materialFixedTemp")]
    pub material_fixed_temp: u32,
}

impl Temperatures {
    pub fn new() -> Self {
        Self {
            specific_heat: 0,
            ignition_point: 0,
            melting_point: 0,
            boiling_point: 0,
            heat_damage_point: 0,
            cold_damage_point: 0,
            material_fixed_temp: 0,
        }
    }
}
