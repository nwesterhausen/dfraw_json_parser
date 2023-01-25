use encoding_rs_io::DecodeReaderBytesBuilder;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::parser::raws::{info_txt, RawModuleLocation};
use crate::parser::refs::{DF_ENCODING, NON_DIGIT_RE, RAW_TOKEN_RE};
use crate::util::get_parent_dir_name;

impl super::DFInfoFile {
    #[allow(clippy::too_many_lines)]
    pub fn parse<P: AsRef<Path>>(info_file_path: &P) -> info_txt::DFInfoFile {
        let parent_dir = get_parent_dir_name(info_file_path);
        let location = RawModuleLocation::from_info_text_file_path(info_file_path);

        let file = match File::open(info_file_path) {
            Ok(f) => f,
            Err(e) => {
                log::error!(
                    "DFInfoFile - Error opening raw file for parsing in \"{}\"\n{:?}",
                    parent_dir,
                    e
                );
                return info_txt::DFInfoFile::empty();
            }
        };

        let decoding_reader = DecodeReaderBytesBuilder::new()
            .encoding(Some(*DF_ENCODING))
            .build(file);
        let reader = BufReader::new(decoding_reader);

        // info.txt details
        let mut caller = String::from("DFInfoFile");
        let mut info_file_data: info_txt::DFInfoFile =
            info_txt::DFInfoFile::new("", location, &parent_dir);

        for (index, line) in reader.lines().enumerate() {
            if line.is_err() {
                log::error!("{} - Error processing {:?}:{}", caller, parent_dir, index);
                continue;
            }
            let line = match line {
                Ok(l) => l,
                Err(e) => {
                    log::error!("{} - Line-reading error\n{:?}", caller, e);
                    continue;
                }
            };
            for cap in RAW_TOKEN_RE.captures_iter(&line) {
                let captured_key = match cap.get(2) {
                    Some(v) => v.as_str(),
                    _ => {
                        continue;
                    }
                };
                let captured_value = match cap.get(3) {
                    Some(v) => v.as_str(),
                    _ => {
                        continue;
                    }
                };

                log::trace!(
                    "{} - Key: {} Value: {}",
                    caller,
                    captured_key,
                    captured_value
                );

                match captured_key {
                    // SECTION FOR MATCHING info.txt DATA
                    "ID" => {
                        // the [ID:identifier] tag should be the top of the info.txt file
                        info_file_data =
                            info_txt::DFInfoFile::new(captured_value, location, &parent_dir);
                        caller = format!("DFInfoFile ({})", &captured_value);
                    }
                    "NUMERIC_VERSION" => match captured_value.parse() {
                        Ok(n) => info_file_data.numeric_version = n,
                        Err(_e) => {
                            log::warn!(
                                "{} - 'NUMERIC_VERSION' should be integer {}",
                                caller,
                                parent_dir
                            );
                            // match on \D to replace any non-digit characters with empty string
                            let digits_only =
                                NON_DIGIT_RE.replace_all(captured_value, "").to_string();
                            match digits_only.parse() {
                                Ok(n) => info_file_data.numeric_version = n,
                                Err(_e) => {
                                    log::error!(
                                        "{} - Unable to parse any numbers from {}",
                                        caller,
                                        digits_only
                                    );
                                }
                            }
                        }
                    },
                    "EARLIEST_COMPATIBLE_NUMERIC_VERSION" => match captured_value.parse() {
                        Ok(n) => info_file_data.earliest_compatible_numeric_version = n,
                        Err(_e) => {
                            log::warn!(
                                "{} - 'EARLIEST_COMPATIBLE_NUMERIC_VERSION' should be integer {}",
                                caller,
                                parent_dir
                            );
                            // match on \D to replace any non-digit characters with empty string
                            let digits_only =
                                NON_DIGIT_RE.replace_all(captured_value, "").to_string();
                            match digits_only.parse() {
                                Ok(n) => info_file_data.earliest_compatible_numeric_version = n,
                                Err(_e) => {
                                    log::error!(
                                        "{} - Unable to parse any numbers from {}",
                                        caller,
                                        digits_only
                                    );
                                }
                            }
                        }
                    },
                    "DISPLAYED_VERSION" => {
                        info_file_data.displayed_version = String::from(captured_value);
                        caller = format!(
                            "DFInfoFile ({}@v{})",
                            info_file_data.get_identifier(),
                            &captured_value
                        );
                    }
                    "EARLIEST_COMPATIBLE_DISPLAYED_VERSION" => {
                        info_file_data.earliest_compatible_displayed_version =
                            String::from(captured_value);
                    }
                    "AUTHOR" => {
                        info_file_data.author = String::from(captured_value);
                    }
                    "NAME" => {
                        info_file_data.name = String::from(captured_value);
                    }
                    "DESCRIPTION" => {
                        info_file_data.description = String::from(captured_value);
                    }
                    &_ => (),
                }
            }
        }

        // Do some final checks to confirm that the name is set. Specifically in "Dark Ages V - War & Mythos" the
        // [name] Token in the info.txt is written incorrectly as "[name]X" instead of [name:X]
        if info_file_data.name.is_empty() || info_file_data.name.is_empty() {
            info_file_data.name = info_file_data.get_identifier();
        }

        // Check for 'unknown' identifier and try to provide any extra info
        if info_file_data.get_identifier() == "unknown" {
            log::error!(
                "Failure parsing proper info from {}",
                info_file_path.as_ref().display()
            );
        }

        info_file_data
    }
}
