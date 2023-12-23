use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use crate::parser::{
    creature_effect::TOKEN_MAP as CREATURE_EFFECT_TOKENS,
    serializer_helper, {clean_search_vec, Searchable},
};

use super::{phf_table::SYNDROME_TOKEN, tokens::SyndromeToken};

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Syndrome {
    /// Seen the \[SYN_IDENTIFIER:INEBRIATION\] tag in material_templates.txt
    #[serde(skip_serializing_if = "String::is_empty")]
    identifier: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    name: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    affected_classes: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    immune_classes: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    affected_creatures: Vec<(String, String)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    immune_creatures: Vec<(String, String)>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    classes: Vec<String>,

    /// Seen the \[SYN_CONCENTRATION_ADDED:100:1000\] tag in material_templates.txt
    #[serde(skip_serializing_if = "serializer_helper::min_max_is_zeroes")]
    concentration_added: [u32; 2],

    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<SyndromeToken>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    conditions: Vec<String>,
}

impl Syndrome {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from_name(name: &str) -> Self {
        Self {
            name: String::from(name),
            ..Self::default()
        }
    }
    pub fn parse_tag(&mut self, key: &str, value: &str) {
        if CREATURE_EFFECT_TOKENS.contains_key(key) {
            self.conditions.push(String::from(value));
            return;
        }
        if key == "CE" {
            debug!("Manual handling of CE tag: {}:{}", key, value);
            self.conditions.push(String::from(value));
            return;
        }

        let token = SYNDROME_TOKEN.get(key).unwrap_or(&SyndromeToken::Unknown);
        match token {
            SyndromeToken::Name => self.name = String::from(value),
            SyndromeToken::Identifier => self.identifier = String::from(value),
            SyndromeToken::AffectedClass => self.affected_classes.push(String::from(value)),
            SyndromeToken::ImmuneClass => self.immune_classes.push(String::from(value)),
            SyndromeToken::AffectedCreature => {
                let mut split = value.split(':');
                let creature = split.next().unwrap_or_default().trim();
                let caste = split.next().unwrap_or_default().trim();
                self.affected_creatures
                    .push((String::from(creature), String::from(caste)));
            }
            SyndromeToken::ImmuneCreature => {
                let mut split = value.split(':');
                let creature = split.next().unwrap_or_default().trim();
                let caste = split.next().unwrap_or_default().trim();
                self.immune_creatures
                    .push((String::from(creature), String::from(caste)));
            }
            SyndromeToken::ConcentrationAdded => {
                let mut split = value.split(':');
                let min = split.next().unwrap_or_default().trim();
                let max = split.next().unwrap_or_default().trim();
                self.concentration_added = [
                    min.parse::<u32>().unwrap_or_default(),
                    max.parse::<u32>().unwrap_or_default(),
                ];
            }
            SyndromeToken::Injected => self.tags.push(SyndromeToken::Injected),
            SyndromeToken::Contact => self.tags.push(SyndromeToken::Contact),
            SyndromeToken::Inhaled => self.tags.push(SyndromeToken::Inhaled),
            SyndromeToken::Ingested => self.tags.push(SyndromeToken::Ingested),

            SyndromeToken::Unknown => {
                warn!("Unknown syndrome token: {}", key);
            }
            SyndromeToken::Class => self.classes.push(String::from(value)),
            SyndromeToken::NoHospital => self.tags.push(SyndromeToken::NoHospital),
        }
    }
}

impl Searchable for Syndrome {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        // Identifier
        vec.push(self.identifier.clone());

        // Name
        vec.push(self.name.clone());

        // Affected classes
        for affected_class in &self.affected_classes {
            vec.push(affected_class.clone());
        }

        // Immune classes
        for immune_class in &self.immune_classes {
            vec.push(immune_class.clone());
        }

        // Affected creatures
        for (creature, caste) in &self.affected_creatures {
            vec.push(creature.clone());
            vec.push(caste.clone());
        }

        // Immune creatures
        for (creature, caste) in &self.immune_creatures {
            vec.push(creature.clone());
            vec.push(caste.clone());
        }

        // Classes
        for class in &self.classes {
            vec.push(class.clone());
        }

        // Conditions
        for condition in &self.conditions {
            vec.push(condition.clone());
        }

        clean_search_vec(vec.as_slice())
    }
}
