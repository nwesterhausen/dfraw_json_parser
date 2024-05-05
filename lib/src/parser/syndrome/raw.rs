use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use crate::parser::{
    creature_effect::TOKEN_MAP as CREATURE_EFFECT_TOKENS,
    serializer_helper, {clean_search_vec, Searchable},
};

use super::{phf_table::SYNDROME_TOKEN, tokens::SyndromeToken};

#[derive(Serialize, Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Syndrome {
    /// Seen the \[SYN_IDENTIFIER:INEBRIATION\] tag in material_templates.txt
    identifier: Option<String>,
    name: Option<String>,

    affected_classes: Option<Vec<String>>,
    immune_classes: Option<Vec<String>>,
    affected_creatures: Option<Vec<(String, String)>>,
    immune_creatures: Option<Vec<(String, String)>>,
    classes: Option<Vec<String>>,

    /// Seen the \[SYN_CONCENTRATION_ADDED:100:1000\] tag in material_templates.txt
    /// default is 0:0
    concentration_added: Option<[u32; 2]>,

    tags: Option<Vec<SyndromeToken>>,

    conditions: Option<Vec<String>>,
}

impl Syndrome {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn from_name(name: &str) -> Self {
        Self {
            name: Some(String::from(name)),
            ..Self::default()
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

        if let Some(identifier) = &cleaned.identifier {
            if identifier.is_empty() {
                cleaned.identifier = None;
            }
        }
        if let Some(name) = &cleaned.name {
            if name.is_empty() {
                cleaned.name = None;
            }
        }
        if let Some(affected_classes) = &cleaned.affected_classes {
            if affected_classes.is_empty() {
                cleaned.affected_classes = None;
            }
        }
        if let Some(immune_classes) = &cleaned.immune_classes {
            if immune_classes.is_empty() {
                cleaned.immune_classes = None;
            }
        }
        if let Some(affected_creatures) = &cleaned.affected_creatures {
            if affected_creatures.is_empty() {
                cleaned.affected_creatures = None;
            }
        }
        if let Some(immune_creatures) = &cleaned.immune_creatures {
            if immune_creatures.is_empty() {
                cleaned.immune_creatures = None;
            }
        }
        if let Some(classes) = &cleaned.classes {
            if classes.is_empty() {
                cleaned.classes = None;
            }
        }
        if serializer_helper::min_max_is_zeroes(&cleaned.concentration_added) {
            cleaned.concentration_added = None;
        }
        if let Some(tags) = &cleaned.tags {
            if tags.is_empty() {
                cleaned.tags = None;
            }
        }
        if let Some(conditions) = &cleaned.conditions {
            if conditions.is_empty() {
                cleaned.conditions = None;
            }
        }

        cleaned
    }

