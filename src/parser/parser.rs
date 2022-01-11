use encoding_rs_io::DecodeReaderBytesBuilder;
use regex::Regex;
use serde_json::to_string;
use slug::slugify;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

use super::creature::Creature;

enum RawObjectKind {
    Creature,
    None,
}

pub fn parse_file(input_path: String) -> Vec<String> {
    let re = Regex::new(r"(\[(?P<key>[^\[:]+):?(?P<value>[^\]\[]*)])").unwrap();
    let enc = encoding_rs::Encoding::for_label("latin1".as_bytes());

    let file = File::open(&input_path).unwrap();
    let decoding_reader = DecodeReaderBytesBuilder::new().encoding(enc).build(file);
    let reader = BufReader::new(decoding_reader);

    // let mut creatures = 0;
    let mut raw_filename = String::new();
    let mut current_object = RawObjectKind::None;
    let mut started = false;
    let mut creature_temp = Creature::new("None", "None");

    let mut results: Vec<String> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        if line.is_err() {
            eprintln!("Error processing {}:{}", &input_path, index);
            continue;
        }
        let line = line.unwrap();
        if index == 0 {
            raw_filename = String::from(&line);
            continue;
        }
        for cap in re.captures_iter(&line) {
            // println!("Key: {} Value: {}", &cap[2], &cap[3])
            match &cap[2] {
                "OBJECT" => match &cap[3] {
                    "CREATURE" => {
                        println!("Discovered raws for creatures.");
                        current_object = RawObjectKind::Creature;
                    }
                    &_ => {
                        println!("No support right now for OBJECT:{}", &cap[3]);
                        return results;
                        // current_object = RawObjectKind::None;
                    }
                },
                "CREATURE" => {
                    // We are starting a creature object capture
                    match current_object {
                        RawObjectKind::Creature => {
                            if started {
                                // If we already *were* capturing a creature, export it.
                                // Reset the temp values !!Todo
                                //println!("{:#?}", creature_temp);
                                // writeln!(stream, "{},", to_string(&creature_temp).unwrap())
                                //  .expect("Unable to write creature info to out.json.");
                                results.push(format!("{}", to_string(&creature_temp).unwrap()));
                            } else {
                                started = true;
                            }
                            creature_temp = Creature::new(&raw_filename, &cap[3]);
                        }
                        RawObjectKind::None => (),
                    }
                    continue;
                }
                "NAME" => {
                    creature_temp.name = String::from(&cap[3]);
                    continue;
                }
                "EGG_SIZE" => {
                    creature_temp.lays_eggs = true;
                    creature_temp.egg_size = cap[3].parse().expect("EGG_SIZE should be an integer");
                    continue;
                }
                "CLUTCH_SIZE" => {
                    creature_temp.lays_eggs = true;
                    let split = cap[3].split(":").collect::<Vec<&str>>();
                    creature_temp.clutch_size[0] = split[0]
                        .parse()
                        .expect("CLUTCH_SIZE min should be an integer");
                    creature_temp.clutch_size[1] = split[1]
                        .parse()
                        .expect("CLUTCH_SIZE max should be an integer");
                    continue;
                }
                "DESCRIPTION" => {
                    creature_temp.description = String::from(&cap[3]);
                    continue;
                }
                "MAXAGE" => {
                    let split = cap[3].split(":").collect::<Vec<&str>>();
                    creature_temp.max_age[0] =
                        split[0].parse().expect("MAXAGE min should be an integer");
                    creature_temp.max_age[1] =
                        split[1].parse().expect("MAXAGE max should be an integer");
                    continue;
                }
                "COPY_TAGS_FROM" => {
                    creature_temp.based_on =
                        format!("{}-{}-{}", raw_filename, "CREATURE", slugify(&cap[3]));
                    continue;
                }
                &_ => (),
            }
        }
    }
    match current_object {
        RawObjectKind::Creature => {
            // If we already *were* capturing a creature, export it.
            // println!("Finished capturing creature, now finished");
            // Reset the temp values !!Todo
            //println!("{:#?}", creature_temp);
            results.push(format!("{}", to_string(&creature_temp).unwrap()));
        }
        RawObjectKind::None => (),
    }
    // println!("{} creatures defined in {}", creatures, &raw_filename);
    results
}

pub fn parse_directory(raws_directory: String, out_directory: PathBuf) {
    let mut json_strings: Vec<String> = Vec::new();

    // Read all the files in the directory, selectively parse the .txt files
    for entry in WalkDir::new(raws_directory)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();

        if f_name.ends_with(".txt") {
            let entry_path = entry.path().to_string_lossy().to_string();
            println!("parsing {}", &entry_path);
            json_strings.append(&mut parse_file(entry_path))
        }
    }
    // The destination file is out.json inside the out_directory
    let out_filepath = out_directory.join("out.json");
    let out_file =
        File::create(&out_filepath.as_path()).expect("Unable to open out.json for writing");

    let mut stream = BufWriter::new(out_file);
    let write_error = &format!("Unable to write to {}", out_filepath.to_string_lossy());
    write!(stream, "[").expect(write_error);

    write!(stream, "{}", json_strings.join(",")).expect(write_error);
    stream.flush().expect(write_error);

    write!(stream, "]").expect(write_error);
    stream.flush().expect(write_error);
}
