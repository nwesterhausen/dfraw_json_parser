use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::{helpers::build_object_id_from_pieces, ObjectType, RawMetadata, RawObject, Searchable};

use super::{Rule, Token, TOKEN_MAP};



#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct CreatureVariation {
    /// Common Raw file Things
    metadata: Option<RawMetadata>,
    identifier: String,
    object_id: String,

    /// Creature variations are basically just a set of simple tag actions which are applied to
    /// the creature which is being modified. The tags are applied in order EXCEPT for the convert
    /// tags which are applied in a reverse order.
    rules: Vec<Rule>,

    /// A creature variation can define any number of arguments which can be used in the rules.
    /// These arguments replace instances of `!ARGn` in the rules. Use `apply_arguments` to apply
    /// a set of arguments to a creature variation (and get a very specific variation back). Use
    /// `apply_to_creature` to apply the variation to a creature (it also takes arguments and will
    /// apply them to the variation before applying the variation to the creature).
    argument_count: usize,
}

impl CreatureVariation {
    /// Create a new creature variation with the given identifier.
    ///
    /// # Arguments
    ///
    /// * `metadata` - The metadata for the creature variation.
    /// * `identifier` - The identifier for the creature variation.
    ///
    /// # Returns
    ///
    /// A new creature variation with the given identifier.
    pub fn new(identifier: &str, metadata: &RawMetadata) -> Self {
        Self {
            metadata: Some(metadata.clone()),
            identifier: identifier.to_string(),
            object_id: build_object_id_from_pieces(
                metadata,
                identifier,
                &crate::ObjectType::CreatureVariation,
            ),
            rules: Vec::new(),
            argument_count: 0,
        }
    }

    pub fn empty() -> Self {
        Self {
            metadata: Some(
                RawMetadata::default()
                    .with_object_type(ObjectType::CreatureVariation)
                    .with_hidden(true),
            ),
            identifier: String::new(),
            object_id: String::new(),
            rules: Vec::new(),
            argument_count: 0,
        }
    }

    pub fn get_rules(&self) -> &Vec<Rule> {
        &self.rules
    }

    pub fn get_convert_rules(&self) -> Vec<&Rule> {
        self.rules
            .iter()
            .filter(|r| {
                matches!(
                    r,
                    Rule::ConvertTag { .. } | Rule::ConditionalConvertTag { .. }
                )
            })
            .collect()
    }

    /// Function to "clean" the creature. This is used to remove any empty list or strings,
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

        // Set metadata to None if it's hidden
        if let Some(metadata) = &cleaned.metadata {
            if metadata.is_hidden() {
                cleaned.metadata = None;
            }
        }

        cleaned
    }
}

#[typetag::serde]
impl RawObject for CreatureVariation {
    fn get_metadata(&self) -> RawMetadata {
        if let Some(metadata) = &self.metadata {
            metadata.clone()
        } else {
            warn!("Metadata is missing for {}", self.get_identifier());
            RawMetadata::default()
                .with_object_type(ObjectType::CreatureVariation)
                .with_hidden(true)
        }
    }

    fn get_identifier(&self) -> &str {
        self.identifier.as_str()
    }

    fn is_empty(&self) -> bool {
        self.rules.is_empty() && self.identifier.is_empty()
    }

    fn get_type(&self) -> &ObjectType {
        &ObjectType::CreatureVariation
    }

    fn clean_self(&mut self) {
        *self = self.cleaned();
    }

