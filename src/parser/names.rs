use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
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
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct SingPlurName {
    singular: String,
    plural: String,
}

impl SingPlurName {
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
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StateName {
    solid: String,
    liquid: String,
    gas: String,
}

#[allow(dead_code)] // Until we add material parsing
impl StateName {
    pub fn set_solid(&mut self, name: &str) {
        self.solid = String::from(name);
    }
    pub fn set_liquid(&mut self, name: &str) {
        self.liquid = String::from(name);
    }
    pub fn set_gas(&mut self, name: &str) {
        self.gas = String::from(name);
    }
    pub fn get_solid(&self) -> &str {
        self.solid.as_str()
    }
    pub fn add_from_value(&mut self, value: &str) {
        // Split the value into a descriptor and value
        let split = value.split(':').collect::<Vec<&str>>();
        let tag_key = match split.first() {
            Some(v) => *v,
            _ => {
                return;
            }
        };
        let tag_value = match split.get(1) {
            Some(v) => *v,
            _ => {
                return;
            }
        };

        match tag_key {
            "ALL_SOLID" | "SOLID" => {
                self.set_solid(tag_value);
            }
            "LIQUID" => {
                self.set_liquid(tag_value);
            }
            "GAS" => {
                self.set_gas(tag_value);
            }
            "ALL" => {
                self.set_solid(tag_value);
                self.set_liquid(tag_value);
                self.set_gas(tag_value);
            }
            _ => (),
        }
    }
}
