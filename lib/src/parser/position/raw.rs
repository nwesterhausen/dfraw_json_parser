use serde::{Deserialize, Serialize};

use crate::parser::{color::Color, names::SingPlurName, serializer_helper};

use super::tokens::PositionToken;

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    identifier: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    allowed_classes: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    allowed_creatures: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    appointed_by: String,
    #[serde(skip_serializing_if = "Color::is_default")]
    color: Color,
    #[serde(skip_serializing_if = "String::is_empty")]
    commander: String,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u32")]
    demand_max: u32,
    #[serde(skip_serializing_if = "String::is_empty")]
    execution_skill: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    gender: String,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u32")]
    land_holder: u32,
    #[serde(skip_serializing_if = "String::is_empty")]
    land_name: String,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u32")]
    mandate_max: u32,
    #[serde(skip_serializing_if = "SingPlurName::is_empty")]
    name: SingPlurName,
    #[serde(skip_serializing_if = "SingPlurName::is_empty")]
    name_male: SingPlurName,
    #[serde(skip_serializing_if = "SingPlurName::is_empty")]
    name_female: SingPlurName,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_i32")]
    number: i32, //set -1 for AS_NEEDED
    #[serde(skip_serializing_if = "serializer_helper::is_zero_i32")]
    precedence: i32, //set -1 for NONE
    #[serde(skip_serializing_if = "Vec::is_empty")]
    rejected_classes: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    rejected_creatures: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    replaced_by: String,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u32")]
    required_bedroom: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u32")]
    required_boxes: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u32")]
    required_cabinets: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u32")]
    required_dining: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u32")]
    required_office: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u32")]
    required_racks: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u32")]
    required_stands: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u32")]
    required_tomb: u32,
    #[serde(skip_serializing_if = "serializer_helper::is_zero_u32")]
    requires_population: u32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    responsibilities: Vec<String>,
    #[serde(skip_serializing_if = "SingPlurName::is_empty")]
    spouse: SingPlurName,
    #[serde(skip_serializing_if = "SingPlurName::is_empty")]
    spouse_female: SingPlurName,
    #[serde(skip_serializing_if = "SingPlurName::is_empty")]
    spouse_male: SingPlurName,
    #[serde(skip_serializing_if = "String::is_empty")]
    squad: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    succession: String,

    tags: Vec<PositionToken>,
}

impl Position {
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
            ..Default::default()
        }
    }
    pub fn parse_tag(&mut self, key: &PositionToken, value: &str) {
        match key {
            PositionToken::AllowedClass => self.allowed_classes.push(value.to_string()),
            PositionToken::AllowedCreature => self.allowed_creatures.push(value.to_string()),
            PositionToken::AppointedBy => self.appointed_by = value.to_string(),
            PositionToken::Color => self.color = Color::from_value(value),
            PositionToken::Commander => self.commander = value.to_string(),
            PositionToken::DemandMax => self.demand_max = value.parse().unwrap_or_default(),
            PositionToken::ExecutionSkill => self.execution_skill = value.to_string(),
            PositionToken::Gender => self.gender = value.to_string(),
            PositionToken::LandHolder => self.land_holder = value.parse().unwrap_or_default(),
            PositionToken::LandName => self.land_name = value.to_string(),
            PositionToken::MandateMax => self.mandate_max = value.parse().unwrap_or_default(),
            PositionToken::Name => self.name = SingPlurName::from_value(value),
            PositionToken::Spouse => self.spouse = SingPlurName::from_value(value),
            PositionToken::NameFemale => self.name_female = SingPlurName::from_value(value),
            PositionToken::SpouseFemale => self.spouse_female = SingPlurName::from_value(value),
            PositionToken::NameMale => self.name_male = SingPlurName::from_value(value),
            PositionToken::SpouseMale => self.spouse_male = SingPlurName::from_value(value),
            PositionToken::Number => self.number = value.parse().unwrap_or_default(),
            PositionToken::Precedence => self.precedence = value.parse().unwrap_or_default(),
            PositionToken::RejectedClass => self.rejected_classes.push(value.to_string()),
            PositionToken::RejectedCreature => self.rejected_creatures.push(value.to_string()),
            PositionToken::ReplacedBy => self.replaced_by = value.to_string(),
            PositionToken::RequiredBedroom => {
                self.required_bedroom = value.parse().unwrap_or_default();
            }
            PositionToken::RequiredBoxes => self.required_boxes = value.parse().unwrap_or_default(),
            PositionToken::RequiredCabinets => {
                self.required_cabinets = value.parse().unwrap_or_default();
            }
            PositionToken::RequiredDining => {
                self.required_dining = value.parse().unwrap_or_default();
            }
            PositionToken::RequiredOffice => {
                self.required_office = value.parse().unwrap_or_default();
            }
            PositionToken::RequiredRacks => self.required_racks = value.parse().unwrap_or_default(),
            PositionToken::RequiredStands => {
                self.required_stands = value.parse().unwrap_or_default();
            }
            PositionToken::RequiredTomb => self.required_tomb = value.parse().unwrap_or_default(),
            PositionToken::RequiresPopulation => {
                self.requires_population = value.parse().unwrap_or_default();
            }
            PositionToken::Responsibility => self.responsibilities.push(value.to_string()),
            PositionToken::Squad => self.squad = value.to_string(),
            PositionToken::Succession => self.succession = value.to_string(),
            _ => self.tags.push(key.clone()),
        }
    }
}
