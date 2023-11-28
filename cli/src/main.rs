use dfraw_json_parser::{ObjectType, ParserOptions, RawModuleLocation};

use std::path::{Path, PathBuf};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

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

    -P, --pretty        Pretty-print the parsed raws
        This is only used when saving the parsed raws to a file.

    -M, --metadata      Attach metadata to the parsed raws
        This includes the raws' file paths and other information about the
        raws' source.

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
    pub log_level: Level,
    pub locations: Vec<RawModuleLocation>,
    pub object_types: Vec<ObjectType>,
    pub legends_exports: Vec<PathBuf>,
    pub print_summary: bool,
    pub attach_metadata: bool,
    pub pretty_print: bool,
    pub output_path: PathBuf,
    pub df_path: PathBuf,
    pub raw_file_paths: Vec<PathBuf>,
    pub raw_module_paths: Vec<PathBuf>,
}

impl std::default::Default for Args {
    fn default() -> Self {
        Self {
            log_level: Level::INFO,
            locations: Vec::new(),
            object_types: Vec::new(),
            legends_exports: Vec::new(),
            print_summary: false,
            attach_metadata: false,
            pretty_print: false,
            output_path: PathBuf::from("parsed-raws.json"),
            df_path: PathBuf::new(),
            raw_file_paths: Vec::new(),
            raw_module_paths: Vec::new(),
        }
    }
}

#[allow(clippy::too_many_lines)]
fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

    // Establish default values for the CLI arguments
    let mut args = Args::default();
    let mut include_graphics = false;
    let mut df_path: Option<PathBuf> = None;

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
            Short('M') | Long("metadata") => {
                args.attach_metadata = true;
            }
            Short('P') | Long("pretty") => {
                args.pretty_print = true;
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
                if args.log_level == Level::INFO {
                    args.log_level = Level::DEBUG;
                } else {
                    args.log_level = Level::TRACE;
                }
            }
            Short('q') | Long("quiet") => {
                if args.log_level == Level::INFO {
                    args.log_level = Level::WARN;
                } else {
                    args.log_level = Level::ERROR;
                }
            }

            Short('h') | Long("help") => {
                println!("{LONG_HELP}");
                std::process::exit(0);
            }
            Short('V') | Long("version") => {
                println!("dfraw-json-parser v{}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }

            Value(val) if df_path.is_none() => {
                df_path = Some(PathBuf::from(val));
            }

            _ => {
                println!("Unknown argument: {arg:?}");
            }
        }
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

    // For all paths, resolve them to absolute paths
    args.df_path = to_absolute_path(&df_path.unwrap_or_default(), "dwarf fortress path")?;

    for path in &mut args.raw_file_paths {
        *path = to_absolute_path(path, "raw file")?;
    }
    for path in &mut args.raw_module_paths {
        *path = to_absolute_path(path, "raw module")?;
    }
    for path in &mut args.legends_exports {
        *path = to_absolute_path(path, "legends export")?;
    }

    // println!("cli.rs::parse_args: parsed arguments: {args:#?}");

    Ok(args)
}

fn to_absolute_path(path: &Path, descriptor: &str) -> Result<PathBuf, lexopt::Error> {
    path.canonicalize()
        .or(Err(lexopt::Error::Custom(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Unable to find {} path {}", descriptor, path.display()),
        )))))
}

/// The main function for the CLI
///
/// # Panics
///
/// This function will panic if the logger cannot be initialized.
///
/// # Errors
///
/// This function will produce errors if the arguments cannot be parsed.
pub fn main() -> Result<(), lexopt::Error> {
    let args = parse_args()?;

    // Initialize the logger
    // a builder for `FmtSubscriber`.
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(args.log_level)
        // make it pretty
        .compact()
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Build ParserOptions for the parser
    let options = ParserOptions {
        locations_to_parse: args.locations,
        raws_to_parse: args.object_types,
        attach_metadata_to_raws: args.attach_metadata,
        raw_files_to_parse: args.raw_file_paths,
        raw_modules_to_parse: args.raw_module_paths,
        legends_exports_to_parse: args.legends_exports,
        dwarf_fortress_directory: args.df_path,
        ..Default::default()
    };

    // Parse the raws
    let result = dfraw_json_parser::parse(&options).map_err(|e| {
        lexopt::Error::Custom(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to parse raws: {e:?}"),
        )))
    })?;

    // Print a summary of the parsed raws
    if args.print_summary {
        println!("Summary not implemented yet..");
    }

    // Save the parsed raws to a file
    dfraw_json_parser::util::write_raw_vec_to_file(
        &result.raws,
        &args.output_path,
        args.pretty_print,
    );

    Ok(())
}
