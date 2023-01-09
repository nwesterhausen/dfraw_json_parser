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
        &self.solid.as_str()
    }
    pub fn set(&mut self, state: &str, name: &str) {
        match state {
            "Solid" | "SOLID" | "ALL_SOLID" => self.solid = String::from(name),
            "Liquid" | "LIQUID" | "ALL_LIQUID" => self.liquid = String::from(name),
            "Gas" | "GAS" | "ALL_GAS" => self.gas = String::from(name),
            _ => {
                log::debug!(
                    "Unable to classify {} for state descriptor '{}'",
                    state,
                    name
                );
            }
        }
    }
}
