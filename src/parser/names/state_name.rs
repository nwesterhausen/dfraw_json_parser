use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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
