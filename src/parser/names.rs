use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    singular: String,
    plural: String,
    adjective: String,
}

impl Name {
    // Take the arguments for a name and split ':' into sing, plural, adjective
    pub fn new(argument_text: &str) -> Self {
        let arg_names: Vec<&str> = argument_text.split(':').collect::<Vec<&str>>();

        let singular_name = *arg_names.first().unwrap_or(&"");
        let plural_name = *arg_names.get(1).unwrap_or(&"");
        let adjective_name = *arg_names.get(2).unwrap_or(&"");

        Self {
            singular: String::from(singular_name),
            plural: String::from(plural_name),
            adjective: String::from(adjective_name),
        }
    }
    pub fn to_string_vec(&self) -> Vec<String> {
        if self.singular.eq(&self.adjective) {
            return vec![String::from(&self.singular), String::from(&self.plural)];
        }
        vec![
            String::from(&self.singular),
            String::from(&self.plural),
            String::from(&self.adjective),
        ]
    }
    pub fn set_singular(&mut self, name: &str) {
        self.singular = String::from(name);
    }
    pub fn set_plural(&mut self, name: &str) {
        self.plural = String::from(name);
    }
    pub fn set_adjective(&mut self, name: &str) {
        self.adjective = String::from(name);
    }
    pub fn set_all(&mut self, name: &str) {
        self.singular = String::from(name);
        self.plural = String::from(name);
        self.adjective = String::from(name);
    }
    pub fn empty() -> Self {
        Self {
            singular: String::new(),
            plural: String::new(),
            adjective: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SingPlurName {
    singular: String,
    plural: String,
}

impl SingPlurName {
    pub fn new(argument_text: &str) -> Self {
        let arg_names: Vec<&str> = argument_text.split(':').collect::<Vec<&str>>();

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

    pub fn to_string_vec(&self) -> Vec<String> {
        vec![String::from(&self.singular), String::from(&self.plural)]
    }
    pub fn empty() -> Self {
        Self {
            singular: String::new(),
            plural: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StateName {
    solid: String,
    liquid: String,
    gas: String,
}

impl StateName {
    pub fn new() -> Self {
        Self {
            solid: String::new(),
            liquid: String::new(),
            gas: String::new(),
        }
    }
    pub fn from(solid: &str, liquid: &str, gas: &str) -> Self {
        Self {
            solid: String::from(solid),
            liquid: String::from(liquid),
            gas: String::from(gas),
        }
    }
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
    pub fn set_from_tag(&mut self, tag_value: &str) {
        // Split the value into a descriptor and value
        let split = tag_value.split(':').collect::<Vec<&str>>();
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
