extern crate libinspire;
extern crate libads;

/// Re-export slog
///
/// Users of this library can, but don't have to use slog to build their own loggers
#[macro_use]
pub extern crate slog ;
extern crate slog_stdlog;

use slog::DrainExt;

#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

use std::fs::File;
use std::io::{Read, BufReader};
use std::io::{Write, BufWriter};

pub struct Inspirer {
    logger: slog::Logger,
    inspire: libinspire::Api,
    ads: libads::Api,
}

impl Inspirer {
    /// Initialize 'Inspirer'
    ///
    /// Either provide a custom slog::Logger or default to the standard `log`
    /// crate.
    ///
    /// # Examples
    /// ```
    /// inspirer::Inspirer::init(None);
    /// ```
    pub fn init(logger: Option<slog::Logger>) -> Self {
        let logger = logger.unwrap_or_else(|| slog::Logger::root(slog_stdlog::StdLog.fuse(), o!()));

        Inspirer {
            logger: logger,
            // inspire: libinspire::Api::init(Some(logger)),
            inspire: libinspire::Api::init(None),
            ads: libads::Api::init(None),
        }
    }

    /// Read input from file or stdin
    ///
    /// # Examples
    /// ```
    /// ```
    pub fn get_input(&self, input_source: Option<&str>) -> String {
        let mut input_data = String::new();

        let mut input_file: File;
        let mut stdin = std::io::stdin();

        let reader: &mut Read = match input_source {
            Some(file_name) => {
                info!(self.logger, "Reading from file";
                      "file_name" => file_name);
                input_file = File::open(file_name).expect("File not found");
                &mut input_file
            }
            None => {
                info!(self.logger, "Reading from stdin");
                &mut stdin
            }
        };
        let mut reader = BufReader::new(reader);
        reader.read_to_string(&mut input_data).unwrap();

        input_data
    }

    /// Write output to file or stdout
    pub fn put_output(&self, output_dest: Option<&str>, output: Vec<String>) {
        let mut stdout = std::io::stdout();
        let mut output_file: std::fs::File;

        let writer: &mut Write = match output_dest {
            Some(file_name) => {
                info!(self.logger, "Writing to file";
                      "file_name" => file_name);
                output_file = std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(file_name)
                    .unwrap();
                &mut output_file
            }
            None => {
                info!(self.logger, "Writing to stdout");
                // stdout.lock();
                &mut stdout
            }
        };

        let mut writer = BufWriter::new(writer);

        for o in output {
            writer.write_all(&o.as_bytes()).unwrap();
        }

        writer.flush().unwrap();
    }

    /// The `aux2key` function extracts TeX keys from LaTeX .aux files. These can be for either
    /// BibTeX or BibLaTeX.
    ///
    /// # Examples
    ///
    /// ## bibtex
    ///
    /// Inspire-formatted BibTeX key:
    ///
    /// ```
    /// let inspirer = inspirer::Inspirer::init(None);
    ///
    /// let input =
    /// r"\relax
    /// \citation{Abramovici:1992ah}".to_string();
    ///
    /// assert_eq!(inspirer.aux2key(input), vec!("Abramovici:1992ah"));
    /// ```
    ///
    /// ADS-formatted BibTeX Key:
    ///
    /// ```
    /// let inspirer = inspirer::Inspirer::init(None);
    ///
    /// let input =
    /// r"\relax
    /// \citation{1998PhRvD..58h4020O}".to_string();
    ///
    /// assert_eq!(inspirer.aux2key(input), vec!("1998PhRvD..58h4020O"));
    /// ```
    ///
    /// ## biber
    ///
    /// Inspire-formatted BibLaTeX key:
    ///
    /// ```
    /// let inspirer = inspirer::Inspirer::init(None);
    ///
    /// let input =
    /// r"\relax
    /// \abx@aux@cite{Cutler:1992tc}".to_string();
    ///
    /// assert_eq!(inspirer.aux2key(input), vec!("Cutler:1992tc"));
    /// ```
    pub fn aux2key(&self, input_data: String) -> Vec<String> {

        lazy_static! {
            // TODO: check on the exact characters allowed in keys
            static ref AUX_REGEX: Regex = Regex::new(
                r"(\\citation|\\abx@aux@cite)\{(.+)\}").unwrap();
        }

        AUX_REGEX
            .captures_iter(&input_data)
            .map(|c| c.get(2).unwrap().as_str().to_string())
            // TODO just return the iterator: makes more sense with rayon
            .collect()
    }

