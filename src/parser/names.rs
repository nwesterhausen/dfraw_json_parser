use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(ts_rs::TS)]
#[ts(export)]
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
}

#[derive(ts_rs::TS)]
#[ts(export)]
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
}

#[derive(ts_rs::TS)]
#[ts(export)]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct StateName {
    solid: String,
    liquid: String,
    gas: String,
}

impl StateName {
    pub fn is_empty(&self) -> bool {
        self.solid.is_empty() && self.liquid.is_empty() && self.gas.is_empty()
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
    pub fn as_vec(&self) -> Vec<String> {
        let mut vec = Vec::new();
        if !self.solid.is_empty() {
            vec.push(self.solid.clone());
        }
        if !self.liquid.is_empty() {
            vec.push(self.liquid.clone());
        }
        if !self.gas.is_empty() {
            vec.push(self.gas.clone());
        }
        vec
    }
}
