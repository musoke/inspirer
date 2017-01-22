#[macro_use(crate_version, crate_authors)]
extern crate clap;

use clap::{App, Arg};

fn main() {

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
}
