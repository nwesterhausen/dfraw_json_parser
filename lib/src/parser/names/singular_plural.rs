use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// The name of a raw object with only singular and plural forms
#[derive(Serialize, Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase", rename = "SingPlurName")]
pub struct Name {
    singular: String,
    plural: String,
}

impl Name {
    /// Takes the arguments for a name and split ':' into sing, plural
    ///
    /// # Arguments
    ///
    /// * `value` - The value to parse into a Name struct (e.g. `singular:plural`)
    ///
    /// # Returns
    ///
    /// * The Name struct
    #[must_use]
    pub fn from_value(value: &str) -> Self {
        let arg_names: Vec<&str> = value.split(':').collect::<Vec<&str>>();

        let singular_name = *arg_names.first().unwrap_or(&"");
        let plural_name = *arg_names.get(1).unwrap_or(&"");

        if plural_name.eq("STP") {
            return Self {
                singular: String::from(singular_name),
                plural: String::from(singular_name),
            };
        }
        Self {
            singular: String::from(singular_name),
            plural: String::from(plural_name),
        }
    }
    /// Returns whether the name is empty
    ///
    /// # Returns
    ///
    /// * `true` if the name is empty, `false` otherwise.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.singular.is_empty() && self.plural.is_empty()
    }
    /// Returns the name as a vector of strings
    ///
    /// # Returns
    ///
    /// * `Vec<String>` - The name as a vector of strings
    #[must_use]
    pub fn as_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();
        if !self.singular.is_empty() {
            vec.push(self.singular.clone());
        }
        if !self.plural.is_empty() {
            vec.push(self.plural.clone());
        }
        vec
    }
    /// Create a new `SingularPluralName` instance.
    ///
    /// # Arguments
    ///
    /// * `name_singular` - The singular name.
    /// * `name_plural` - The plural name.
    ///
    /// # Returns
    ///
    /// A new `SingularPluralName` instance.
    #[must_use]
    pub fn new(name_singular: &str, name_plural: &str) -> Self {
        Self {
            singular: name_singular.to_string(),
            plural: name_plural.to_string(),
        }
    }
}
