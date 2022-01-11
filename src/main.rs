use clap::Parser;
use std::path::Path;

mod parser;

const HELP_RAWS_DIR: &str = "Specify the directory containing the raw files.

This usually is a directory named 'raw' in the save or game 
directory. If this is left unspecified, no raws will be
parsed when running the program.";

const HELP_OUT_DIR: &str = "Specify the directory that the JSON database should be saved into.

If raw files are parsed, a JSON database (an array of objects) is
saved to disk in a location specified by this argument. This will
create an 'out.json' file in the directory specified by this argument.";

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Path to raw files directory
    #[clap(short, long, default_value_t = String::new(), long_help = HELP_RAWS_DIR)]
    raws_dir: String,

    /// Path to save JSON database
    #[clap(short, long, default_value_t = String::from("./www/"), long_help = HELP_OUT_DIR)]
    out_dir: String,
}

fn main() {
    let args = Args::parse();

    if !args.raws_dir.is_empty() {
        // If a directory for raws was specified, we will parse what raws we find
        parser::parser::parse_directory(args.raws_dir, Path::new(&args.out_dir).to_path_buf());
    }
}
