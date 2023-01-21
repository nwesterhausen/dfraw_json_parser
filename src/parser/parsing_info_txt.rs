use std::path::Path;

use crate::parser::raws::info_txt::DFInfoFile;

use super::DFParser;

impl super::DFParser {
    /// It takes a path to a info.txt file, and returns a `DFInfoFile` for it
    ///
    /// Arguments:
    ///
    /// * `input_path`: The path to the file to be parsed.
    pub fn parse_info_file<P: AsRef<Path>>(input_path: &P) -> DFInfoFile {
        DFInfoFile::parse(input_path)
    }

    /// It reads the `info.txt` file in a given directory, and returns a struct containing the information
    /// in that file
    ///
    /// Arguments:
    ///
    /// * `root_path`: The path to the directory containing the raws.
    ///
    /// Returns:
    ///
    /// `DFInfoFile`
    pub fn parse_info_file_from_module_directory<P: AsRef<Path>>(root_path: &P) -> DFInfoFile {
        //1. Get information from the info.txt file
        if !root_path.as_ref().exists() {
            log::error!(
                "Provided directory to parse raws does not exist: {:?}",
                root_path.as_ref()
            );
            return DFInfoFile::empty();
        }
        if !root_path.as_ref().is_dir() {
            log::error!(
                "Provided 'directory' to parse is not actually a directory! {:?}",
                root_path.as_ref()
            );
            return DFInfoFile::empty();
        }

        // Check for info.txt
        let info_txt_path = root_path.as_ref().join("info.txt");
        if !info_txt_path.exists() {
            let Some(dir_name) = root_path.as_ref().file_name() else {
            log::error!("Error reading module dir {:?}", root_path.as_ref());
            return DFInfoFile::empty();
        };
            let dir_name_str = dir_name.to_str().unwrap_or("");

            if !(dir_name_str.eq("mod_upload")
                || dir_name_str.eq("examples and notes")
                || dir_name_str.eq("interaction examples"))
            {
                log::error!(
                    "No info.txt as expected in {:?}. Is this DF 50.xx?",
                    root_path.as_ref().file_name().unwrap_or_default()
                );
                return DFInfoFile::empty();
            }

            log::error!(
                "info_txt_path doesn't exist here: {}",
                info_txt_path.display()
            );
        }

        DFParser::parse_info_file(&info_txt_path)
    }
}