    #[allow(clippy::too_many_lines)]
    fn parse_tag(&mut self, key: &str, value: &str) {
        let Some(token) = TOKEN_MAP.get(key) else {
            warn!("Unknown tag in creature variation: {}", key);
            return;
        };

        // We need to split up the value string into it's parts.
        //
        // Add/new tags [CV_TAG:value] (value is optional)
        // Add/new with conditions [CV_TAG:argument_index:argument_value:value(s)] (value is optional)
        // Remove tags [CV_TAG] (value is optional)
        // Remove with conditions [CV_TAG:argument_index:argument_value:value(s)] (value is optional)
        // Convert tags
        //  [CV_CONVERT_TAG]
        //      [CVCT_MASTER:tag:value]
        //      [CVCT_TARGET:tag:value(s)]
        //      [CVCT_REPLACEMENT:tag:value(s)]
        // Convert with conditions
        //  [CV_CONVERT_CTAG:argument_index:argument_value]
        //      [CVCT_MASTER:tag:argument_index:argument_value]
        //      [CVCT_TARGET:tag:argument_index:argument_value(s)]
        //      [CVCT_REPLACEMENT:tag:argument_index:argument_value(s)]

        let mut parts = value.split(':');

        match token {
            Token::AddTag | Token::NewTag => {
                // Parts can be any number of strings long, but the first part is always the tag
                let tag = parts.next().unwrap_or_default().to_string();
                // For Add and New we just want to squish all the remaining parts together for value
                let value = parts.collect::<Vec<&str>>().join(":");
                let value = if value.is_empty() { None } else { Some(value) };

                self.rules.push(Rule::AddTag { tag, value });
            }
            Token::ConditionalAddTag | Token::ConditionalNewTag => {
                // For conditional tags, the first part is the argument index, the second part is the
                // argument value, the third part is the tag, and the remaining parts are the value.
                let argument_index = parts.next().unwrap_or_default();
                let Ok(argument_index) = argument_index.parse::<usize>() else {
                    warn!(
                        "Invalid index argument '{}' for conditional tag: {}",
                        argument_index, key
                    );
                    return;
                };
                let argument_requirement = parts.next().unwrap_or_default().to_string();
                let tag = parts.next().unwrap_or_default().to_string();
                let value = parts.collect::<Vec<&str>>().join(":");
                let value = if value.is_empty() { None } else { Some(value) };

                self.rules.push(Rule::ConditionalAddTag {
                    argument_index,
                    tag,
                    value,
                    argument_requirement,
                });
            }
            Token::RemoveTag => {
                // Parts can be any number of strings long, but the first part is always the tag
                let tag = parts.next().unwrap_or_default().to_string();
                // For Add and New we just want to squish all the remaining parts together for value
                let value = parts.collect::<Vec<&str>>().join(":");
                let value = if value.is_empty() { None } else { Some(value) };

                self.rules.push(Rule::RemoveTag { tag, value });
            }
            Token::ConditionalRemoveTag => {
                // For conditional tags, the first part is the argument index, the second part is the
                // argument value, the third part is the tag, and the remaining parts are the value.
                let argument_index = parts.next().unwrap_or_default();
                let Ok(argument_index) = argument_index.parse::<usize>() else {
                    warn!(
                        "Invalid index argument '{}' for conditional tag: {}",
                        argument_index, key
                    );
                    return;
                };
                let argument_requirement = parts.next().unwrap_or_default().to_string();
                let tag = parts.next().unwrap_or_default().to_string();
                let value = parts.collect::<Vec<&str>>().join(":");
                let value = if value.is_empty() { None } else { Some(value) };

                self.rules.push(Rule::ConditionalRemoveTag {
                    tag: tag.to_string(),
                    value,
                    argument_index,
                    argument_requirement,
                });
            }
            Token::ConvertTag => {
                // Convert tag actually just tells us that we're starting a convert tag rule.
                self.rules.push(Rule::ConvertTag {
                    tag: String::new(),
                    replacement: None,
                    target: None,
                });
            }
            Token::ConditionalConvertTag => {
                // For conditional tags, the first part is the argument index, the second part is the
                // argument value, the third part is the tag, and the remaining parts are the value.
                let argument_index = parts.next().unwrap_or_default();
                let Ok(argument_index) = argument_index.parse::<usize>() else {
                    warn!(
                        "Invalid index argument '{}' for conditional tag: {}",
                        argument_index, key
                    );
                    return;
                };
                let argument_requirement = parts.next().unwrap_or_default().to_string();

                self.rules.push(Rule::ConditionalConvertTag {
                    argument_index,
                    argument_requirement,
                    tag: String::new(),
                    replacement: None,
                    target: None,
                });
            }
            Token::ConvertTagMaster => {
                // Grab the last rule and set the master (i.e. the target tag)
                let Some(rule) = self.rules.last_mut() else {
                    warn!("No rule to add master tag to for tag: {}", key);
                    return;
                };

                let Some(new_tag) = parts.next() else {
                    warn!("No target tag for convert tag: {}", key);
                    return;
                };

                match rule {
                    Rule::ConvertTag { tag, .. } | Rule::ConditionalConvertTag { tag, .. } => {
                        *tag = String::from(new_tag);
                    }
                    Rule::Unknown => {
                        warn!("No rule to add master tag to for tag: {}", key);
                    }
                    _ => {
                        warn!("Invalid rule to add master tag to for tag: {}", key);
                    }
                }
            }
            Token::ConvertTagTarget => {
                // Grab the last rule and set the target (i.e. the tag to convert)
                let Some(rule) = self.rules.last_mut() else {
                    warn!("No rule to add target tag to for tag: {}", key);
                    return;
                };

                let Some(new_target) = parts.next() else {
                    warn!("No target tag for convert tag: {}", key);
                    return;
                };

                match rule {
                    Rule::ConvertTag { target, .. }
                    | Rule::ConditionalConvertTag { target, .. } => {
                        *target = Some(String::from(new_target));
                    }
                    Rule::Unknown => {
                        warn!("No rule to add target tag to for tag: {}", key);
                    }
                    _ => {
                        warn!("Invalid rule to add target tag to for tag: {}", key);
                    }
                }
            }
            Token::ConvertTagReplacement => {
                // Grab the last rule and set the replacement (i.e. the tag to convert to)
                let Some(rule) = self.rules.last_mut() else {
                    warn!("No rule to add replacement tag to for tag: {}", key);
                    return;
                };

                let Some(new_replacement) = parts.next() else {
                    warn!("No replacement tag for convert tag: {}", key);
                    return;
                };

                match rule {
                    Rule::ConvertTag { replacement, .. }
                    | Rule::ConditionalConvertTag { replacement, .. } => {
                        *replacement = Some(String::from(new_replacement));
                    }
                    Rule::Unknown => {
                        warn!("No rule to add replacement tag to for tag: {}", key);
                    }
                    _ => {
                        warn!("Invalid rule to add replacement tag to for tag: {}", key);
                    }
                }
            }
            Token::Unknown => {
                warn!("Unknown tag in creature variation: {}", key);
            }
        }
    }

    fn get_object_id(&self) -> &str {
        self.object_id.as_str()
    }

    fn get_name(&self) -> &str {
        self.identifier.as_str()
    }
}

impl Searchable for CreatureVariation {
    fn get_search_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();

        vec.push(self.identifier.clone());
        vec.push(self.object_id.clone());

        // Add the tags from the rules
        vec.extend(
            self.rules
                .iter()
                .map(|r| match r {
                    Rule::AddTag { tag, .. }
                    | Rule::ConditionalAddTag { tag, .. }
                    | Rule::RemoveTag { tag, .. }
                    | Rule::ConditionalRemoveTag { tag, .. }
                    | Rule::NewTag { tag, .. }
                    | Rule::ConditionalNewTag { tag, .. }
                    | Rule::ConvertTag { tag, .. }
                    | Rule::ConditionalConvertTag { tag, .. } => tag.clone(),
                    Rule::Unknown => String::new(),
                })
                .filter(|s| !s.is_empty())
                .collect::<Vec<String>>(),
        );

        vec
    }
}
