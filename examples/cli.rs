use std::path::{Component, Path, PathBuf};

use clap::{Parser, ValueEnum};
use dfraw_json_parser::{
    RawModuleLocation, {ParserOptions, ParsingJob},
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

#[allow(clippy::struct_excessive_bools)]
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Args {
    // Verbose (debug) logging
    #[clap(short, long, long_help = "Enable verbose (debug) logging")]
    verbose: bool,

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
    #[clap(long, default_value_t = false, long_help = "Parse vanilla raws")]
    parse_vanilla: bool,

    /// Parse workshop mods location (downloaded mods)
    #[clap(
        long,
        default_value_t = false,
        long_help = "Parse downloaded workshop mods"
    )]
    parse_mods_downloaded: bool,

    /// Parse installed mods location
    #[clap(long, default_value_t = false, long_help = "Parse installed mods")]
    parse_mods_installed: bool,

    /// Include Metadata in output
    #[clap(
        long,
        default_value_t = false,
        long_help = "Include metadata in output"
    )]
    include_metadata: bool,
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

#[allow(clippy::too_many_lines)]
fn main() {
    let args = Args::parse();
    let log_level = if args.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

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
        .level(log_level)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        // Apply globally
        .apply()
        .expect("Failed to start logger");

    let target_path = normalize_path(Path::new(args.target_path.as_str()));
    let mut output_path = normalize_path(Path::new(args.out_dir.as_str()));

    // Check if output path is only a directory (and doesn't specify a file).
    // If so, we should add the default filename.
    if output_path.is_dir() {
        output_path.push("raws.json");
    }

    // It's possible the output file doesn't exist. We should touch an empty file to ensure it does.
    if !output_path.exists() {
        if let Err(e) = std::fs::File::create(&output_path) {
            log::error!("Unable to create output file: {}", e);
        }
    }

    let mut options = ParserOptions::new(target_path);
    options.set_output_path(&output_path);

    if args.include_metadata {
        options.attach_metadata_to_raws();
    }

    let parse_target = args.parse;

    match parse_target {
        ParseTarget::RawFile => {
            options.set_job(ParsingJob::SingleRaw);
            log::info!("Parsing options: {:#?}", options);
            dfraw_json_parser::parse_to_file(&options);
        }
        ParseTarget::RawModuleInfoFile => {
            options.set_job(ParsingJob::SingleModuleInfoFile);
            log::info!("Parsing options: {:#?}", options);
            dfraw_json_parser::parse_module_info_file(&options);
        }
        ParseTarget::RawModule => {
            options.set_job(ParsingJob::SingleModule);
            log::info!("Parsing options: {:#?}", options);
            dfraw_json_parser::parse_to_file(&options);
        }
        ParseTarget::All => {
            let mut locations: Vec<RawModuleLocation> = Vec::new();
            if args.parse_vanilla {
                log::debug!("Parsing options: {:#?}", options);
                locations.push(RawModuleLocation::Vanilla);
            }
            if args.parse_mods_downloaded {
                log::debug!("Parsing options: {:#?}", options);
                locations.push(RawModuleLocation::Mods);
            }
            if args.parse_mods_installed {
                log::debug!("Parsing options: {:#?}", options);
                locations.push(RawModuleLocation::InstalledMods);
            }
            options.set_job(ParsingJob::AllModulesInLocations);
            options.set_locations_to_parse(locations.clone());

            // If no locations were specified, parse just vanilla
            if locations.is_empty() {
                options.set_locations_to_parse(vec![RawModuleLocation::Vanilla]);
            }

            // If we have all 3 locations in the list,
            if locations.len() == 3_usize {
                // Check if the provided output_path ends with a filename (e.g. *.json or similar).
                if output_path.is_file() {
                    // Remove the filename from the path for parsing ALL
                    output_path.pop();
                    options.set_output_path(&output_path);
                }
            }

            log::info!("Parsing options: {:#?}", options);
            dfraw_json_parser::parse_to_file(&options);

            let mut module_json_fname = String::new();
            if output_path.is_file() {
                // Remove the filename portion of the path
                module_json_fname = format!(
                    "{}_modules.json",
                    output_path
                        .clone()
                        .file_name()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap()
                        .to_string()
                        .trim()
                        .replace(".json", "")
                );
                output_path.pop();
            }
            // Change the output path to be a modules.json file
            output_path.push(module_json_fname);
            options.set_output_path(&output_path);
            if !output_path.exists() {
                if let Err(e) = std::fs::File::create(&output_path) {
                    log::error!("Unable to create output file: {}", e);
                }
            }
            options.set_job(ParsingJob::AllModuleInfoFiles);
            dfraw_json_parser::parse_info_modules_to_file(&options);
        }
    }
}

/// The `normalize_path` function takes a path and returns a normalized version of it by removing
/// redundant components such as "." and "..".
///
/// Arguments:
///
/// * `path`: The `path` parameter is of type `&Path`, which represents a file or directory path.
///
/// Returns:
///
/// The function `normalize_path` returns a `PathBuf` object.
fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().copied() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}
