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

const HELP_SINGLE_RAW: &str = "Specify a single raw file to parse, output is saved or put to console.

Since there are some details dfraw_json_parser gets from the directory structure, those will
be filled with dummy values when using this command. They will be filled-in automatically. If you choose
to specify an out_dir, the parsed JSON will be saved to single-raw.json, otherwise it will be output
to the console.";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Args {
    /// Path to df game directory
    #[clap(short, long, default_value_t = String::new(), long_help = HELP_GAME_DIR)]
    game_dir: String,

    /// Path to save JSON database
    #[clap(short, long, default_value_t = String::new(), long_help = HELP_OUT_DIR)]
    out_dir: String,

    /// Single raw file to parse
    #[clap(short, long, default_value_t = String::new(), long_help = HELP_SINGLE_RAW)]
    raw_file: String,
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

    // If the Game Dir is specified
    if !args.game_dir.is_empty() {
        if args.out_dir.is_empty() {
            log::error!("Unable to parse and output JSON without specifying out_dir");
            return;
        }
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

    if !args.raw_file.is_empty() {
        let Ok(raw_file_path) = std::fs::canonicalize(Path::new(&args.raw_file)) else {
            log::error!("Unable to standardize raw file path to read. {}", &args.raw_file);
            return;
        };

        if args.out_dir.is_empty() {
            log::warn!("No output directory specified, dumping to console.");
            let parsed_raws = dfraw_json_parser::read_single_raw_file(&raw_file_path);
            println!("{}", parsed_raws);
            return;
        }

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
            dfraw_json_parser::read_single_raw_file_to_file(
                &raw_file_path,
                &out_path.join("single-raw.json").to_path_buf(),
            )
        }
    }
}
