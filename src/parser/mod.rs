use serde_json::to_string;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::LogLevels;

mod conversion;
mod raws;
mod reader;

pub fn parse_directory(raws_directory: String, out_directory: PathBuf, logging: LogLevels) {
    let mut creatures: Vec<raws::creature::DFCreature> = Vec::new();

    if logging.info {
        println!("Parsing raw files recursively from {}", raws_directory);
    }

    // Read all the files in the directory, selectively parse the .txt files
    for entry in WalkDir::new(raws_directory)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.ends_with(".txt") {
            let entry_path = entry.path().to_string_lossy().to_string();
            // println!("parsing {}", &entry_path);
            creatures.append(&mut reader::parse_file(entry_path, logging));
        }
    }
    // The destination file is out.json inside the out_directory
    let out_filepath = out_directory.join("out.json");
    let out_file =
        File::create(&out_filepath.as_path()).expect("Unable to open out.json for writing");

    let mut stream = BufWriter::new(out_file);
    let write_error = &format!("Unable to write to {}", out_filepath.to_string_lossy());
    write!(stream, "[").expect(write_error);

    if logging.info {
        println!(" {:->69}", "");
    }

    write!(
        stream,
        "{}",
        stringify_raw_vec(creatures, logging).join(",")
    )
    .expect(write_error);
    stream.flush().expect(write_error);

    write!(stream, "]").expect(write_error);
    stream.flush().expect(write_error);
}

fn stringify_raw_vec(raws: Vec<raws::creature::DFCreature>, logging: LogLevels) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    if logging.info {
        println!("{:>57}{:>4} creatures", "Parsed ", raws.len());
    }
    for creature in raws {
        // Conversion into the "web format" for JSON occurs inline here where we
        // use serde to create a JSON string.
        results.push(format!(
            "{}",
            to_string(&conversion::WebCreature::from(creature)).unwrap()
        ));
    }
    results
}
