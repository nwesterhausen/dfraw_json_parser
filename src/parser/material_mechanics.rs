use crate::parser::{helpers::serializer_helper, material::Property as MaterialProperty};
use serde::{Deserialize, Serialize};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MaterialMechanics {
    #[serde(skip_serializing_if = "MechanicalProperties::is_empty")]
    /// The impact properties of the material.
    impact: MechanicalProperties,
    #[serde(skip_serializing_if = "MechanicalProperties::is_empty")]
    /// The compressive properties of the material.
    compressive: MechanicalProperties,
    #[serde(skip_serializing_if = "MechanicalProperties::is_empty")]
    /// The tensile properties of the material.
    tensile: MechanicalProperties,
    #[serde(skip_serializing_if = "MechanicalProperties::is_empty")]
    /// The torsion properties of the material.
    torsion: MechanicalProperties,
    #[serde(skip_serializing_if = "MechanicalProperties::is_empty")]
    /// The shear properties of the material.
    shear: MechanicalProperties,
    #[serde(skip_serializing_if = "MechanicalProperties::is_empty")]
    /// The bending properties of the material.
    bending: MechanicalProperties,

    #[serde(skip_serializing_if = "serializer_helper::is_zero_i32")]
    /// The maximum edge size of the material.
    max_edge: i32,

    #[serde(skip_serializing_if = "serializer_helper::is_zero_i32")]
    /// The solid density of the material.
    solid_density: i32,
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct MechanicalProperties {
    #[serde(rename = "yield")]
    /// The yield stress of the material.
    yield_stress: i32,
    /// The fracture stress of the material.
    fracture: i32,
    /// The elasticity of the material.
    elasticity: i32,
}

impl MechanicalProperties {
    /// Create a new `MechanicalProperties` struct with all values set to zero.
    pub fn new() -> Self {
        Self::default()
    }
    /// The function `is_empty` returns true if all values are zero.
    pub fn is_empty(&self) -> bool {
        self.yield_stress == 0 && self.fracture == 0 && self.elasticity == 0
    }
    /// The function `set_yield` sets the yield stress of the material.
    ///
    /// Arguments:
    ///
    /// * `value`: The value to set the yield stress to.
    pub fn set_yield(&mut self, value: i32) {
        self.yield_stress = value;
    }
    /// The function `set_fracture` sets the fracture stress of the material.
    ///
    /// Arguments:
    ///
    /// * `value`: The value to set the fracture stress to.
    pub fn set_fracture(&mut self, value: i32) {
        self.fracture = value;
    }
    /// The function `set_elasticity` sets the elasticity of the material.
    ///
    /// Arguments:
    ///
    /// * `value`: The value to set the elasticity to.
    pub fn set_elasticity(&mut self, value: i32) {
        self.elasticity = value;
    }
}

impl MaterialMechanics {
    /// Create a new `MaterialMechanics` struct with all default values.
    pub fn new() -> Self {
        Self::default()
    }
    /// The function `is_empty` returns true if all values are the defaults
    pub fn is_empty(&self) -> bool {
        self.impact.is_empty()
            && self.compressive.is_empty()
            && self.tensile.is_empty()
            && self.torsion.is_empty()
            && self.shear.is_empty()
            && self.bending.is_empty()
    }
    /// The function `parse_tag` takes a `MaterialProperty` and a string value and sets the
    /// appropriate value in the `MaterialMechanics` struct.
    ///
    /// Arguments:
    ///
    /// * `key`: The `MaterialProperty` to set.
    /// * `value`: The value to set the property to.
    pub fn parse_tag(&mut self, key: &MaterialProperty, value: &str) {
        match key {
            MaterialProperty::ImpactYield => {
                self.impact.set_yield(value.parse::<i32>().unwrap_or(0));
            }
            MaterialProperty::ImpactFracture => {
                self.impact.set_fracture(value.parse::<i32>().unwrap_or(0));
            }
            MaterialProperty::ImpactElasticity => self
                .impact
                .set_elasticity(value.parse::<i32>().unwrap_or(0)),
            MaterialProperty::CompressiveYield => self
                .compressive
                .set_yield(value.parse::<i32>().unwrap_or(0)),
            MaterialProperty::CompressiveFracture => self
                .compressive
                .set_fracture(value.parse::<i32>().unwrap_or(0)),
            MaterialProperty::CompressiveElasticity => self
                .compressive
                .set_elasticity(value.parse::<i32>().unwrap_or(0)),
            MaterialProperty::TensileYield => {
                self.tensile.set_yield(value.parse::<i32>().unwrap_or(0));
            }
            MaterialProperty::TensileFracture => {
                self.tensile.set_fracture(value.parse::<i32>().unwrap_or(0));
            }
            MaterialProperty::TensileElasticity => self
                .tensile
                .set_elasticity(value.parse::<i32>().unwrap_or(0)),
            MaterialProperty::TorsionYield => {
                self.torsion.set_yield(value.parse::<i32>().unwrap_or(0));
            }
            MaterialProperty::TorsionFracture => {
                self.torsion.set_fracture(value.parse::<i32>().unwrap_or(0));
            }
            MaterialProperty::TorsionElasticity => self
                .torsion
                .set_elasticity(value.parse::<i32>().unwrap_or(0)),
            MaterialProperty::ShearYield => self.shear.set_yield(value.parse::<i32>().unwrap_or(0)),
            MaterialProperty::ShearFracture => {
                self.shear.set_fracture(value.parse::<i32>().unwrap_or(0));
            }
            MaterialProperty::ShearElasticity => {
                self.shear.set_elasticity(value.parse::<i32>().unwrap_or(0));
            }
            MaterialProperty::BendingYield => {
                self.bending.set_yield(value.parse::<i32>().unwrap_or(0));
            }
            MaterialProperty::BendingFracture => {
                self.bending.set_fracture(value.parse::<i32>().unwrap_or(0));
            }
            MaterialProperty::BendingElasticity => self
                .bending
                .set_elasticity(value.parse::<i32>().unwrap_or(0)),

            MaterialProperty::MaxEdge => {
                self.max_edge = value.parse::<i32>().unwrap_or(0);
            }
            MaterialProperty::SolidDensity => {
                self.solid_density = value.parse::<i32>().unwrap_or(0);
            }

            _ => {
                log::warn!("Unhandled material mechanics token: '{:?}'", key);
            }
        }
    }
}
