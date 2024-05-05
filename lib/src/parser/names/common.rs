use serde::{Deserialize, Serialize};
use std::fmt::Debug;



#[derive(Serialize, Deserialize, Debug, Clone, Default, specta::Type)]
#[serde(rename_all = "camelCase")]
/// A name with a singular, plural, and adjective form
pub struct Name {
    singular: String,
    plural: String,
    adjective: String,
}

impl Name {
    // Take the arguments for a name and split ':' into sing, plural, adjective
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
    pub fn is_empty(&self) -> bool {
        self.singular.is_empty() && self.plural.is_empty() && self.adjective.is_empty()
    }
    pub fn update_singular(&mut self, name: &str) {
        self.singular = String::from(name);
    }
    pub fn update_plural(&mut self, name: &str) {
        self.plural = String::from(name);
    }
    pub fn update_adjective(&mut self, name: &str) {
        self.adjective = String::from(name);
    }
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
    pub fn get_singular(&self) -> &str {
        &self.singular
    }
    pub fn get_plural(&self) -> &str {
        &self.plural
    }
    pub fn get_adjective(&self) -> &str {
        &self.adjective
    }
    pub fn new(name_singular: &str, name_plural: &str, name_adjective: &str) -> Self {
        Self {
            singular: name_singular.to_string(),
            plural: name_plural.to_string(),
            adjective: name_adjective.to_string(),
        }
    }
}
