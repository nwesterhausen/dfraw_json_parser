use crate::parser::raws::{
    info::DFInfoFile,
    names::{Name, SingPlurName},
    tags::{self, CasteTag},
};
use crate::parser::reader::RawObjectKind;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DFMaterial {
    // Common Raw file Things
    identifier: String,
    parent_raw: String,
    dfraw_identifier: String,
    dfraw_version: String,
    dfraw_found_in: String,
    raw_type: RawObjectKind,
    #[serde(rename = "objectId")]
    object_id: String,
    pub tags: Vec<tags::MaterialTag>,

    // Material Properties
    pub use_material_template: Vec<String>,
    pub prefix: String,
    pub stone_name: String,
    pub is_gem: IsGemToken,
    pub temp_diet_info: tags::ContaminateTag,
    pub states: MaterialStateDescription,
    pub absorption: u32,
    pub impact_yield: u32,
    pub impact_fracture: u32,
    pub impact_elasticity: u32, // aka impact_strain_at_yield
    pub compressive_yield: u32,
    pub compressive_fracture: u32,
    pub compressive_elasticity: u32, // aka compressive_strain_at_yield
    pub tensile_yield: u32,
    pub tensile_fracture: u32,
    pub tensile_elasticity: u32, // aka tensile_strain_at_yield
    pub torsion_yield: u32,
    pub torsion_fracture: u32,
    pub torsion_elasticity: u32, // aka torsion_strain_at_yield
    pub shear_yield: u32,
    pub shear_fracture: u32,
    pub shear_elasticity: u32, // aka torsion_strain_at_yield
    pub bending_yield: u32,
    pub bending_fracture: u32,
    pub bending_elasticity: u32, // aka torsion_strain_at_yield
    pub max_edge: u32,
    pub material_value: u32,
    pub multiple_value: u32,
    pub specific_head_capacity: u32,
    pub damage_point_heat: u32,
    pub damage_point_cold: u32,
    pub ignite_point: u32,
    pub melting_point: u32,
    pub boiling_point: u32,
    pub mat_fixed_temp: u32,
    pub solid_density: u32,
    pub liquid_density: u32,
    pub molar_mass: u32,
    pub extract_storage: MaterialExtractStorage,
    pub butcher_special: String,
    pub meat_name: MaterialMeatName,
    pub block_name: MaterialBlockName,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialBlockName {
    pub singular: String,
    pub plural: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialMeatName {
    pub prefix: String,
    pub name: String,
    pub adjective: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MaterialExtractStorage {
    Barrel,
    Flask,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MaterialStateDescription {
    pub state: tags::MaterialStateTag,
    pub name: String,
    pub adjective: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsGemToken {
    pub name: String,
    pub plural_name: String,
    pub overwrite_solid: bool,
}
