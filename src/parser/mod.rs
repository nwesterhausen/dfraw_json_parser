use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

mod creature;
mod raws;
mod reader;

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
            // println!("parsing {}", &entry_path);
            json_strings.append(&mut reader::parse_file(entry_path))
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
