use clap::Parser;
use std::path::Path;

const HELP_GAME_DIR: &str = "Specify the directory where Dwarf Fortress is installed.

This directory will likely include the 'gamelog.txt' file, and it should have a 'data' subdirectory.";

const HELP_OUT_DIR: &str = "Specify the directory that the JSON database should be saved into.

If raw files are parsed, a JSON database (an array of objects) is
saved to disk in a location specified by this argument. This will
create an 'raws.json' file in the directory specified by this argument.

Alongside raws.json will be a modules.json which is a JSON database for the
raw modules that were found and parsed.";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Args {
    /// Path to df game directory
    #[clap(short, long, default_value_t = String::new(), long_help = HELP_GAME_DIR)]
    game_dir: String,

    /// Path to save JSON database
    #[clap(short, long, default_value_t = String::from("./www/"), long_help = HELP_OUT_DIR)]
    out_dir: String,
}

fn main() {
    // Configure logger at runtime
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| out.finish(format_args!("[{}] {}", record.level(), message)))
        // Add blanket level filter -
        .level(log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        // Apply globally
        .apply()
        .expect("Failed to start logger");

    let args = Args::parse();

    if !args.game_dir.is_empty() {
        let Ok(out_path) = std::fs::canonicalize(Path::new(&args.out_dir)) else {
            log::error!("Unable to standardize output path {} for writing.", &args.out_dir);
            return;
        };
        if !out_path.exists() {
            log::error!(
                "Non-existent path specified for saving file to {:?}",
                out_path
            );
            return;
        }
        if out_path.is_dir() {
            // If a directory for raws was specified, we will parse what raws we find
            dfraw_json_parser::parse_game_raws_to_file(
                args.game_dir.as_str(),
                &out_path.join("raws.json").to_path_buf(),
            );
            // Also save the modules info
            dfraw_json_parser::parse_raw_module_info_to_file(
                args.game_dir.as_str(),
                &out_path.join("modules.json").to_path_buf(),
            );
        } else {
            log::error!("A non-directory was specified for out_dir");
        }
    }
}
