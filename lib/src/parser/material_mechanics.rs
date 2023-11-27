use crate::parser::{material::PropertyToken, serializer_helper};
use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MaterialMechanics {
    #[serde(skip_serializing_if = "MechanicalProperties::is_empty")]
    impact: MechanicalProperties,
    #[serde(skip_serializing_if = "MechanicalProperties::is_empty")]
    compressive: MechanicalProperties,
    #[serde(skip_serializing_if = "MechanicalProperties::is_empty")]
    tensile: MechanicalProperties,
    #[serde(skip_serializing_if = "MechanicalProperties::is_empty")]
    torsion: MechanicalProperties,
    #[serde(skip_serializing_if = "MechanicalProperties::is_empty")]
    shear: MechanicalProperties,
    #[serde(skip_serializing_if = "MechanicalProperties::is_empty")]
    bending: MechanicalProperties,

    #[serde(skip_serializing_if = "serializer_helper::is_zero_i32")]
    max_edge: i32,

    #[serde(skip_serializing_if = "serializer_helper::is_zero_i32")]
    solid_density: i32,
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MechanicalProperties {
    #[serde(rename = "yield")]
    yield_stress: i32,
    fracture: i32,
    elasticity: i32,
}

impl MechanicalProperties {
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

impl MaterialMechanics {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_empty(&self) -> bool {
        self.impact.is_empty()
            && self.compressive.is_empty()
            && self.tensile.is_empty()
            && self.torsion.is_empty()
            && self.shear.is_empty()
            && self.bending.is_empty()
    }
    pub fn parse_tag(&mut self, key: &PropertyToken, value: &str) {
        match key {
            PropertyToken::ImpactYield => {
                self.impact.set_yield(value.parse::<i32>().unwrap_or(0));
            }
            PropertyToken::ImpactFracture => {
                self.impact.set_fracture(value.parse::<i32>().unwrap_or(0));
            }
            PropertyToken::ImpactElasticity => self
                .impact
                .set_elasticity(value.parse::<i32>().unwrap_or(0)),
            PropertyToken::CompressiveYield => self
                .compressive
                .set_yield(value.parse::<i32>().unwrap_or(0)),
            PropertyToken::CompressiveFracture => self
                .compressive
                .set_fracture(value.parse::<i32>().unwrap_or(0)),
            PropertyToken::CompressiveElasticity => self
                .compressive
                .set_elasticity(value.parse::<i32>().unwrap_or(0)),
            PropertyToken::TensileYield => {
                self.tensile.set_yield(value.parse::<i32>().unwrap_or(0));
            }
            PropertyToken::TensileFracture => {
                self.tensile.set_fracture(value.parse::<i32>().unwrap_or(0));
            }
            PropertyToken::TensileElasticity => self
                .tensile
                .set_elasticity(value.parse::<i32>().unwrap_or(0)),
            PropertyToken::TorsionYield => {
                self.torsion.set_yield(value.parse::<i32>().unwrap_or(0));
            }
            PropertyToken::TorsionFracture => {
                self.torsion.set_fracture(value.parse::<i32>().unwrap_or(0));
            }
            PropertyToken::TorsionElasticity => self
                .torsion
                .set_elasticity(value.parse::<i32>().unwrap_or(0)),
            PropertyToken::ShearYield => self.shear.set_yield(value.parse::<i32>().unwrap_or(0)),
            PropertyToken::ShearFracture => {
                self.shear.set_fracture(value.parse::<i32>().unwrap_or(0));
            }
            PropertyToken::ShearElasticity => {
                self.shear.set_elasticity(value.parse::<i32>().unwrap_or(0));
            }
            PropertyToken::BendingYield => {
                self.bending.set_yield(value.parse::<i32>().unwrap_or(0));
            }
            PropertyToken::BendingFracture => {
                self.bending.set_fracture(value.parse::<i32>().unwrap_or(0));
            }
            PropertyToken::BendingElasticity => self
                .bending
                .set_elasticity(value.parse::<i32>().unwrap_or(0)),

            PropertyToken::MaxEdge => {
                self.max_edge = value.parse::<i32>().unwrap_or(0);
            }
            PropertyToken::SolidDensity => {
                self.solid_density = value.parse::<i32>().unwrap_or(0);
            }

            _ => {
                warn!("Unhandled material mechanics token: '{:?}'", key);
            }
        }
    }
}