    #[allow(clippy::too_many_lines)]
    pub fn parse_tag(&mut self, key: &str, value: &str) {
        if CREATURE_EFFECT_TOKENS.contains_key(key) {
            if self.conditions.is_none() {
                self.conditions = Some(Vec::new());
            }

            if let Some(conditions) = self.conditions.as_mut() {
                conditions.push(String::from(value));
            }
            return;
        }
        if key == "CE" {
            debug!("Manual handling of CE tag: {}:{}", key, value);
            if self.conditions.is_none() {
                self.conditions = Some(Vec::new());
            }

            if let Some(conditions) = self.conditions.as_mut() {
                conditions.push(String::from(value));
            }
            return;
        }

        let token = SYNDROME_TOKEN.get(key).unwrap_or(&SyndromeToken::Unknown);
        match token {
            SyndromeToken::Name => self.name = Some(String::from(value)),
            SyndromeToken::Identifier => self.identifier = Some(String::from(value)),
            SyndromeToken::AffectedClass => {
                if self.affected_classes.is_none() {
                    self.affected_classes = Some(Vec::new());
                }
                if let Some(affected_classes) = self.affected_classes.as_mut() {
                    affected_classes.push(String::from(value));
                }
            }
            SyndromeToken::ImmuneClass => {
                if self.immune_classes.is_none() {
                    self.immune_classes = Some(Vec::new());
                }
                if let Some(immune_classes) = self.immune_classes.as_mut() {
                    immune_classes.push(String::from(value));
                }
            }
            SyndromeToken::AffectedCreature => {
                if self.affected_creatures.is_none() {
                    self.affected_creatures = Some(Vec::new());
                }

                let mut split = value.split(':');
                let creature = split.next().unwrap_or_default().trim();
                let caste = split.next().unwrap_or_default().trim();

                if let Some(affected_creatures) = self.affected_creatures.as_mut() {
                    affected_creatures.push((String::from(creature), String::from(caste)));
                }
            }
            SyndromeToken::ImmuneCreature => {
                if self.immune_creatures.is_none() {
                    self.immune_creatures = Some(Vec::new());
                }

                let mut split = value.split(':');
                let creature = split.next().unwrap_or_default().trim();
                let caste = split.next().unwrap_or_default().trim();

                if let Some(immune_creatures) = self.immune_creatures.as_mut() {
                    immune_creatures.push((String::from(creature), String::from(caste)));
                }
            }
            SyndromeToken::ConcentrationAdded => {
                let mut split = value.split(':');
                let min = split.next().unwrap_or_default().trim();
                let max = split.next().unwrap_or_default().trim();
                self.concentration_added = Some([
                    min.parse::<u32>().unwrap_or_default(),
                    max.parse::<u32>().unwrap_or_default(),
                ]);
            }
            SyndromeToken::Injected => {
                if self.tags.is_none() {
                    self.tags = Some(Vec::new());
                }
                if let Some(tags) = self.tags.as_mut() {
                    tags.push(SyndromeToken::Injected);
                }
            }
            SyndromeToken::Contact => {
                if self.tags.is_none() {
                    self.tags = Some(Vec::new());
                }
                if let Some(tags) = self.tags.as_mut() {
                    tags.push(SyndromeToken::Contact);
                }
            }
            SyndromeToken::Inhaled => {
                if self.tags.is_none() {
                    self.tags = Some(Vec::new());
                }
                if let Some(tags) = self.tags.as_mut() {
                    tags.push(SyndromeToken::Inhaled);
                }
            }
            SyndromeToken::Ingested => {
                if self.tags.is_none() {
                    self.tags = Some(Vec::new());
                }
                if let Some(tags) = self.tags.as_mut() {
                    tags.push(SyndromeToken::Ingested);
                }
            }
            SyndromeToken::Unknown => {
                warn!("Unknown syndrome token: {}", key);
            }
            SyndromeToken::Class => {
                if self.classes.is_none() {
                    self.classes = Some(Vec::new());
                }
                if let Some(classes) = self.classes.as_mut() {
                    classes.push(String::from(value));
                }
            }
            SyndromeToken::NoHospital => {
                if self.tags.is_none() {
                    self.tags = Some(Vec::new());
                }
                if let Some(tags) = self.tags.as_mut() {
                    tags.push(SyndromeToken::NoHospital);
                }
            }
        }
    }
}

impl Searchable for Syndrome {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        // Identifier
        if let Some(identifier) = &self.identifier {
            vec.push(identifier.clone());
        }

        // Name
        if let Some(name) = &self.name {
            vec.push(name.clone());
        }

        // Affected classes
        if let Some(affected_classes) = &self.affected_classes {
            for affected_class in affected_classes {
                vec.push(affected_class.clone());
            }
        }

        // Immune classes
        if let Some(immune_classes) = &self.immune_classes {
            for immune_class in immune_classes {
                vec.push(immune_class.clone());
            }
        }

        // Affected creatures
        if let Some(affected_creatures) = &self.affected_creatures {
            for (creature, caste) in affected_creatures {
                vec.push(creature.clone());
                vec.push(caste.clone());
            }
        }

        // Immune creatures
        if let Some(immune_creatures) = &self.immune_creatures {
            for (creature, caste) in immune_creatures {
                vec.push(creature.clone());
                vec.push(caste.clone());
            }
        }

        // Classes
        if let Some(classes) = &self.classes {
            for class in classes {
                vec.push(class.clone());
            }
        }

        // Conditions
        if let Some(conditions) = &self.conditions {
            for condition in conditions {
                vec.push(condition.clone());
            }
        }

        clean_search_vec(vec.as_slice())
    }
}
