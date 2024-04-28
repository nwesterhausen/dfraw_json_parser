use super::properties::Properties;
use crate::parser::{material::PropertyToken, serializer_helper};
use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", rename = "MaterialMechanics")]
/// Represents the specific yield, fracture, and elasticity of a material for the various
/// types of mechanical stress.
pub struct Mechanics {
    //#[serde(skip_serializing_if = "Properties::is_empty")]
    impact: Option<Properties>,
    //#[serde(skip_serializing_if = "Properties::is_empty")]
    compressive: Option<Properties>,
    //#[serde(skip_serializing_if = "Properties::is_empty")]
    tensile: Option<Properties>,
    //#[serde(skip_serializing_if = "Properties::is_empty")]
    torsion: Option<Properties>,
    //#[serde(skip_serializing_if = "Properties::is_empty")]
    shear: Option<Properties>,
    //#[serde(skip_serializing_if = "Properties::is_empty")]
    bending: Option<Properties>,

    //#[serde(skip_serializing_if = "serializer_helper::is_zero_i32")]
    max_edge: Option<i32>,

    //#[serde(skip_serializing_if = "serializer_helper::is_zero_i32")]
    solid_density: Option<i32>,
}

impl Mechanics {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_empty(&self) -> bool {
        self.impact.is_none()
            && self.compressive.is_none()
            && self.tensile.is_none()
            && self.torsion.is_none()
            && self.shear.is_none()
            && self.bending.is_none()
    }
    #[allow(clippy::too_many_lines)]
    pub fn parse_tag(&mut self, key: &PropertyToken, value: &str) {
        match key {
            PropertyToken::ImpactYield => {
                if self.impact.is_none() {
                    self.impact = Some(Properties::new());
                }
                if let Some(impact) = &mut self.impact {
                    impact.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            PropertyToken::ImpactFracture => {
                if self.impact.is_none() {
                    self.impact = Some(Properties::new());
                }
                if let Some(impact) = &mut self.impact {
                    impact.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            PropertyToken::ImpactElasticity => {
                if self.impact.is_none() {
                    self.impact = Some(Properties::new());
                }
                if let Some(impact) = &mut self.impact {
                    impact.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }
            PropertyToken::CompressiveYield => {
                if self.compressive.is_none() {
                    self.compressive = Some(Properties::new());
                }
                if let Some(compressive) = &mut self.compressive {
                    compressive.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            PropertyToken::CompressiveFracture => {
                if self.compressive.is_none() {
                    self.compressive = Some(Properties::new());
                }
                if let Some(compressive) = &mut self.compressive {
                    compressive.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            PropertyToken::CompressiveElasticity => {
                if self.compressive.is_none() {
                    self.compressive = Some(Properties::new());
                }
                if let Some(compressive) = &mut self.compressive {
                    compressive.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }
            PropertyToken::TensileYield => {
                if self.tensile.is_none() {
                    self.tensile = Some(Properties::new());
                }
                if let Some(tensile) = &mut self.tensile {
                    tensile.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            PropertyToken::TensileFracture => {
                if self.tensile.is_none() {
                    self.tensile = Some(Properties::new());
                }
                if let Some(tensile) = &mut self.tensile {
                    tensile.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            PropertyToken::TensileElasticity => {
                if self.tensile.is_none() {
                    self.tensile = Some(Properties::new());
                }
                if let Some(tensile) = &mut self.tensile {
                    tensile.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }
            PropertyToken::TorsionYield => {
                if self.torsion.is_none() {
                    self.torsion = Some(Properties::new());
                }
                if let Some(torsion) = &mut self.torsion {
                    torsion.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            PropertyToken::TorsionFracture => {
                if self.torsion.is_none() {
                    self.torsion = Some(Properties::new());
                }
                if let Some(torsion) = &mut self.torsion {
                    torsion.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            PropertyToken::TorsionElasticity => {
                if self.torsion.is_none() {
                    self.torsion = Some(Properties::new());
                }
                if let Some(torsion) = &mut self.torsion {
                    torsion.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }
            PropertyToken::ShearYield => {
                if self.shear.is_none() {
                    self.shear = Some(Properties::new());
                }
                if let Some(shear) = &mut self.shear {
                    shear.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            PropertyToken::ShearFracture => {
                if self.shear.is_none() {
                    self.shear = Some(Properties::new());
                }
                if let Some(shear) = &mut self.shear {
                    shear.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            PropertyToken::ShearElasticity => {
                if self.shear.is_none() {
                    self.shear = Some(Properties::new());
                }
                if let Some(shear) = &mut self.shear {
                    shear.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }
            PropertyToken::BendingYield => {
                if self.bending.is_none() {
                    self.bending = Some(Properties::new());
                }
                if let Some(bending) = &mut self.bending {
                    bending.set_yield(value.parse::<i32>().unwrap_or(0));
                }
            }
            PropertyToken::BendingFracture => {
                if self.bending.is_none() {
                    self.bending = Some(Properties::new());
                }
                if let Some(bending) = &mut self.bending {
                    bending.set_fracture(value.parse::<i32>().unwrap_or(0));
                }
            }
            PropertyToken::BendingElasticity => {
                if self.bending.is_none() {
                    self.bending = Some(Properties::new());
                }
                if let Some(bending) = &mut self.bending {
                    bending.set_elasticity(value.parse::<i32>().unwrap_or(0));
                }
            }

            PropertyToken::MaxEdge => {
                self.max_edge = Some(value.parse::<i32>().unwrap_or(0));
            }
            PropertyToken::SolidDensity => {
                self.solid_density = Some(value.parse::<i32>().unwrap_or(0));
            }

            _ => {
                warn!("Unhandled material mechanics token: '{:?}'", key);
            }
        }
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
    #[must_use]
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        if let Some(impact) = &cleaned.impact {
            if impact.is_empty() {
                cleaned.impact = None;
            }
        }
        if let Some(compressive) = &cleaned.compressive {
            if compressive.is_empty() {
                cleaned.compressive = None;
            }
        }
        if let Some(tensile) = &cleaned.tensile {
            if tensile.is_empty() {
                cleaned.tensile = None;
            }
        }
        if let Some(torsion) = &cleaned.torsion {
            if torsion.is_empty() {
                cleaned.torsion = None;
            }
        }
        if let Some(shear) = &cleaned.shear {
            if shear.is_empty() {
                cleaned.shear = None;
            }
        }
        if let Some(bending) = &cleaned.bending {
            if bending.is_empty() {
                cleaned.bending = None;
            }
        }

        if serializer_helper::is_zero_i32(cleaned.max_edge) {
            cleaned.max_edge = None;
        }
        if serializer_helper::is_zero_i32(cleaned.solid_density) {
            cleaned.solid_density = None;
        }

        cleaned
    }
}
