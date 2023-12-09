use crate::{creature::Creature, RawObject, VARIATION_ARGUMENT_RE};

use super::Requirements;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq, Default, ts_rs::TS)]
#[ts(export)]
pub enum CreatureVariationRule {
    #[default]
    Unknown,
    RemoveTag {
        tag: String,
        value: Option<String>,
    },
    NewTag {
        tag: String,
        value: Option<String>,
    },
    AddTag {
        tag: String,
        value: Option<String>,
    },
    ConvertTag {
        tag: String,
        target: Option<String>,
        replacement: Option<String>,
    },
    ConditionalNewTag {
        tag: String,
        value: Option<String>,
        argument_index: usize,
        argument_requirement: String,
    },
    ConditionalAddTag {
        tag: String,
        value: Option<String>,
        argument_index: usize,
        argument_requirement: String,
    },
    ConditionalRemoveTag {
        tag: String,
        value: Option<String>,
        argument_index: usize,
        argument_requirement: String,
    },
    ConditionalConvertTag {
        tag: String,
        target: Option<String>,
        replacement: Option<String>,
        argument_index: usize,
        argument_requirement: String,
    },
}

impl CreatureVariationRule {
    #[allow(clippy::too_many_lines)]
    #[must_use]
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
    pub fn with_args(&self, args: &[&str]) -> Self {
        // Short circuit if there are no arguments to replace.
        if args.is_empty() {
            return self.clone();
        }
        // We simply replace all instances of `!ARGn` with the corresponding argument.
        match self {
            CreatureVariationRule::RemoveTag { tag, value } => {
                // Only have the tag to replace.
                CreatureVariationRule::RemoveTag {
                    tag: replace_args_in_string(tag, args),
                    value: value
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                }
            }
            CreatureVariationRule::NewTag { tag, value }
            | CreatureVariationRule::AddTag { tag, value } => {
                // Have both the tag and the value to replace.
                CreatureVariationRule::NewTag {
                    tag: replace_args_in_string(tag, args),
                    value: value
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                }
            }
            CreatureVariationRule::ConvertTag {
                tag,
                target,
                replacement,
            } => {
                // Have the tag, target, and replacement to replace.
                CreatureVariationRule::ConvertTag {
                    tag: replace_args_in_string(tag, args),
                    target: target
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                    replacement: replacement
                        .as_ref()
                        .map(|value| replace_args_in_string(value, args)),
                }
            }
            CreatureVariationRule::ConditionalRemoveTag {
                tag,
                value,
                argument_requirement,
                argument_index,
            } => {
                // Have the tag and the argument requirement to replace.
                CreatureVariationRule::ConditionalRemoveTag {
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
            CreatureVariationRule::ConditionalNewTag {
                tag,
                value,
                argument_requirement,
                argument_index,
            }
            | CreatureVariationRule::ConditionalAddTag {
                tag,
                value,
                argument_requirement,
                argument_index,
            } => {
                // Have the tag, value, and argument requirement to replace.
                CreatureVariationRule::ConditionalNewTag {
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
            CreatureVariationRule::ConditionalConvertTag {
                tag,
                target,
                replacement,
                argument_index,
                argument_requirement,
            } => {
                // Have the tag, target, replacement, and argument requirement to replace.
                CreatureVariationRule::ConditionalConvertTag {
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
            CreatureVariationRule::Unknown => {
                // Unknown rules don't have anything to replace.
                CreatureVariationRule::Unknown
            }
        }
    }
    pub fn apply(&self, creature: &mut Creature, args: &[&str]) {
        match self.with_args(args) {
            CreatureVariationRule::RemoveTag { tag, .. } => {
                remove_tag(creature, &tag);
            }
            CreatureVariationRule::NewTag { tag, value }
            | CreatureVariationRule::AddTag { tag, value } => {
                apply_new_tag(creature, &tag, value.as_deref());
            }
            CreatureVariationRule::ConvertTag {
                tag,
                target,
                replacement,
            } => convert_tag(creature, &tag, target.as_deref(), replacement.as_deref()),
            CreatureVariationRule::ConditionalNewTag {
                tag,
                value,
                argument_index,
                argument_requirement,
            }
            | CreatureVariationRule::ConditionalAddTag {
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
            CreatureVariationRule::ConditionalRemoveTag {
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
            CreatureVariationRule::ConditionalConvertTag {
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
            CreatureVariationRule::Unknown => {}
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
        } else {
            tracing::trace!(
                "Converting tag {}:{} to {} on creature {}",
                tag,
                target,
                tag,
                creature.get_identifier()
            );
        }
    } else {
        tracing::trace!(
            "Converting tag {} to {} on creature {}",
            tag,
            replacement.unwrap_or_default(),
            creature.get_identifier()
        );
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
