use encoding_rs_io::DecodeReaderBytesBuilder;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::parser::raws::info_txt::DFInfoFile;
use crate::parser::raws::{tile_page, RawObjectKind};
use crate::parser::refs::{DF_ENCODING, RAW_TOKEN_RE};

impl super::DFTilePage {
    #[allow(clippy::too_many_lines)]
    pub fn parse<P: AsRef<Path>>(
        input_path: &P,
        info_text: &DFInfoFile,
    ) -> Vec<tile_page::DFTilePage> {
        let caller = "Parse Graphic Tile Page Raw";

        let file = match File::open(input_path) {
            Ok(f) => f,
            Err(e) => {
                log::error!("{} - Error opening raw file for parsing!\n{:?}", caller, e);
                return Vec::new();
            }
        };

        let decoding_reader = DecodeReaderBytesBuilder::new()
            .encoding(Some(*DF_ENCODING))
            .build(file);
        let reader = BufReader::new(decoding_reader);

        let mut raw_filename = String::new();
        let mut current_object = RawObjectKind::None;
        let mut started = false;
        let mut results: Vec<tile_page::DFTilePage> = Vec::new();
        let mut tile_page_temp = tile_page::DFTilePage::new("None", "None", info_text);

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
                        "TILE_PAGE" => {
                            // Discovered raws for plants.
                            current_object = RawObjectKind::GraphicsTilePage;
                        }
                        &_ => {
                            log::debug!("{} - Wrong type of raw ({})", caller, &cap[3]);
                            return Vec::new();
                        }
                    },
                    "TILE_PAGE" => {
                        // We are starting a creature object capture
                        if let RawObjectKind::GraphicsTilePage = current_object {
                            if started {
                                results.push(tile_page_temp);
                            } else {
                                started = true;
                            }
                            //Reset all temp values
                            log::trace!("Starting new graphic {}", &cap[3]);
                            //1. Make new sprite from its definition
                            tile_page_temp =
                                tile_page::DFTilePage::new(&raw_filename, &cap[3], info_text);
                        }
                    }
                    "FILE" => {
                        tile_page_temp.set_file(&cap[3]);
                    }
                    "TILE_DIM" => {
                        tile_page_temp.set_tile_dim_from_token(&cap[3]);
                    }
                    "PAGE_DIM_PIXELS" => {
                        tile_page_temp.set_page_dim_from_token(&cap[3]);
                    }
                    &_ => (),
                }
            }
        }

        if let RawObjectKind::GraphicsTilePage = current_object {
            results.push(tile_page_temp);
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
