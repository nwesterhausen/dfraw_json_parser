use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", rename = "SingPlurName")]
/// The name of a raw object with only singular and plural forms
pub struct Name {
    singular: String,
    plural: String,
}

impl Name {
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
    pub fn is_empty(&self) -> bool {
        self.singular.is_empty() && self.plural.is_empty()
    }
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

    pub fn new(name_singular: &str, name_plural: &str) -> Self {
        Self {
            singular: name_singular.to_string(),
            plural: name_plural.to_string(),
        }
    }
}
