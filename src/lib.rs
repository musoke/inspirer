/// Re-export slog
///
/// Users of this library can, but don't have to use slog to build their own loggers
#[macro_use]
pub extern crate slog ;
extern crate slog_stdlog;

use slog::DrainExt;

extern crate regex;
use regex::Regex;

extern crate reqwest;
use reqwest::Url;

extern crate select;
use select::document::Document;
use select::predicate::Name;

use std::io::Read;

pub struct Inspirer {
    logger: slog::Logger,
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
        Inspirer {
            logger: logger.unwrap_or_else(|| slog::Logger::root(slog_stdlog::StdLog.fuse(), o!())),
        }
    }

    /// The `aux2key` function extracts TeX keys from LaTeX .aux files. These can be for either
    /// BibTeX or BibLaTeX.
    ///
    /// # Examples
    ///
    /// ## bibtex
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
    /// ## biber
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

        let regex = Regex::new(r"(\\citation|\\abx@aux@cite)\{([a-zA-Z]+:\d{4}[a-z]{2,3})\}")
            .unwrap();

        regex.captures_iter(&input_data)
            .map(|c| {
                     c.get(2)
                         .unwrap()
                         .as_str()
                         .to_string()
                 })
            .collect()
    }

    /// # The blg2key function extracts missing references from bibtex logs
    pub fn blg2key(&self, input_data: String) -> Vec<String> {

        let regex = Regex::new(r#"(Warning--|WARN - )I didn't find a database entry for ["']([a-zA-Z]+:\d{4}[a-z]{2,3})["']"#)
            .unwrap();

        regex.captures_iter(&input_data)
            .map(|c| {
                     c.get(2)
                         .unwrap()
                         .as_str()
                         .to_string()
                 })
            .collect()
    }

    /// Fetches BibTeX entries from inspire.net.
    ///
    /// # Examples
    ///
    /// ```
    /// let inspirer = inspirer::Inspirer::init(None);
    ///
    /// println!("{}", inspirer.fetch_bibtex_with_key(
    ///     "Abramovici:1992ah".to_string()).expect("Error"));
    /// ```
    pub fn fetch_bibtex_with_key(&self, key: String) -> Option<String> {

        let mut api_url: Url = Url::parse("https://inspirehep.net")
            .expect("Unable to parse API URL")
            .join("search")
            .unwrap();
        api_url.query_pairs_mut().append_pair("of", "hx").append_pair("p", &key);

        debug!(self.logger, "Querying inspire API";
               "URL" => api_url.to_string());
        let mut response = reqwest::get(api_url).expect("Failed to send get request");
        debug!(self.logger, "GET request completed";
               "HTTP response status" => response.status().to_string());

        let mut html = String::new();
        response.read_to_string(&mut html).expect("Failed to read response.");

        let document = Document::from(html.as_str());

        Some(document.find(Name("pre"))
                 .first()
                 .expect("No text found.")
                 .text())
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
