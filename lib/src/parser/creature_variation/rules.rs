use crate::{creature::Creature, RawObject, VARIATION_ARGUMENT_RE};

use super::Requirements;

/// A variation rule for a creature.
#[derive(
    serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Default, specta::Type,
)]
pub enum CreatureVariationRule {
    /// An unknown rule.
    #[default]
    Unknown,
    /// Removes a tag from a creature.
    RemoveTag {
        /// The tag to remove.
        tag: String,
        /// The value to remove.
        value: Option<String>,
    },
    /// Adds a new tag to a creature.
    NewTag {
        /// The tag to add.
        tag: String,
        /// The value to add.
        value: Option<String>,
    },
    /// Adds a new tag to a creature.
    AddTag {
        /// The tag to add.
        tag: String,
        /// The value to add.
        value: Option<String>,
    },
    /// Converts a tag on a creature.
    ConvertTag {
        /// The tag to convert.
        tag: String,
        /// The target value to convert.
        target: Option<String>,
        /// The replacement value to convert to.
        replacement: Option<String>,
    },
    /// Adds a new tag to a creature if a condition is met.
    ConditionalNewTag {
        /// The tag to add.
        tag: String,
        /// The value to add.
        value: Option<String>,
        /// The index of the argument to check.
        argument_index: usize,
        /// The requirement for the argument.
        argument_requirement: String,
    },
    /// Adds a new tag to a creature if a condition is met.
    ConditionalAddTag {
        /// The tag to add.
        tag: String,
        /// The value to add.
        value: Option<String>,
        /// The index of the argument to check.
        argument_index: usize,
        /// The requirement for the argument.
        argument_requirement: String,
    },
    /// Removes a tag from a creature if a condition is met.
    ConditionalRemoveTag {
        /// The tag to remove.
        tag: String,
        /// The value to remove.
        value: Option<String>,
        /// The index of the argument to check.
        argument_index: usize,
        /// The requirement for the argument.
        argument_requirement: String,
    },
    /// Converts a tag on a creature if a condition is met.
    ConditionalConvertTag {
        /// The tag to convert.
        tag: String,
        /// The target value to convert.
        target: Option<String>,
        /// The replacement value to convert to.
        replacement: Option<String>,
        /// The index of the argument to check.
        argument_index: usize,
        /// The requirement for the argument.
        argument_requirement: String,
    },
}