    /// The blg2key function extracts missing references from bibtex logs
    ///
    /// # Examples
    ///
    /// ADS-formatted BibTeX key:
    ///
    /// ```
    /// let inspirer = inspirer::Inspirer::init(None);
    ///
    /// let input =
    /// r##"
    /// This is BibTeX, Version 0.99d (TeX Live 2016/Arch Linux)
    /// Capacity: max_strings=35307, hash_size=35307, hash_prime=30011
    /// The top-level auxiliary file: test_bibtex.aux
    /// The style file: unsrt.bst
    /// Database file #1: test_bibtex.bib
    /// Warning--I didn't find a database entry for "2015CQGra..32g4001L"
    /// You've used 0 entries,
    /// ....
    /// "##.to_string();
    ///
    /// assert_eq!(inspirer.blg2key(input), vec!("2015CQGra..32g4001L"));
    /// ```
    pub fn blg2key(&self, input_data: String) -> Vec<String> {

        lazy_static! {
            static ref BLG_REGEX: Regex = Regex::new(
                r#"(Warning--|WARN - )I didn't find a database entry for ["'](.+)["']"#,
            ).unwrap();
        }

        BLG_REGEX
            .captures_iter(&input_data)
            .map(|c| c.get(2).unwrap().as_str().to_string())
            // TODO just return the iterator: makes more sense with rayon
            .collect()
    }

    /// Fetch BibTeX entries
    pub fn bibtex(&self, key: &str) -> Option<String> {
        let key = Sources::from(key);

        match key {
            Sources::Inspire(k) => {
                debug!(self.logger, "Got Inspire record"; "key" => k.id);
                self.inspire.fetch_bibtex_with_key(k)
            }
            Sources::Ads(k) => {
                debug!(self.logger, "Got ADS record"; "key" => k.bibcode);
                self.ads.fetch_bibtex_with_key(k)
            }
            _ => {
                // debug!(self.logger, "Unknown record source"; "key" => key);
                debug!(self.logger, "Unknown record source");
                None
            }
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum Sources<'a> {
    Inspire(libinspire::RecID<'a>),
    Ads(libads::BibCode<'a>),
    Arxiv,
    None,
}

/// Guess a likely source for a BibTeX key
///
/// Returns `Sources::None` if unable to make a good guess.
///
/// # Examples
/// ```
/// extern crate inspirer;
/// extern crate libinspire;
/// let inspirer = inspirer::Inspirer::init(None);
///
/// assert_eq!(
///     inspirer::Sources::from("Randall:1999ee"),
///     inspirer::Sources::Inspire(libinspire::RecID::new("Randall:1999ee").unwrap())
/// );
/// ```
///
/// ```
/// extern crate inspirer;
/// extern crate libads;
/// let inspirer = inspirer::Inspirer::init(None);
///
/// assert_eq!(
///     inspirer::Sources::from("1999PhRvL..83.3370R"),
///     inspirer::Sources::Ads(libads::BibCode::new("1999PhRvL..83.3370R").unwrap())
/// );
/// ```
impl<'a> From<&'a str> for Sources<'a> {
    fn from(s: &'a str) -> Sources<'a> {
        if libinspire::validate_recid(s) {
            Sources::Inspire(libinspire::RecID::new(s).unwrap())
        } else if libads::validate_bib_code(s) {
            Sources::Ads(libads::BibCode::new(s).unwrap())
        } else {
            Sources::None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aux_bibtex_0_citations() {
        let input = r"\relax ".to_string();

        let output: Vec<String> = Vec::new();
        assert_eq!(Inspirer::init(None).aux2key(input), output);
    }

    #[test]
    fn test_aux_bibtex_1_citation() {
        let input = r"\relax
            \citation{Abramovici:1992ah}"
                .to_string();

        assert_eq!(Inspirer::init(None).aux2key(input),
                   vec!["Abramovici:1992ah"]);
    }

    #[test]
    fn test_aux_bibtex_2_citation() {
        let input = r"\relax
            \citation{Abramovici:1992ah}
            \citation{Thorne:1992sdb}"
                .to_string();

        assert_eq!(Inspirer::init(None).aux2key(input),
                   vec!["Abramovici:1992ah", "Thorne:1992sdb"]);
    }
}
