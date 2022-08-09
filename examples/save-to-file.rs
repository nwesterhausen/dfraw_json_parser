use simple_logger::SimpleLogger;

fn main() {
    let _l = SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init();

    dfraw_json_parser::parse_directory_to_json_file(
        "examples/ELEMENTMAN_AMETHYST.txt",
        &std::env::current_dir().unwrap().join("out"),
    );
    dfraw_json_parser::parse_directory_to_json_file(
        "examples/MONGOOSE.txt",
        &std::env::current_dir().unwrap().join("out"),
    );
}
