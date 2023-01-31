use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Temperatures {
    pub specific_heat: u32,
    pub ignition_point: u32,
    pub melting_point: u32,
    pub boiling_point: u32,
    pub heat_damage_point: u32,
    pub cold_damage_point: u32,
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
