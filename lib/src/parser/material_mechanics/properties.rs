use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase", rename = "MechanicalProperties")]
/// Represents the mechanical properties of a material via the yield, fracture, and elasticity
pub struct Properties {
    #[serde(rename = "yield")]
    yield_stress: i32,
    fracture: i32,
    elasticity: i32,
}

impl Properties {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_empty(&self) -> bool {
        self.yield_stress == 0 && self.fracture == 0 && self.elasticity == 0
    }
    pub fn set_yield(&mut self, value: i32) {
        self.yield_stress = value;
    }
    pub fn set_fracture(&mut self, value: i32) {
        self.fracture = value;
    }
    pub fn set_elasticity(&mut self, value: i32) {
        self.elasticity = value;
    }
}
