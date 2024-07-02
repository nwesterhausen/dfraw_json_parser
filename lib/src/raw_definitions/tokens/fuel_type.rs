//! String token to parsed tag map for fuel type tokens.

use crate::tags::FuelTypeTag;

/// Mapping of fuel type tokens to strings
pub static FUEL_TYPE_TOKENS: phf::Map<&'static str, FuelTypeTag> = phf::phf_map! {
  "COAL" => FuelTypeTag::Charcoal,
  "COKE" => FuelTypeTag::Coke,
  "NO_MATGLOSS" => FuelTypeTag::NoMaterialGloss,
};
