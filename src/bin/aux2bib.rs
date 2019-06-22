#[macro_use(crate_version, crate_authors)]
extern crate clap;
use inspirer;

use inspirer::InspirerError;
use std::error::Error;

use human_panic::setup_panic;

#[macro_use]
extern crate slog;
use slog::DrainExt;
use slog_term;

use clap::{App, Arg};

fn main() {
    setup_panic!();

    // Initialize logging
    let drain = slog_term::streamer().stderr().build().fuse();
    let root_logger = slog::Logger::root(drain, o!("version" => crate_version!()));

    if let Err(ref e) = run(&root_logger) {
        match e {
            _ => error!(root_logger, e.to_string();
                    "error" => format!("{}", match e.source(){
                        Some(e) => e.to_string(),
                        None => String::new(),
                    })
            ),
        }
        ::std::process::exit(1);
    }
}

fn run(root_logger: &slog::Logger) -> Result<(), InspirerError> {
    info!(root_logger, "Application started");

    // Initialize instance of InspirerLib
    let lib = inspirer::Inspirer::init(Some(root_logger.new(o!())));

    // Define CLI
    let matches = App::new("aux2bib")
        .version(crate_version!())
        .author(crate_authors!(",\n"))
        .about("gets BibTeX keys from Inspire")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the file from which to extract BibTeX keys")
                .index(1),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Sets the file to which results should be appended")
                .index(2),
        )
        .get_matches();

    // Get input from specified file or stdin
    let input_data = lib.get_input(matches.value_of("INPUT"))?;

    // Extract BibTeX tags from document
    let keys = lib.aux2key(input_data);
    info!(root_logger, "Extracted BibTeX keys";
          "number_of_keys" => keys.len());

    // Retrieve BibTeX entries from inspire.net
    info!(root_logger, "Retrieving entries");
    let mut bibtex_entries: Vec<String> = Vec::new();
    for key in keys {
        if let Some(bibtex_entry) = lib.bibtex(&key) {
            bibtex_entries.push(bibtex_entry);
        }
    }

    // Write BibTeX entries to file or stdout
    lib.put_output(matches.value_of("OUTPUT"), bibtex_entries)?;

    info!(root_logger, "Done");

    Ok(())
}