impl CreatureVariationRule {
    /// Apply a set of arguments to the rule and get a rule that has the arguments applied.
    /// This will replace all instances of `!ARGn` with the corresponding argument.
    ///
    /// This returns a new rule with the arguments applied because we don't want to mutate the
    /// original rule (multiple creatures may use the same rule)
    ///
    /// ## Arguments
    ///
    /// * `args` - The arguments to apply to the rule.
    ///
    /// ## Returns
    ///
    /// * `CreatureVariationRule` - The rule with the arguments applied.
    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn with_args(&self, args: &[&str]) -> Self {
        // Short circuit if there are no arguments to replace.
        if args.is_empty() {
            return self.clone();
        }
        // We simply replace all instances of `!ARGn` with the corresponding argument.
        match self {
            Self::RemoveTag { tag, value } => {
                // Only have the tag to replace.
                Self::RemoveTag {
                    tag: replace_args_in_string(tag, args),
                    value: value
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                }
            }
            Self::NewTag { tag, value } | Self::AddTag { tag, value } => {
                // Have both the tag and the value to replace.
                Self::NewTag {
                    tag: replace_args_in_string(tag, args),
                    value: value
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                }
            }
            Self::ConvertTag {
                tag,
                target,
                replacement,
            } => {
                // Have the tag, target, and replacement to replace.
                Self::ConvertTag {
                    tag: replace_args_in_string(tag, args),
                    target: target
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                    replacement: replacement
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                }
            }
            Self::ConditionalRemoveTag {
                tag,
                value,
                argument_requirement,
                argument_index,
            } => {
                // Have the tag and the argument requirement to replace.
                Self::ConditionalRemoveTag {
                    tag: replace_args_in_string(tag, args),
                    value: value
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                    argument_requirement: String::from(
                        VARIATION_ARGUMENT_RE.replace_all(
                            argument_requirement.as_str(),
                            |caps: &regex::Captures| argument_as_string(caps, args),
                        ),
                    ),
                    argument_index: *argument_index,
                }
            }
            Self::ConditionalNewTag {
                tag,
                value,
                argument_requirement,
                argument_index,
            }
            | Self::ConditionalAddTag {
                tag,
                value,
                argument_requirement,
                argument_index,
            } => {
                // Have the tag, value, and argument requirement to replace.
                Self::ConditionalNewTag {
                    tag: replace_args_in_string(tag, args),
                    value: value
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                    argument_requirement: String::from(
                        VARIATION_ARGUMENT_RE.replace_all(
                            argument_requirement.as_str(),
                            |caps: &regex::Captures| argument_as_string(caps, args),
                        ),
                    ),
                    argument_index: *argument_index,
                }
            }
            Self::ConditionalConvertTag {
                tag,
                target,
                replacement,
                argument_index,
                argument_requirement,
            } => {
                // Have the tag, target, replacement, and argument requirement to replace.
                Self::ConditionalConvertTag {
                    tag: replace_args_in_string(tag, args),
                    target: target
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                    replacement: replacement
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                    argument_requirement: String::from(
                        VARIATION_ARGUMENT_RE.replace_all(
                            argument_requirement.as_str(),
                            |caps: &regex::Captures| argument_as_string(caps, args),
                        ),
                    ),
                    argument_index: *argument_index,
                }
            }
            Self::Unknown => {
                // Unknown rules don't have anything to replace.
                Self::Unknown
            }
        }
    }
    /// Apply the rule to a creature. This will apply the rule to the creature based on the arguments
    /// provided.
    ///
    /// # Arguments
    ///
    /// * `creature` - The creature to apply the rule to.
    /// * `args` - The arguments to apply to the rule.
    ///
    /// # Side Effects
    ///
    /// This will modify the creature provided.
    pub fn apply(&self, creature: &mut Creature, args: &[&str]) {
        match self.with_args(args) {
            Self::RemoveTag { tag, .. } => {
                remove_tag(creature, &tag);
            }
            Self::NewTag { tag, value } | Self::AddTag { tag, value } => {
                apply_new_tag(creature, &tag, value.as_deref());
            }
            Self::ConvertTag {
                tag,
                target,
                replacement,
            } => convert_tag(creature, &tag, target.as_deref(), replacement.as_deref()),
            Self::ConditionalNewTag {
                tag,
                value,
                argument_index,
                argument_requirement,
            }
            | Self::ConditionalAddTag {
                tag,
                value,
                argument_index,
                argument_requirement,
            } => {
                // Guard against out of bounds arguments.
                if args.len() < argument_index {
                    tracing::warn!(
                        "Creature Variation Argument index {} is out of bounds for {:?}",
                        argument_index,
                        args
                    );
                    return;
                }
                // Check if the argument matches the requirement.
                if let Some(argument_value) = args.get(argument_index - 1) {
                    if argument_value == &argument_requirement {
                        apply_new_tag(creature, &tag, value.as_deref());
                    }
                }
            }
            Self::ConditionalRemoveTag {
                tag,
                argument_index,
                argument_requirement,
                ..
            } => {
                // Guard against out of bounds arguments.
                if args.len() < argument_index {
                    tracing::warn!(
                        "Creature Variation Argument index {} is out of bounds for {:?}",
                        argument_index,
                        args
                    );
                    return;
                }
                // Check if the argument matches the requirement.
                if let Some(argument_value) = args.get(argument_index - 1) {
                    if argument_value == &argument_requirement {
                        remove_tag(creature, &tag);
                    }
                }
            }
            Self::ConditionalConvertTag {
                tag,
                target,
                replacement,
                argument_index,
                argument_requirement,
            } => {
                // Guard against out of bounds arguments.
                if args.len() < argument_index {
                    tracing::warn!(
                        "Creature Variation Argument index {} is out of bounds for {:?}",
                        argument_index,
                        args
                    );
                    return;
                }
                // Check if the argument matches the requirement.
                if let Some(argument_value) = args.get(argument_index - 1) {
                    if argument_value == &argument_requirement {
                        convert_tag(creature, &tag, target.as_deref(), replacement.as_deref());
                    }
                }
            }
            Self::Unknown => {}
        }
    }
}

