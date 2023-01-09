use encoding_rs_io::DecodeReaderBytesBuilder;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::parser::raws::info;
use crate::parser::refs::{DF_ENCODING, NON_DIGIT_RE, RAW_TOKEN_RE};

pub fn parse(info_file_path: &Path, source_dir: &str) -> info::DFInfoFile {
    let relative_path = match info_file_path.parent() {
        Some(parent_dir) => {
            format!(
                "\"{}/{}/{}\"",
                source_dir,
                parent_dir.file_name().unwrap_or_default().to_string_lossy(),
                info_file_path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
            )
        }
        None => format!(
            "\"{}/<unknown>/{:?}\"",
            source_dir,
            info_file_path.file_name()
        ),
    };

    let file = match File::open(&info_file_path) {
        Ok(f) => f,
        Err(e) => {
            log::error!("DFInfoFile - Error opening raw file for parsing!\n{:?}", e);
            return info::DFInfoFile::new("error", source_dir, &relative_path);
        }
    };

    let decoding_reader = DecodeReaderBytesBuilder::new()
        .encoding(*DF_ENCODING)
        .build(file);
    let reader = BufReader::new(decoding_reader);

    // info.txt details
    let mut header = String::from("DFInfoFile");
    let mut info_file_data: info::DFInfoFile =
        info::DFInfoFile::new("", source_dir, &relative_path);

    for (index, line) in reader.lines().enumerate() {
        if line.is_err() {
            log::error!(
                "{} - Error processing {:?}:{}",
                header,
                relative_path,
                index
            );
            continue;
        }
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                log::error!("{} - Line-reading error\n{:?}", header, e);
                continue;
            }
        };
        for cap in RAW_TOKEN_RE.captures_iter(&line) {
            log::trace!("Key: {} Value: {}", &cap[2], &cap[3]);
            match &cap[2] {
                // SECTION FOR MATCHING info.txt DATA
                "ID" => {
                    // the [ID:identifier] tag should be the top of the info.txt file
                    info_file_data = info::DFInfoFile::new(&cap[3], source_dir, &relative_path);
                    header = format!("DFInfoFile ({})", &cap[3]);
                }
                "NUMERIC_VERSION" => match cap[3].parse() {
                    Ok(n) => info_file_data.numeric_version = n,
                    Err(_e) => {
                        log::warn!(
                            "{} - 'NUMERIC_VERSION' should be integer {}",
                            header,
                            relative_path
                        );
                        // match on \D to replace any non-digit characters with empty string
                        let digits_only = NON_DIGIT_RE.replace_all(&cap[3], "").to_string();
                        match digits_only.parse() {
                            Ok(n) => info_file_data.numeric_version = n,
                            Err(_e) => {
                                log::error!(
                                    "{} - Unable to parse any numbers from {}",
                                    header,
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
                            "{} - 'EARLIEST_COMPATIBLE_NUMERIC_VERSION' should be integer {}",
                            header,
                            relative_path
                        );
                        // match on \D to replace any non-digit characters with empty string
                        let digits_only = NON_DIGIT_RE.replace_all(&cap[3], "").to_string();
                        match digits_only.parse() {
                            Ok(n) => info_file_data.earliest_compatible_numeric_version = n,
                            Err(_e) => {
                                log::error!(
                                    "{} - Unable to parse any numbers from {}",
                                    header,
                                    digits_only
                                );
                            }
                        }
                    }
                },
                "DISPLAYED_VERSION" => {
                    info_file_data.displayed_version = String::from(&cap[3]);
                    header = format!(
                        "DFInfoFile ({}@v{})",
                        info_file_data.get_identifier(),
                        &cap[3]
                    );
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
