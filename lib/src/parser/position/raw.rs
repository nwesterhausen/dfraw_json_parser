use serde::{Deserialize, Serialize};

use crate::parser::{serializer_helper, Color, SingPlurName};

use super::tokens::PositionToken;

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    identifier: String,

    allowed_classes: Option<Vec<String>>,
    allowed_creatures: Option<Vec<String>>,
    appointed_by: Option<String>,
    color: Option<Color>,
    commander: Option<String>,
    demand_max: Option<u32>,
    execution_skill: Option<String>,
    gender: Option<String>,
    land_holder: Option<u32>,
    land_name: Option<String>,
    mandate_max: Option<u32>,
    name: Option<SingPlurName>,
    name_male: Option<SingPlurName>,
    name_female: Option<SingPlurName>,

    number: Option<i32>,     //set -1 for AS_NEEDED
    precedence: Option<i32>, //set -1 for NONE
    rejected_classes: Option<Vec<String>>,
    rejected_creatures: Option<Vec<String>>,
    replaced_by: Option<String>,

    required_bedroom: Option<u32>,
    required_boxes: Option<u32>,
    required_cabinets: Option<u32>,
    required_dining: Option<u32>,
    required_office: Option<u32>,
    required_racks: Option<u32>,
    required_stands: Option<u32>,
    required_tomb: Option<u32>,
    requires_population: Option<u32>,

    responsibilities: Option<Vec<String>>,
    spouse: Option<SingPlurName>,
    spouse_female: Option<SingPlurName>,
    spouse_male: Option<SingPlurName>,
    squad: Option<String>,
    succession: Option<String>,

    tags: Vec<PositionToken>,
}

impl Position {
    pub fn new(identifier: String) -> Self {
        Self {
            identifier,
            ..Default::default()
        }
    }

