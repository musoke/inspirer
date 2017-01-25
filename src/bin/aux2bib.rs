#[macro_use(crate_version, crate_authors)]
extern crate clap;
extern crate inspirer;

use clap::{App, Arg};

use std::fs::File;
use std::io::{Read, BufReader};

fn main() {

    // Define CLI
    let matches = App::new("aux2bib")
                        .version(crate_version!())
                        .author(crate_authors!(",\n"))
                        .about("gets BibTeX keys from Inspire")
                        .arg(Arg::with_name("INPUT")
                             .help("Sets the input file to use")
                             .index(1))
                        .arg(Arg::with_name("OUTPUT")
                            .help("Sets the output file to use")
                            .index(2))
                        .get_matches();

    // Get input from specified file or stdin
    let mut input_data = String::new();

    match matches.value_of("INPUT") {
        Some(input_file_name) => {
            println!("Reading from file: {}", input_file_name);

            let f = File::open(input_file_name).expect("File not found");
            let mut reader = BufReader::new(f);
            reader.read_to_string(&mut input_data).unwrap();
        },
        None => {
            println!("Reading from stdin");

            let stdin = std::io::stdin();
            let stdin = stdin.lock();

            let mut reader = BufReader::new(stdin);
            reader.read_to_string(&mut input_data).unwrap();
        }
    }

    // Extract BibTeX tags from document
    let keys = inspirer::aux2key(input_data);

    // Retrieve BibTeX entries from inspire.net

    // Write BibTeX entries to file or stdout
    match matches.value_of("OUTPUT") {
        Some(output_file_name) => {
            println!("Writing to file: {}", output_file_name);
        },
        None => {
            println!("Writing to stdout");
        }
    }
}
