extern crate clap;

use clap::{App, Arg};

fn main() {

    let matches = App::new("aux2bib")
                        .version("0.1.0")
                        .author("Nathan Musoke")
                        .about("gets BibTeX keys from Inspire")
                        .arg(Arg::with_name("INPUT")
                             .help("Sets the input file to use")
                             .index(1))
                        .arg(Arg::with_name("OUTPUT")
                            .help("Sets the output file to use")
                            .index(2))
                        .get_matches();
}
