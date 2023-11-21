use std::io::Read;
use std::path::Path;

use crate::{parser::raws::RawObject, util::try_get_file};

pub fn parse_legends_export<P: AsRef<Path>>(input_path: &P) -> Vec<Box<dyn RawObject>> {
    let mut results = Vec::new();
    let Some(mut file) = try_get_file(input_path) else {
        log::error!(
            "parse_legends_export: Unable to open file {}",
            input_path.as_ref().display()
        );
        return results;
    };

    // Read the file into a str for parsing
    let mut file_str = String::new();
    let Ok(_) = file.read_to_string(&mut file_str) else {
        log::error!(
            "parse_legends_export: Unable to read file {}",
            input_path.as_ref().display()
        );
        return results;
    };

    for token in xmlparser::Tokenizer::from(file_str.as_str()) {
        println!("{:?}", token);
        // match token.unwrap() {
        //     xmlparser::Token::ElementStart { name, .. } => {
        //         if name == "entity" {
        //             results.push(Box::new(xml_entity::parse_entity(&mut token)));
        //         } else if name == "creature" {
        //             results.push(Box::new(xml_creature::parse_creature(&mut token)));
        //         }
        //     }
        //     _ => {}
        // }
    }

    results
}
