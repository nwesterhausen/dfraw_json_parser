use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::parser::{Color, SingPlurName};

#[derive(ts_rs::TS)]
#[ts(export)]
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SeedMaterial {
    name: SingPlurName,
    color: Color,
    material: String,
}

impl SeedMaterial {
    pub fn is_empty(&self) -> bool {
        self.name.is_empty() && self.color.is_default() && self.material.is_empty()
    }
    pub fn from_value(value: &str) -> SeedMaterial {
        // Example seed tag:
        // [SEED:apricot pit:apricot pits:6:0:0:LOCAL_PLANT_MAT:SEED]
        // Leaving value to be "apricot pit:apricot pits:6:0:0:LOCAL_PLANT_MAT:SEED"
        // We need to split value into its parts
        let mut parts = value.split(':');

        // If the parts are less than 7, then we don't have enough information
        if parts.clone().count() < 7 {
            warn!(
                "SeedMaterial::from_value() was provided a value with less than 7 parts: {}",
                value
            );
            return SeedMaterial::default();
        }

        // The name uses the first two parts
        let name = SingPlurName::from_value(&format!(
            "{}:{}",
            parts.next().unwrap_or_default(),
            parts.next().unwrap_or_default()
        ));
        // The color uses the next three parts
        let color = Color::from_value(&format!(
            "{}:{}:{}",
            parts.next().unwrap_or_default(),
            parts.next().unwrap_or_default(),
            parts.next().unwrap_or_default()
        ));
        // The material uses the remaining parts
        let material = parts.collect::<Vec<&str>>().join(":");
        SeedMaterial {
            name,
            color,
            material,
        }
    }
}