/// Replaces all instances of `!ARGn` with the corresponding argument.
///
/// ## Arguments
///
/// * `string` - The string to replace the arguments in.
/// * `args` - The arguments to replace in the string.
///
/// ## Returns
///
/// * `String` - The string with the arguments replaced.
fn replace_args_in_string(string: &str, args: &[&str]) -> String {
    VARIATION_ARGUMENT_RE
        .replace_all(string, |caps: &regex::Captures| {
            argument_as_string(caps, args)
        })
        .to_string()
}

/// ADD or NEW tags can simply be applied by the parsing logic that already exists.
///
/// ## Arguments
///
/// * `creature` - The creature to apply the tag to.
/// * `tag` - The tag to apply.
/// * `value` - The value to apply to the tag.
fn apply_new_tag(creature: &mut Creature, tag: &str, value: Option<&str>) {
    (creature as &mut dyn Requirements).add_tag_and_value(tag, value.unwrap_or_default());
}

/// Removes a tag from a creature.
///
/// ## Arguments
///
/// * `creature` - The creature to remove the tag from.
/// * `tag` - The tag to remove.
fn remove_tag(creature: &mut Creature, tag: &str) {
    (creature as &mut dyn Requirements).remove_tag(tag);
}

/// Converts a tag on a creature.
fn convert_tag(
    creature: &mut Creature,
    tag: &str,
    target: Option<&str>,
    replacement: Option<&str>,
) {
    if let Some(target) = target {
        if let Some(replacement) = replacement {
            tracing::trace!(
                "Converting tag {}:{} to {}:{} on creature {}",
                tag,
                target,
                tag,
                replacement,
                creature.get_identifier()
            );
            // Convert the tag to the target value.
            (creature as &mut dyn Requirements).remove_tag_and_value(tag, target);
            (creature as &mut dyn Requirements).add_tag_and_value(tag, replacement);
        } else {
            tracing::trace!(
                "Converting tag {}:{} to {}:{} on creature {}",
                tag,
                target,
                replacement.unwrap_or_default(),
                target,
                creature.get_identifier(),
            );
            // Convert the tag to the target value.
            (creature as &mut dyn Requirements).remove_tag_and_value(tag, target);
            (creature as &mut dyn Requirements).add_tag_and_value(tag, target);
        }
    } else {
        tracing::trace!(
            "Converting tag {} to {} on creature {}",
            tag,
            replacement.unwrap_or_default(),
            creature.get_identifier()
        );
        // Convert the tag to the replacement value.
        (creature as &mut dyn Requirements).remove_tag(tag);
        (creature as &mut dyn Requirements).add_tag(replacement.unwrap_or_default());
    }
}

/// Returns the argument which matches the given capture group.
/// This expects you to be capturing based on the regex in `VARIATION_ARGUMENT_RE`.
///
/// That way it will match `!ARGn` and `!ARGnn` and `!ARGnnn` and replace with the corresponding
/// argument.
///
/// ## Arguments
///
/// * `caps` - The capture group to get the argument name from.
/// * `args` - The arguments to get the argument from.
///
/// ## Returns
///
/// * `String` - The argument which matches the given capture group.
fn argument_as_string(caps: &regex::Captures, args: &[&str]) -> String {
    if let Some(index) = caps.get(1) {
        let index = index.as_str().parse::<usize>().unwrap_or_default();
        if let Some(argument_value) = args.get(index - 1) {
            return (*argument_value).to_string();
        }
    }
    if let Some(arg) = caps.get(0) {
        tracing::warn!(
            "Creature Variation Argument is invalid. Argument captured: '{}'",
            arg.as_str()
        );
        return arg.as_str().to_string();
    }
    String::new()
}
