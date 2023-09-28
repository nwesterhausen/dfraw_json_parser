use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use dfraw_json_parser::{
    options::{ParserOptions, ParsingJob},
    parser::raw_locations::RawModuleLocation,
};
use fern::colors::{Color, ColoredLevelConfig};

const HELP_OUT_DIR: &str = "Specify the directory that the JSON database should be saved into.

If raw files are parsed, a JSON database (an array of objects) is
saved to disk in a location specified by this argument. This will
create an 'raws.json' file in the directory specified by this argument.

If not specified, the current working directory will be used.";

const HELP_TARGET_PATH: &str = "Specify the path to parse raws from.

If you choose `-p RawFile`, or `-p RawModuleInfoFile`, this must be a file path.

If you choose `-p RawModule` this must be the path to the module folder.

If you choose `-p All` (or leave it unset), this must be the path to the Dwarf Fortress 
game directory (e.g. `C:/Program Files/Dwarf Fortress`).";

const HELP_PARSE: &str = "What level of parsing to perform.

Choose from:
    - RawFile: Parse a single raw file
    - RawModuleInfoFile: Parse a single raw module info file
    - RawModule: Parse a single raw module
    - All: Parse all, limited to locations if specified

If All is chosen, the flags for parse_vanilla, parse_mods_downloaded, and parse_mods_installed will all 
influence exactly which locations are parsed. All also includes a modules.json which is a database of all
modules and their info.
";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Args {
    // Target path to parse
    #[clap(short, long, long_help = HELP_TARGET_PATH)]
    #[arg(required = true)]
    target_path: String,

    /// Path to save JSON to
    #[clap(short, long, default_value_t = String::new(), long_help = HELP_OUT_DIR)]
    out_dir: String,

    /// What to parse
    #[arg(value_enum)]
    #[clap(long, default_value_t = ParseTarget::All, long_help = HELP_PARSE)]
    parse: ParseTarget,

    /// Parse vanilla location
    #[clap(long, default_value_t = true, long_help = "Parse vanilla raws")]
    parse_vanilla: bool,

    /// Parse workshop mods location (downloaded mods)
    #[clap(
        long,
        default_value_t = true,
        long_help = "Parse downloaded workshop mods"
    )]
    parse_mods_downloaded: bool,

    /// Parse installed mods location
    #[clap(long, default_value_t = true, long_help = "Parse installed mods")]
    parse_mods_installed: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ParseTarget {
    /// Parse a single raw file
    RawFile,
    /// Parse a single raw module info file
    RawModuleInfoFile,
    /// Parse a single raw module
    RawModule,
    /// Parse all, limited to locations if specified
    All,
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

    let target_path = clean_path(args.target_path.as_str());
    let output_path = clean_path(args.out_dir.as_str());

    let mut options = ParserOptions::new(target_path);
    options.set_output_path(output_path);

    let parse_target = args.parse;

    match parse_target {
        ParseTarget::RawFile => {
            options.set_job(ParsingJob::SingleRaw);
            dfraw_json_parser::parse_to_file(&options);
        }
        ParseTarget::RawModuleInfoFile => {
            options.set_job(ParsingJob::SingleModuleInfoFile);
            dfraw_json_parser::parse_module_info_file(&options);
        }
        ParseTarget::RawModule => {
            options.set_job(ParsingJob::SingleModule);
            dfraw_json_parser::parse_to_file(&options);
        }
        ParseTarget::All => {
            let mut locations: Vec<RawModuleLocation> = Vec::new();
            if args.parse_vanilla {
                locations.push(RawModuleLocation::Vanilla);
            }
            if args.parse_mods_downloaded {
                locations.push(RawModuleLocation::Mods);
            }
            if args.parse_mods_installed {
                locations.push(RawModuleLocation::InstalledMods);
            }
            options.set_job(dfraw_json_parser::options::ParsingJob::All);
            options.set_locations_to_parse(locations);

            dfraw_json_parser::parse_to_file(&options);
            dfraw_json_parser::parse_info_modules_to_file(&options);
        }
    }
}

fn clean_path(path: &str) -> PathBuf {
    let path = PathBuf::from(path);
    path.canonicalize().unwrap_or(path)
}
