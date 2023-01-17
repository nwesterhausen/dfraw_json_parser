use crate::parser::refs::{DF_ENCODING, RAW_TOKEN_RE};

use encoding_rs_io::DecodeReaderBytesBuilder;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use super::raws::RawObjectKind;

pub mod creature;
pub mod info_file;
pub mod inorganic;
pub mod plant;

/// It reads a file, line by line, and checks the first line for the filename, reads lines until it encounters the
/// [OBJECT:<type>] tag in the file.
///
/// Arguments:
///
/// * `input_path`: Path to the file to be read
/// * `module_info`: Information about the raw module `input_path` is within
///
/// Returns:
///
/// `RawObjectKind` for the type of [OBJECT] tag encountered, and `RawObjectKind::None` if it is unsupported.
pub fn read_raw_file_type<P: AsRef<Path>>(input_path: &P) -> RawObjectKind {
    let caller = "Raw File Type Checker";
    // Validate file exists
    if !input_path.as_ref().exists() {
        log::error!(
            "{} - Path doesn't exist {}",
            caller,
            input_path.as_ref().display()
        );
        return RawObjectKind::None;
    }
    if !input_path.as_ref().is_file() {
        log::error!(
            "{} - Path does not point to a file {}",
            caller,
            input_path.as_ref().display(),
        );
        return RawObjectKind::None;
    }

    // Open the file
    let Ok(file) = File::open(input_path) else {
        log::error!("{} - Unable to open file {}",caller, input_path.as_ref().display());
        return RawObjectKind::None;
    };

    // Setup a file reader for the encoding used by DF
    let decoding_reader = DecodeReaderBytesBuilder::new()
        .encoding(Some(*DF_ENCODING))
        .build(file);
    let reader = BufReader::new(decoding_reader);

    // String to store the parsed filename in
    let mut raw_filename = String::new();

    // Read in lines until we encounter the [OBJECT tag] or complete the file.
    for (index, line) in reader.lines().enumerate() {
        if line.is_err() {
            log::error!(
                "{} - Error processing {}:{}",
                caller,
                input_path.as_ref().display(),
                index
            );
            continue;
        }
        // Match the line so we can check the pieces of it
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                log::error!("Line-reading error\n{:?}", e);
                continue;
            }
        };
        // The filename is always the top line of a DF raw file
        if index == 0 {
            raw_filename = String::from(&line);
            continue;
        }
        // Multiple matches can occur in a single line, so we loop over all captures within the match
        // for this line.
        for cap in RAW_TOKEN_RE.captures_iter(&line) {
            log::trace!("{} - Key: {} Value: {}", caller, &cap[2], &cap[3]);
            // Match the front part of the tag
            match &cap[2] {
                // We are only concerned with the [OBJECT] key
                "OBJECT" => {
                    log::trace!("{} - {} is a {} raw file", caller, raw_filename, &cap[3]);
                    match &cap[3] {
                        "CREATURE" => {
                            return RawObjectKind::Creature;
                        }
                        "PLANT" => {
                            return RawObjectKind::Plant;
                        }
                        "INORGANIC" => {
                            return RawObjectKind::Inorganic;
                        }
                        // Currently, any other type of raw object is unsupported
                        &_ => {
                            log::debug!("{} - Currently no support for OBJECT:{}", caller, &cap[3]);
                            return RawObjectKind::None;
                            // current_object = RawObjectKind::None;
                        }
                    }
                }
                &_ => (),
            }
        }
    }

    // Reading through the entire file and not finding an [OBJECT] tag means the raw file is invalid
    log::warn!(
        "{} - no [OBJECT] tag in {}",
        caller,
        input_path.as_ref().display()
    );
    RawObjectKind::None
}
