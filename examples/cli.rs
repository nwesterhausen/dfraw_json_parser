use dfraw_json_parser::parser::{object_types::ObjectType, raw_locations::RawModuleLocation};
use std::path::PathBuf;

const LONG_HELP: &str = "Usage: dfraw-json-parser [OPTIONS] <dwarf-fortress-path>

Default behavior:
    - Parse the vanilla raws
    - Parse all object types
    - Print a summary of the parsed raws
    - Save the parsed raws to 'parsed-raws.json'
    - Log at the 'info' level

The following options are supported:
    -c, --creature      Parse creature raws
    -p, --plant         Parse plant raws
    -e, --entity        Parse entity raws
    -i, --inorganic     Parse inorganic raws
    -g, --graphics      Parse graphics raws

    -s, --summary       Print a summary of the parsed raws

    -o, --output PATH   Set the output path for the parsed raws
        Default value: 'parsed-raws.json'

    -r, --raw PATH      Parse a raw file
        This can be included multiple times to parse multiple raw files
        directly. 

    -l, --legends PATH  Parse a legends export
        This can be included multiple times to parse multiple legends
        exports. These should be 'legends-plus' exports from DFHack.

    -m, --module PATH   Parse a raw module
        This can be included multiple times to parse multiple raw modules
        directly. This could be to specify a single raw module to parse, or
        to specify a raw module to parse in addition to the raw modules
        specified by the --vanilla, --mods, and --installed flags.

    -v, --verbose       Increase the verbosity of the output
        Default log level: 'info'

        This supports up to two levels of verbosity. The first level is
        'debug', and the second level is 'trace'. (e.g. '-vv' or '--verbose --verbose')

    -q, --quiet         Decrease the verbosity of the output
        Default log level: 'info'

        This supports up to two levels of verbosity. The first level is
        'warn', and the second level is 'error'. (e.g. '-qq' or '--quiet --quiet')

    --vanilla           Parse the vanilla raws
    --mods              Parse the raws from all mods
    --installed         Parse the raws from installed mods

    -h, --help              Print this help message
    -V, --version           Print the version number";

#[derive(Debug)]
struct Args {
    pub log_level: log::LevelFilter,
    pub locations: Vec<RawModuleLocation>,
    pub object_types: Vec<ObjectType>,
    pub legends_exports: Vec<PathBuf>,
    pub print_summary: bool,
    pub output_path: PathBuf,
    pub df_path: PathBuf,
    pub raw_file_paths: Vec<PathBuf>,
    pub raw_module_paths: Vec<PathBuf>,
}

impl std::default::Default for Args {
    fn default() -> Self {
        Self {
            log_level: log::LevelFilter::Info,
            locations: Vec::new(),
            object_types: Vec::new(),
            legends_exports: Vec::new(),
            print_summary: false,
            output_path: PathBuf::from("parsed-raws.json"),
            df_path: PathBuf::new(),
            raw_file_paths: Vec::new(),
            raw_module_paths: Vec::new(),
        }
    }
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    // Establish default values for the CLI arguments
    let mut args = Args::default();
    let mut include_graphics = false;

    let mut parser = lexopt::Parser::from_env();

    // Parse the CLI arguments
    while let Some(arg) = parser.next()? {
        match arg {
            Short('c') | Long("creature") => {
                args.object_types.push(ObjectType::Creature);
            }
            Short('p') | Long("plant") => {
                args.object_types.push(ObjectType::Plant);
            }
            Short('e') | Long("entity") => {
                args.object_types.push(ObjectType::Entity);
            }
            Short('i') | Long("inorganic") => {
                args.object_types.push(ObjectType::Inorganic);
            }

            Long("vanilla") => {
                args.locations.push(RawModuleLocation::Vanilla);
            }
            Long("mods") => {
                args.locations.push(RawModuleLocation::Mods);
            }
            Long("installed") => {
                args.locations.push(RawModuleLocation::InstalledMods);
            }

            Short('s') | Long("summary") => {
                args.print_summary = true;
            }
            Short('g') | Long("graphics") => {
                include_graphics = true;
            }

            Short('o') | Long("output") => {
                args.output_path = PathBuf::from(parser.value()?);
            }
            Short('r') | Long("raw") => {
                args.raw_file_paths.push(PathBuf::from(parser.value()?));
            }
            Short('l') | Long("legends") => {
                args.legends_exports.push(PathBuf::from(parser.value()?));
            }
            Short('m') | Long("module") => {
                args.raw_module_paths.push(PathBuf::from(parser.value()?));
            }

            Short('v') | Long("verbose") => {
                if args.log_level == log::LevelFilter::Info {
                    args.log_level = log::LevelFilter::Debug;
                } else {
                    args.log_level = log::LevelFilter::Trace;
                }
            }
            Short('q') | Long("quiet") => {
                if args.log_level == log::LevelFilter::Info {
                    args.log_level = log::LevelFilter::Warn;
                } else {
                    args.log_level = log::LevelFilter::Error;
                }
            }

            Short('h') | Long("help") => {
                println!("{LONG_HELP}");
                std::process::exit(0);
            }

            Value(val) if args.df_path.to_str().is_none() => {
                args.df_path = PathBuf::from(val);
            }

            _ => {
                println!("Unknown argument: {arg:?}");
            }
        }
    }

    // If no locations were specified, parse just vanilla
    if args.locations.is_empty() {
        args.locations.push(RawModuleLocation::Vanilla);
    }
    // If no object types were specified, parse all
    if args.object_types.is_empty() {
        args.object_types.push(ObjectType::Creature);
        args.object_types.push(ObjectType::Plant);
        args.object_types.push(ObjectType::Entity);
        args.object_types.push(ObjectType::Inorganic);
    }
    // Include graphic types if requested
    if include_graphics {
        args.object_types.push(ObjectType::Graphics);
        args.object_types.push(ObjectType::TilePage);
    }

    Ok(args)
}

pub fn main() -> Result<(), lexopt::Error> {
    let args = parse_args()?;

    println!("Parsed arguments: {args:#?}");

    Ok(())
}
