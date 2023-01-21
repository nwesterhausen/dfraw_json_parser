use encoding_rs_io::DecodeReaderBytesBuilder;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::parser::raws::info_txt::DFInfoFile;
use crate::parser::raws::{graphics, RawObjectKind};
use crate::parser::refs::{DF_ENCODING, RAW_TOKEN_RE};

impl super::DFGraphic {
    #[allow(clippy::too_many_lines)]
    pub fn parse<P: AsRef<Path>>(
        input_path: &P,
        info_text: &DFInfoFile,
    ) -> Vec<graphics::DFGraphic> {
        let caller = "Parse Simple Graphic Raw";
        let mut results: Vec<graphics::DFGraphic> = Vec::new();

        let file = match File::open(input_path) {
            Ok(f) => f,
            Err(e) => {
                log::error!("{} - Error opening raw file for parsing!\n{:?}", caller, e);
                return results;
            }
        };

        let decoding_reader = DecodeReaderBytesBuilder::new()
            .encoding(Some(*DF_ENCODING))
            .build(file);
        let reader = BufReader::new(decoding_reader);

        let mut raw_filename = String::new();
        let mut current_object = RawObjectKind::None;
        let mut started = false;
        let mut sprite_temp = graphics::DFGraphic::new(&raw_filename, "None:None", info_text);

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

            let line = match line {
                Ok(l) => l,
                Err(e) => {
                    log::error!("{} - Line-reading error\n{:?}", caller, e);
                    continue;
                }
            };

            if index == 0 {
                raw_filename = String::from(&line);
                continue;
            }

            for cap in RAW_TOKEN_RE.captures_iter(&line) {
                log::trace!("{} - Key: {} Value: {}", caller, &cap[2], &cap[3]);
                match &cap[2] {
                    "OBJECT" => match &cap[3] {
                        "GRAPHICS" => {
                            // Discovered raws for plants.
                            current_object = RawObjectKind::Graphics;
                        }
                        &_ => {
                            log::debug!("{} - Wrong type of raw ({})", caller, &cap[3]);
                            return Vec::new();
                            // current_object = RawObjectKind::None;
                        }
                    },
                    "CREATURE_GRAPHICS"
                    | "CREATURE_CASTE_GRAPHICS"
                    | "TILE_GRAPHICS"
                    | "PLANT_GRAPHICS" => {
                        // We are starting a creature object capture
                        if let RawObjectKind::Graphics = current_object {
                            if started {
                                if !&sprite_temp.kind.eq(&graphics::Kind::Empty) {
                                    results.push(sprite_temp);
                                }
                            } else {
                                started = true;
                            }
                            //Reset all temp values
                            log::trace!("Starting new graphic {}", &cap[3]);
                            //1. Make new sprite from its definition
                            sprite_temp = graphics::DFGraphic::new(
                                &raw_filename,
                                format!("{}:{}", &cap[2], &cap[3]).as_str(),
                                info_text,
                            );
                        }
                    }
                    "DEFAULT" | "SHRUB" | "PICKED" | "SEED" | "CROP" => {
                        sprite_temp.add_tile_from_token(&format!("{}:{}", &cap[2], &cap[3]));
                    }
                    &_ => {
                        // if sprite_temp.kind.eq(&graphics::Kind::Empty) {
                        //     log::debug!("Skipping {}:{} because empty sprite", &cap[2], &cap[3]);
                        // } else {
                        //     sprite_temp.add_tile_from_token(&format!("{}:{}", &cap[2], &cap[3]));
                        // }
                    }
                }
            }
        }

        if let RawObjectKind::Graphics = current_object {
            if !&sprite_temp.kind.eq(&graphics::Kind::Empty) {
                results.push(sprite_temp);
            }
        }
        log::info!(
            "{} sprite graphics objects defined in {} ({} {} in {:?})",
            results.len(),
            &raw_filename,
            info_text.get_identifier(),
            info_text.displayed_version,
            info_text.get_location(),
        );
        results
    }
}
