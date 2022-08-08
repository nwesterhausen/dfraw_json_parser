use clap::Parser;
use std::path::Path;

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
}

fn main() {
    let args = Args::parse();

    if !args.raws_dir.is_empty() {
        // If a directory for raws was specified, we will parse what raws we find
        dfraw_json_parser::parse_directory_to_json_file(
            args.raws_dir.as_str(),
            &Path::new(&args.out_dir).to_path_buf(),
        );
    }

    if args.serve {
        serve_files(args.out_dir, args.port);
    }
}

fn serve_files(directory: String, port: u16) {
    let server_string = format!("localhost:{}", port);
    println!("Starting server at http://{}", server_string);

    rouille::start_server(&server_string, move |request| {
        {
            // The `match_assets` function tries to find a file whose name corresponds to the URL
            // of the request. The second parameter (`"."`) tells where the files to look for are
            // located.
            // In order to avoid potential security threats, `match_assets` will never return any
            // file outside of this directory even if the URL is for example `/../../foo.txt`.
            let response = rouille::match_assets(&request, &directory);

            // If a file is found, the `match_assets` function will return a response with a 200
            // status code and the content of the file. If no file is found, it will instead return
            // an empty 404 response.
            // Here we check whether if a file is found, and if so we return the response.
            if response.is_success() {
                return response;
            }
        }

        // This point of the code is reached only if no static file matched the request URL.

        // In a real website you probably want to serve non-static files here (with the `router!`
        // macro for example), but here we just return a 404 response.
        rouille::router!(request,
            (GET) (/) => {
                // If you requested '/' redirect to index.html
                rouille::Response::redirect_302("/index.html")
            },

            _ => rouille::Response::empty_404()
        )
    });
}
