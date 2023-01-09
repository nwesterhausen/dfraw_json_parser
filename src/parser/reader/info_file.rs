use encoding_rs_io::DecodeReaderBytesBuilder;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::parser::raws::info;
use crate::parser::refs::{DF_ENCODING, NON_DIGIT_RE, RAW_TOKEN_RE};

pub fn parse(info_file_path: &Path, source_dir: &str) -> info::DFInfoFile {
    let file = match File::open(&info_file_path) {
        Ok(f) => f,
        Err(e) => {
            log::error!("Error opening raw file for parsing!\n{:?}", e);
            return info::DFInfoFile::new("error", source_dir);
        }
    };

    let decoding_reader = DecodeReaderBytesBuilder::new()
        .encoding(*DF_ENCODING)
        .build(file);
    let reader = BufReader::new(decoding_reader);

    // info.txt details
    let mut info_file_data: info::DFInfoFile = info::DFInfoFile::new("", source_dir);

    for (index, line) in reader.lines().enumerate() {
        if line.is_err() {
            log::error!(
                "DFInfoFile - Error processing {:?}:{}",
                &info_file_path,
                index
            );
            continue;
        }
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                log::error!("DFInfoFile - Line-reading error\n{:?}", e);
                continue;
            }
        };
        for cap in RAW_TOKEN_RE.captures_iter(&line) {
            log::trace!("Key: {} Value: {}", &cap[2], &cap[3]);
            match &cap[2] {
                // SECTION FOR MATCHING info.txt DATA
                "ID" => {
                    // the [ID:identifier] tag should be the top of the info.txt file
                    info_file_data = info::DFInfoFile::new(&cap[3], source_dir);
                }
                "NUMERIC_VERSION" => match cap[3].parse() {
                    Ok(n) => info_file_data.numeric_version = n,
                    Err(_e) => {
                        log::warn!(
                            "DFInfoFile - 'numeric_version' value in {}: '{}' is not integer! file: {}",
                            info_file_data.get_identifier(),
                            &cap[3],
                            info_file_path.display()
                        );
                        // match on \D to replace any non-digit characters with empty string
                        let digits_only = NON_DIGIT_RE.replace_all(&cap[3], "").to_string();
                        match digits_only.parse() {
                            Ok(n) => info_file_data.numeric_version = n,
                            Err(_e) => {
                                log::error!(
                                    "DFInfoFile - Unable to parse numerals from {}",
                                    digits_only
                                );
                            }
                        }
                    }
                },
                "EARLIEST_COMPATIBLE_NUMERIC_VERSION" => match cap[3].parse() {
                    Ok(n) => info_file_data.earliest_compatible_numeric_version = n,
                    Err(_e) => {
                        log::warn!(
                            "DFInfoFile - 'earliest_compatible_numeric_version' value in {}: '{}' is not integer! file: {}",
                            info_file_data.get_identifier(),
                            &cap[3],
                            info_file_path.display()
                        );
                        // match on \D to replace any non-digit characters with empty string
                        let digits_only = NON_DIGIT_RE.replace_all(&cap[3], "").to_string();
                        match digits_only.parse() {
                            Ok(n) => info_file_data.earliest_compatible_numeric_version = n,
                            Err(_e) => {
                                log::error!(
                                    "DFInfoFile - Unable to parse numerals from {}",
                                    digits_only
                                );
                            }
                        }
                    }
                },
                "DISPLAYED_VERSION" => {
                    info_file_data.displayed_version = String::from(&cap[3]);
                }
                "EARLIEST_COMPATIBLE_DISPLAYED_VERSION" => {
                    info_file_data.earliest_compatible_displayed_version = String::from(&cap[3]);
                }
                "AUTHOR" => {
                    info_file_data.author = String::from(&cap[3]);
                }
                "NAME" => {
                    info_file_data.name = String::from(&cap[3]);
                }
                "DESCRIPTION" => {
                    info_file_data.description = String::from(&cap[3]);
                }
                &_ => (),
            }
        }
    }

    // Do some final checks to confirm that the name is set. Specifically in "Dark Ages V - War & Mythos" the
    // [name] Token in the info.txt is written incorrectly as "[name]X" instead of [name:X]
    if info_file_data.name.is_empty() || info_file_data.name.len() == 0 {
        info_file_data.name = String::from(info_file_data.get_identifier());
    }

    info_file_data
}