    /// Function to "clean" the raw. This is used to remove any empty list or strings,
    /// and to remove any default values. By "removing" it means setting the value to None.
    ///
    /// This also will remove the metadata if is_metadata_hidden is true.
    ///
    /// Steps for all "Option" fields:
    /// - Set any metadata to None if is_metadata_hidden is true.
    /// - Set any empty string to None.
    /// - Set any empty list to None.
    /// - Set any default values to None.
    pub fn cleaned(&self) -> Self {
        let mut cleaned = self.clone();

        if let Some(allowed_classes) = &cleaned.allowed_classes {
            if allowed_classes.is_empty() {
                cleaned.allowed_classes = None;
            }
        }

        if let Some(allowed_creatures) = &cleaned.allowed_creatures {
            if allowed_creatures.is_empty() {
                cleaned.allowed_creatures = None;
            }
        }
        if let Some(rejected_classes) = &cleaned.rejected_classes {
            if rejected_classes.is_empty() {
                cleaned.rejected_classes = None;
            }
        }
        if let Some(appointed_by) = &cleaned.appointed_by {
            if appointed_by.is_empty() {
                cleaned.appointed_by = None;
            }
        }
        if let Some(color) = &cleaned.color {
            if color.is_default() {
                cleaned.color = None;
            }
        }
        if let Some(commander) = &cleaned.commander {
            if commander.is_empty() {
                cleaned.commander = None;
            }
        }
        if serializer_helper::is_zero_u32(&cleaned.demand_max) {
            cleaned.demand_max = None;
        }
        if let Some(execution_skill) = &cleaned.execution_skill {
            if execution_skill.is_empty() {
                cleaned.execution_skill = None;
            }
        }
        if let Some(gender) = &cleaned.gender {
            if gender.is_empty() {
                cleaned.gender = None;
            }
        }
        if serializer_helper::is_zero_u32(&cleaned.land_holder) {
            cleaned.land_holder = None;
        }
        if let Some(land_name) = &cleaned.land_name {
            if land_name.is_empty() {
                cleaned.land_name = None;
            }
        }
        if serializer_helper::is_zero_u32(&cleaned.mandate_max) {
            cleaned.mandate_max = None;
        }
        if let Some(name) = &cleaned.name {
            if name.is_empty() {
                cleaned.name = None;
            }
        }
        if let Some(name_male) = &cleaned.name_male {
            if name_male.is_empty() {
                cleaned.name_male = None;
            }
        }
        if let Some(name_female) = &cleaned.name_female {
            if name_female.is_empty() {
                cleaned.name_female = None;
            }
        }
        if serializer_helper::is_zero_i32(&cleaned.number) {
            cleaned.number = None;
        }
        if serializer_helper::is_zero_i32(&cleaned.precedence) {
            cleaned.precedence = None;
        }
        if let Some(rejected_creatures) = &cleaned.rejected_creatures {
            if rejected_creatures.is_empty() {
                cleaned.rejected_creatures = None;
            }
        }
        if let Some(replaced_by) = &cleaned.replaced_by {
            if replaced_by.is_empty() {
                cleaned.replaced_by = None;
            }
        }
        if serializer_helper::is_zero_u32(&cleaned.required_bedroom) {
            cleaned.required_bedroom = None;
        }
        if serializer_helper::is_zero_u32(&cleaned.required_boxes) {
            cleaned.required_boxes = None;
        }
        if serializer_helper::is_zero_u32(&cleaned.required_cabinets) {
            cleaned.required_cabinets = None;
        }
        if serializer_helper::is_zero_u32(&cleaned.required_dining) {
            cleaned.required_dining = None;
        }
        if serializer_helper::is_zero_u32(&cleaned.required_office) {
            cleaned.required_office = None;
        }
        if serializer_helper::is_zero_u32(&cleaned.required_racks) {
            cleaned.required_racks = None;
        }
        if serializer_helper::is_zero_u32(&cleaned.required_stands) {
            cleaned.required_stands = None;
        }
        if serializer_helper::is_zero_u32(&cleaned.required_tomb) {
            cleaned.required_tomb = None;
        }
        if serializer_helper::is_zero_u32(&cleaned.requires_population) {
            cleaned.requires_population = None;
        }
        if let Some(responsibilities) = &cleaned.responsibilities {
            if responsibilities.is_empty() {
                cleaned.responsibilities = None;
            }
        }
        if let Some(spouse) = &cleaned.spouse {
            if spouse.is_empty() {
                cleaned.spouse = None;
            }
        }
        if let Some(spouse_female) = &cleaned.spouse_female {
            if spouse_female.is_empty() {
                cleaned.spouse_female = None;
            }
        }
        if let Some(spouse_male) = &cleaned.spouse_male {
            if spouse_male.is_empty() {
                cleaned.spouse_male = None;
            }
        }
        if let Some(squad) = &cleaned.squad {
            if squad.is_empty() {
                cleaned.squad = None;
            }
        }
        if let Some(succession) = &cleaned.succession {
            if succession.is_empty() {
                cleaned.succession = None;
            }
        }

        cleaned
    }
    pub fn parse_tag(&mut self, key: &PositionToken, value: &str) {
        match key {
            PositionToken::AllowedClass => {
                if self.allowed_classes.is_none() {
                    self.allowed_classes = Some(Vec::new());
                }
                if let Some(allowed_classes) = self.allowed_classes.as_mut() {
                    allowed_classes.push(value.to_string());
                }
            }
            PositionToken::AllowedCreature => {
                if self.allowed_creatures.is_none() {
                    self.allowed_creatures = Some(Vec::new());
                }
                if let Some(allowed_creatures) = self.allowed_creatures.as_mut() {
                    allowed_creatures.push(value.to_string());
                }
            }
            PositionToken::AppointedBy => self.appointed_by = Some(value.to_string()),
            PositionToken::Color => self.color = Some(Color::from_value(value)),
            PositionToken::Commander => self.commander = Some(value.to_string()),
            PositionToken::DemandMax => self.demand_max = Some(value.parse().unwrap_or_default()),
            PositionToken::ExecutionSkill => self.execution_skill = Some(value.to_string()),
            PositionToken::Gender => self.gender = Some(value.to_string()),
            PositionToken::LandHolder => self.land_holder = Some(value.parse().unwrap_or_default()),
            PositionToken::LandName => self.land_name = Some(value.to_string()),
            PositionToken::MandateMax => self.mandate_max = Some(value.parse().unwrap_or_default()),
            PositionToken::Name => self.name = Some(SingPlurName::from_value(value)),
            PositionToken::Spouse => self.spouse = Some(SingPlurName::from_value(value)),
            PositionToken::NameFemale => self.name_female = Some(SingPlurName::from_value(value)),
            PositionToken::SpouseFemale => {
                self.spouse_female = Some(SingPlurName::from_value(value))
            }
            PositionToken::NameMale => self.name_male = Some(SingPlurName::from_value(value)),
            PositionToken::SpouseMale => self.spouse_male = Some(SingPlurName::from_value(value)),
            PositionToken::Number => self.number = Some(value.parse().unwrap_or_default()),
            PositionToken::Precedence => self.precedence = Some(value.parse().unwrap_or_default()),
            PositionToken::RejectedClass => {
                if self.rejected_classes.is_none() {
                    self.rejected_classes = Some(Vec::new());
                }
                if let Some(rejected_classes) = self.rejected_classes.as_mut() {
                    rejected_classes.push(value.to_string());
                }
            }
            PositionToken::RejectedCreature => {
                if self.rejected_creatures.is_none() {
                    self.rejected_creatures = Some(Vec::new());
                }
                if let Some(rejected_creatures) = self.rejected_creatures.as_mut() {
                    rejected_creatures.push(value.to_string());
                }
            }
            PositionToken::ReplacedBy => self.replaced_by = Some(value.to_string()),
            PositionToken::RequiredBedroom => {
                self.required_bedroom = Some(value.parse().unwrap_or_default());
            }
            PositionToken::RequiredBoxes => {
                self.required_boxes = Some(value.parse().unwrap_or_default())
            }
            PositionToken::RequiredCabinets => {
                self.required_cabinets = Some(value.parse().unwrap_or_default());
            }
            PositionToken::RequiredDining => {
                self.required_dining = Some(value.parse().unwrap_or_default());
            }
            PositionToken::RequiredOffice => {
                self.required_office = Some(value.parse().unwrap_or_default());
            }
            PositionToken::RequiredRacks => {
                self.required_racks = Some(value.parse().unwrap_or_default())
            }
            PositionToken::RequiredStands => {
                self.required_stands = Some(value.parse().unwrap_or_default());
            }
            PositionToken::RequiredTomb => {
                self.required_tomb = Some(value.parse().unwrap_or_default())
            }
            PositionToken::RequiresPopulation => {
                self.requires_population = Some(value.parse().unwrap_or_default());
            }
            PositionToken::Responsibility => {
                if self.responsibilities.is_none() {
                    self.responsibilities = Some(Vec::new());
                }
                if let Some(responsibilities) = self.responsibilities.as_mut() {
                    responsibilities.push(value.to_string());
                }
            }
            PositionToken::Squad => self.squad = Some(value.to_string()),
            PositionToken::Succession => self.succession = Some(value.to_string()),
            _ => self.tags.push(key.clone()),
        }
    }
}
