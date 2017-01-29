#[macro_use(crate_version, crate_authors)]
extern crate clap;
extern crate inspirer;

use clap::{App, Arg};

use std::fs::File;
use std::io::{Read, BufReader};
use std::io::{Write, BufWriter};

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
    println!("Retrieving entries...");
    let mut bibtex_entries: Vec<String> = Vec::new();
    for key in keys {
        if let Some(bibtex_entry) = inspirer::fetch_bibtex_with_key(key) {
            bibtex_entries.push(bibtex_entry);
        }
    }

    // Write BibTeX entries to file or stdout

    let mut stdout = std::io::stdout();
    let mut output_file: std::fs::File;

    let writer: &mut Write = match matches.value_of("OUTPUT") {
        Some(file_name) => {
            println!("Writing to file: {}", file_name);
            output_file = File::create(file_name).unwrap();
            &mut output_file
        },
        None            => {
            println!("Writing to stdout");
            // stdout.lock();
            &mut stdout
        },
    };

    let mut writer = BufWriter::new(writer);

    for bibtex_entry in bibtex_entries {
        writer.write(&bibtex_entry.as_bytes()).unwrap();
    }

    writer.flush().unwrap();
}
