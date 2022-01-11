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
    pub fn new(argument_text: String) -> Self {
        let mut arg_names: Vec<&str> = argument_text.split(":").collect::<Vec<&str>>();
        let mut names: Vec<&str> = Vec::new();
        while arg_names.len() > 0 {
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChildName {
    singular: String,
    plural: String,
}

impl ChildName {
    pub fn new(argument_text: String) -> Self {
        let mut arg_names: Vec<&str> = argument_text.split(":").collect::<Vec<&str>>();
        let mut names: Vec<&str> = Vec::new();
        while arg_names.len() > 0 {
            names.push(arg_names.remove(0));
        }
        while names.len() < 2 {
            names.push("");
        }
        Self {
            singular: String::from(names[0]),
            plural: String::from(names[1]),
        }
    }
}
