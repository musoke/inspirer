#[macro_use(crate_version, crate_authors)]
extern crate clap;
extern crate inspirer;

#[macro_use]
extern crate slog;
extern crate slog_term;
use slog::DrainExt;

use clap::{App, Arg};

use std::fs::File;
use std::io::{Read, BufReader};
use std::io::{Write, BufWriter};

fn main() {

    // Initialize logging
    let drain = slog_term::streamer()
        .stderr()
        .build()
        .fuse();
    let root_logger = slog::Logger::root(drain, o!("version" => crate_version!()));
    info!(root_logger, "Application started");

    // Define CLI
    let matches = App::new("aux2bib")
        .version(crate_version!())
        .author(crate_authors!(",\n"))
        .about("gets BibTeX keys from Inspire")
        .arg(Arg::with_name("INPUT")
            .help("Sets the file from which to extract BibTeX keys")
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .help("Sets the file to which results should be appended")
            .index(2))
        .get_matches();

    // Get input from specified file or stdin
    let mut input_data = String::new();

    let mut input_file: File;
    let mut stdin = std::io::stdin();

    let reader: &mut Read = match matches.value_of("INPUT") {
        Some(file_name) => {
            info!(root_logger, "Reading from file";
                  "file_name" => file_name);
            input_file = File::open(file_name).expect("File not found");
            &mut input_file
        }
        None => {
            info!(root_logger, "Reading from stdin");
            &mut stdin
        }
    };
    let mut reader = BufReader::new(reader);
    reader.read_to_string(&mut input_data).unwrap();

    // Extract BibTeX tags from document
    let keys = inspirer::aux2key(input_data);
    info!(root_logger, "Extracted BibTeX keys";
          "number_of_keys" => keys.len());

    // Retrieve BibTeX entries from inspire.net
    info!(root_logger, "Retrieving entries");
    let mut bibtex_entries: Vec<String> = Vec::new();
    for key in keys {
        debug!(root_logger, "Retrieving record from inspire";
               "bibtex_key" => key);
        if let Some(bibtex_entry) = inspirer::fetch_bibtex_with_key(key) {
            bibtex_entries.push(bibtex_entry);
        }
    }

    // Write BibTeX entries to file or stdout
    let mut stdout = std::io::stdout();
    let mut output_file: std::fs::File;

    let writer: &mut Write = match matches.value_of("OUTPUT") {
        Some(file_name) => {
            info!(root_logger, "Writing to file";
                  "file_name" => file_name);
            output_file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(file_name)
                .unwrap();
            &mut output_file
        }
        None => {
            info!(root_logger, "Writing to stdout");
            // stdout.lock();
            &mut stdout
        }
    };

    let mut writer = BufWriter::new(writer);

    for bibtex_entry in bibtex_entries {
        writer.write_all(&bibtex_entry.as_bytes()).unwrap();
    }

    writer.flush().unwrap();

    info!(root_logger, "Done");
}
