use clap::App;

use fluent_cli::parse_file;

fn main() {
    let matches = App::new("Fluent Parser")
        .version("1.0")
        .about("Parses FTL file into an AST")
        .args(&[
            clap::arg!(-s --silent "Disables error reporting"),
            clap::arg!(<FILE> "FTL file to parse"),
        ])
        .get_matches();

    let input = matches.value_of("FILE").unwrap();
    parse_file(input, matches.is_present("silent"));
}
