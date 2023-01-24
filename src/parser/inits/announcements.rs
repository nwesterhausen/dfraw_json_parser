use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use encoding_rs_io::DecodeReaderBytesBuilder;
use serde::{Deserialize, Serialize};

use crate::parser::{
    refs::{DF_ENCODING, RAW_TOKEN_RE},
    RawsStyleSerializable,
};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Flag {
    /// announcement will be displayed in the main adventure announcement log (and on screen)
    AdventureDisplay,
    /// announcement will be displayed in the dwarf announcement alerts (D_D)
    DwarfDisplay,
    /// announcement will appear in a box and pause the game (while box displayed)
    DoMega,
    /// the announcement will cause the alert button to light up
    Alert,
    /// announcement will cause the game to pause
    Pause,
    /// the announcement will cause the game to recenter (if possible)
    Recenter,
    /// announcement will be associated to the unit combat/hunting/sparring reports
    UnitCombatReport,
    /// announcement will be associated to any active unit combat/hunting/sparring reports, but if there
    /// are no reports it will not create one
    UnitCombatReportAllActive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct DFAnnouncement {
    /// Announcement key string
    key: String,
    flags: Vec<Flag>,
}

impl DFAnnouncement {
    // pub fn empty() -> Self {
    //     Self {
    //         key: String::from("Empty"),
    //         flags: Vec::new(),
    //     }
    // }
    pub fn new(key: &str) -> Self {
        Self {
            key: String::from(key),
            flags: Vec::new(),
        }
    }
    pub fn add_flag(&mut self, flag: Flag) {
        if !self.flags.contains(&flag) {
            self.flags.push(flag);
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn parse<P: AsRef<Path>>(announcements_txt_path: &P) -> Vec<DFAnnouncement> {
        let caller = "Parse announcements.txt";
        let mut results: Vec<DFAnnouncement> = Vec::new();

        let file = match File::open(announcements_txt_path) {
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

        for (index, line) in reader.lines().enumerate() {
            if line.is_err() {
                log::error!(
                    "{} - Error processing {}:{}",
                    caller,
                    announcements_txt_path.as_ref().display(),
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

            let mut announcement_temp;
            for cap in RAW_TOKEN_RE.captures_iter(&line) {
                log::trace!("{} - Key: {} Value: {}", caller, &cap[2], &cap[3]);

                announcement_temp = DFAnnouncement::new(&cap[2]);
                let raw_flags = cap[3].split(':');
                for (_idx, raw_flag) in raw_flags.enumerate() {
                    match raw_flag {
                        "A_D" | "A_DISPLAY" => announcement_temp.add_flag(Flag::AdventureDisplay),
                        "D_D" | "D_DISPLAY" => announcement_temp.add_flag(Flag::DwarfDisplay),
                        "UCR" | "UNIT_COMBAT_REPORT" => {
                            announcement_temp.add_flag(Flag::UnitCombatReport);
                        }
                        "UCR_A" | "UNIT_COMBAT_REPORT_ALL_ACTIVE" => {
                            announcement_temp.add_flag(Flag::UnitCombatReportAllActive);
                        }
                        "ALERT" => announcement_temp.add_flag(Flag::Alert),
                        "P" | "PAUSE" => announcement_temp.add_flag(Flag::Pause),
                        "R" | "RECENTER" => announcement_temp.add_flag(Flag::Recenter),
                        "BOX" | "DO_MEGA" => announcement_temp.add_flag(Flag::DoMega),
                        &_ => (),
                    }
                }

                results.push(announcement_temp);
            }
        }

        results
    }
}

impl RawsStyleSerializable for DFAnnouncement {
    /// Build the raws-style bracketed string to write to file for the given announcement.
    ///
    /// Returns:
    ///
    /// A string like `[REACHED_PEAK:A_D:D_D:BOX]`
    fn to_raws_style(&self) -> String {
        let mut values = vec![self.key.clone()];
        self.flags.iter().for_each(|flag| match flag {
            Flag::AdventureDisplay => values.push(String::from("A_D")),
            Flag::DwarfDisplay => values.push(String::from("D_D")),
            Flag::DoMega => values.push(String::from("BOX")),
            Flag::Alert => values.push(String::from("ALERT")),
            Flag::Pause => values.push(String::from("P")),
            Flag::Recenter => values.push(String::from("R")),
            Flag::UnitCombatReport => values.push(String::from("UCR")),
            Flag::UnitCombatReportAllActive => values.push(String::from("UCR_A")),
        });

        format!("[{}]", values.join(":"))
    }
}
