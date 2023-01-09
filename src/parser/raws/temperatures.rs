use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Temperatures {
    pub specific_heat: u32,
    pub ignite_point: u32,
    pub melting_point: u32,
    pub boiling_point: u32,
    pub heat_dam_point: u32,
    pub cold_dam_point: u32,
    pub material_fixed_temp: u32,
}

impl Temperatures {
    pub fn new() -> Self {
        Self {
            specific_heat: 0,
            ignite_point: 0,
            melting_point: 0,
            boiling_point: 0,
            heat_dam_point: 0,
            cold_dam_point: 0,
            material_fixed_temp: 0,
        }
    }
}
