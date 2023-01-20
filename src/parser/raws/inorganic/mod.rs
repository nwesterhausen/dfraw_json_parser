use super::{environment, roll_chance};
use super::{material, DFRawCommon};

mod inorganic_basic;
mod parse;
mod typed_json;

#[derive(Debug, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct DFInorganic {
    // Common Raw file Things
    raw_header: DFRawCommon,
    pub tags: Vec<super::tags::InorganicTag>,

    // Basic Tokens
    pub material: material::SimpleMaterial,
    pub environments: Vec<environment::Environment>,
    pub environments_specific: Vec<environment::Environment>,

    pub metal_ores: Vec<roll_chance::RollChance>,
    pub thread_metals: Vec<roll_chance::RollChance>,
}
