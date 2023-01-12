use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Name {
    singular: String,
    plural: String,
    adjective: String,
}

impl Name {
    // Take the arguments for a name and split ':' into sing, plural, adjective
    pub fn new(argument_text: &str) -> Self {
        let mut arg_names: Vec<&str> = argument_text.split(':').collect::<Vec<&str>>();
        let mut names: Vec<&str> = Vec::new();
        while !arg_names.is_empty() {
            names.push(arg_names.remove(0));
        }
        while names.len() < 3 {
            names.push("");
        }
        Self {
            singular: String::from(names[0]),
            plural: String::from(names[1]),
            adjective: String::from(names[2]),
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingPlurName {
    singular: String,
    plural: String,
}

impl SingPlurName {
    pub fn new(argument_text: &str) -> Self {
        let mut arg_names: Vec<&str> = argument_text.split(':').collect::<Vec<&str>>();
        let mut names: Vec<&str> = Vec::new();
        while !arg_names.is_empty() {
            names.push(arg_names.remove(0));
        }
        while names.len() < 2 {
            names.push("");
        }
        if names[1].eq("STP") {
            return Self {
                singular: String::from(names[0]),
                plural: String::new(),
            };
        }
        Self {
            singular: String::from(names[0]),
            plural: String::from(names[1]),
        }
    }
    pub fn to_string_vec(&self) -> Vec<String> {
        vec![String::from(&self.singular), String::from(&self.plural)]
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

        if split.len() != 2 {
            log::error!("Unable to read name from {}", tag_value);
            // When we can't do anything about this name, just continue
            return;
        }

        match split[0] {
            "ALL_SOLID" | "SOLID" => {
                self.set_solid(split[1]);
            }
            "LIQUID" => {
                self.set_liquid(split[1]);
            }
            "GAS" => {
                self.set_gas(split[1]);
            }
            "ALL" => {
                self.set_solid(split[1]);
                self.set_liquid(split[1]);
                self.set_gas(split[1]);
            }
            _ => (),
        }
    }
}
