use clap::Parser;
use std::path::Path;

mod parser;
mod server;

const HELP_RAWS_DIR: &str = "Specify the directory containing the raw files.

This usually is a directory named 'raw' in the save or game directory. 
If this is left unspecified, no raws will be parsed when running.";

const HELP_OUT_DIR: &str = "Specify the directory that the JSON database should be saved into.

If raw files are parsed, a JSON database (an array of objects) is
saved to disk in a location specified by this argument. This will
create an 'out.json' file in the directory specified by this argument.";

const HELP_SERVE: &str = "Include this flag to start a web server for the web search client.

Included in the repository is a 'www' folder with a small web client
that will fetch the JSON database created by this program (out.json)
and present it in a searchable manner to the user. 

If you include this flag, after any parsing is done, a tiny HTTP server
will start server files from the directory specified by 'out-dir' which
defaults to ./www";

const HELP_PORT: &str = "Specify the port to run the web server on.";

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Path to raw files directory
    #[clap(short, long, default_value_t = String::new(), long_help = HELP_RAWS_DIR)]
    raws_dir: String,

    /// Path to save JSON database
    #[clap(short, long, default_value_t = String::from("./www/"), long_help = HELP_OUT_DIR)]
    out_dir: String,

    /// Whether we should start a web server for the out_dir
    #[clap(short, long, takes_value = false, long_help = HELP_SERVE)]
    serve: bool,

    /// Port to serve the web client on
    #[clap(short, long, default_value_t = 4501, long_help = HELP_PORT)]
    port: u16,

    #[clap(
        short,
        long,
        takes_value = false,
        help = "Don't print out parsing status to console."
    )]
    quiet: bool,

    #[clap(
        short,
        long,
        takes_value = false,
        help = "Show even more information while parsing."
    )]
    verbose: bool,
}

#[derive(Copy, Clone)]
pub struct LogLevels {
    error: bool,
    warn: bool,
    info: bool,
    verbose: bool,
}

fn main() {
    let args = Args::parse();

    let logging = LogLevels {
        error: true,
        warn: true,
        info: !args.quiet,
        verbose: args.verbose,
    };

    if !args.raws_dir.is_empty() {
        // If a directory for raws was specified, we will parse what raws we find
        parser::parse_directory(
            args.raws_dir,
            Path::new(&args.out_dir).to_path_buf(),
            logging,
        );
    }

    if args.serve {
        server::serve_files(args.out_dir, args.port);
    }
}
