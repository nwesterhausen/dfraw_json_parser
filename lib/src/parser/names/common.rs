use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// A name with a singular, plural, and adjective form
#[derive(Serialize, Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    singular: String,
    plural: String,
    adjective: String,
}

impl Name {
    /// Takes the arguments for a name and split ':' into sing, plural, adjective
    ///
    /// # Arguments
    ///
    /// * `value` - The value to parse into a Name struct (e.g. `singular:plural:adjective`)
    ///
    /// # Returns
    ///
    /// * The Name struct
    #[must_use]
    pub fn from_value(value: &str) -> Self {
        let arg_names: Vec<&str> = value.split(':').collect::<Vec<&str>>();

        let singular_name = *arg_names.first().unwrap_or(&"");
        let plural_name = *arg_names.get(1).unwrap_or(&"");
        let adjective_name = *arg_names.get(2).unwrap_or(&"");

        Self {
            singular: String::from(singular_name),
            plural: String::from(plural_name),
            adjective: String::from(adjective_name),
        }
    }
    /// Returns whether the name is empty
    ///
    /// # Returns
    ///
    /// * `true` if the name is empty, `false` otherwise.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.singular.is_empty() && self.plural.is_empty() && self.adjective.is_empty()
    }
    /// Sets the singular name
    ///
    /// # Arguments
    ///
    /// * `name` - The name to set
    pub fn update_singular(&mut self, name: &str) {
        self.singular = String::from(name);
    }
    /// Sets the plural name
    ///
    /// # Arguments
    ///
    /// * `name` - The name to set
    pub fn update_plural(&mut self, name: &str) {
        self.plural = String::from(name);
    }
    /// Sets the adjective name
    ///
    /// # Arguments
    ///
    /// * `name` - The name to set
    pub fn update_adjective(&mut self, name: &str) {
        self.adjective = String::from(name);
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
        if !self.adjective.is_empty() {
            vec.push(self.adjective.clone());
        }
        vec
    }
    /// Returns the singular name
    ///
    /// # Returns
    ///
    /// * `&str` - The singular name
    #[must_use]
    pub fn get_singular(&self) -> &str {
        &self.singular
    }
    /// Returns the plural name
    ///
    /// # Returns
    ///
    /// * `&str` - The plural name
    #[must_use]
    pub fn get_plural(&self) -> &str {
        &self.plural
    }
    /// Returns the adjective name
    ///
    /// # Returns
    ///
    /// * `&str` - The adjective name
    #[must_use]
    pub fn get_adjective(&self) -> &str {
        &self.adjective
    }
    /// Creates a new Name struct with the given singular, plural, and adjective names
    ///
    /// # Arguments
    ///
    /// * `name_singular` - The singular name
    /// * `name_plural` - The plural name
    /// * `name_adjective` - The adjective name
    ///
    /// # Returns
    ///
    /// * The Name struct
    #[must_use]
    pub fn new(name_singular: &str, name_plural: &str, name_adjective: &str) -> Self {
        Self {
            singular: name_singular.to_string(),
            plural: name_plural.to_string(),
            adjective: name_adjective.to_string(),
        }
    }
}
