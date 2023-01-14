use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};
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

const HELP_SINGLE_RAW_MODULE: &str = "Specify a single raw module to parse, output is saved or put to console.

Details will be filled in based on provided path and its contents. This should then output all the parsed raws
within the specified raw module.";

const HELP_SINGLE_RAWS_LOCATION: &str = "Specify a single raw module container directory to parse, output is saved or put to console.

Details will be filled in based on provided path and its contents. This should then output all the parsed raws
from the raw modules within the location path specified.";

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

    /// Single raw module to parse
    #[clap(short, long, default_value_t = String::new(), long_help = HELP_SINGLE_RAW_MODULE)]
    module_path: String,

    /// Single raw modules location to parse
    #[clap(short, long, default_value_t = String::new(), long_help = HELP_SINGLE_RAWS_LOCATION)]
    location_path: String,
}

fn main() {
    // Specify color configuration
    let colors = ColoredLevelConfig::new()
        // Specify info as cyan
        .info(Color::Cyan);
    // Configure logger at runtime
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                message
            ));
        })
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
        run_for_game_dir(args.game_dir.as_str(), args.out_dir.as_str());
    }

    if !args.raw_file.is_empty() {
        run_for_single_raw_file(args.raw_file.as_str(), args.out_dir.as_str());
    }

    if !args.module_path.is_empty() {
        run_for_single_raw_module(args.module_path.as_str(), args.out_dir.as_str());
    }

    if !args.location_path.is_empty() {
        run_for_single_raw_module_location(args.location_path.as_str(), args.out_dir.as_str());
    }
}

fn run_for_game_dir(game_dir: &str, out_dir: &str) {
    if out_dir.is_empty() {
        log::error!("Unable to parse and output JSON without specifying out_dir");
        return;
    }
    let Ok(out_path) = std::fs::canonicalize(Path::new(&out_dir)) else {
        log::error!("Unable to standardize output path {} for writing.", &out_dir);
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
        dfraw_json_parser::parse_game_raws_to_file(&game_dir, &out_path.join("raws.json"));
        // Also save the modules info
        dfraw_json_parser::parse_all_raw_module_info_to_file(
            &game_dir,
            &out_path.join("modules.json"),
        );
    } else {
        log::error!("A non-directory was specified for out_dir");
    }
}

fn run_for_single_raw_file(raw_file: &str, out_dir: &str) {
    let Ok(raw_file_path) = std::fs::canonicalize(Path::new(&raw_file)) else {
        log::error!("Unable to standardize raw file path to read. {}", &raw_file);
        return;
    };

    if out_dir.is_empty() {
        log::warn!("No output directory specified, dumping to console.");
        let parsed_raws = dfraw_json_parser::parse_single_raw_file(&raw_file_path);
        println!("{}", parsed_raws);
        return;
    }

    let Ok(out_path) = std::fs::canonicalize(Path::new(&out_dir)) else {
        log::error!("Unable to standardize output path {} for writing.", &out_dir);
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
            &out_path.join("single-raw.json"),
        );
    }
}

fn run_for_single_raw_module(module_path: &str, out_dir: &str) {
    let Ok(raw_module_path) = std::fs::canonicalize(Path::new(&module_path)) else {
        log::error!("Unable to standardize raw file path to read. {}", &module_path);
        return;
    };

    if out_dir.is_empty() {
        log::warn!("No output directory specified, dumping to console.");
        let parsed_raws = dfraw_json_parser::parse_info_txt_from_raw_module(&raw_module_path);
        println!("{}", parsed_raws);
        return;
    }

    let Ok(out_path) = std::fs::canonicalize(Path::new(&out_dir)) else {
        log::error!("Unable to standardize output path {} for writing.", &out_dir);
        return;    };
    if !out_path.exists() {
        log::error!(
            "Non-existent path specified for saving file to {:?}",
            out_path
        );
        return;
    }
    if out_path.is_dir() {
        dfraw_json_parser::parse_raw_module_to_file(
            &raw_module_path,
            &out_path.join("single-module.json"),
        );
    }
}

fn run_for_single_raw_module_location(location_path: &str, out_dir: &str) {
    let Ok(raw_module_location_path) = std::fs::canonicalize(Path::new(&location_path)) else {
        log::error!("Unable to standardize raw file path to read. {}", &location_path);
        return;
    };

    if out_dir.is_empty() {
        log::warn!("No output directory specified, dumping to console.");
        let parsed_raws = dfraw_json_parser::parse_module_location(&raw_module_location_path);
        println!("[{}]", parsed_raws);
        return;
    }

    let Ok(out_path) = std::fs::canonicalize(Path::new(&out_dir)) else {
        log::error!("Unable to standardize output path {} for writing.", &out_dir);
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
        dfraw_json_parser::parse_module_location_dir_to_file(
            &raw_module_location_path,
            &out_path.join("single-location.json"),
        );
    }
}
