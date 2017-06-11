extern crate libinspire;

/// Re-export slog
///
/// Users of this library can, but don't have to use slog to build their own loggers
#[macro_use]
pub extern crate slog ;
extern crate slog_stdlog;

use slog::DrainExt;

extern crate regex;
use regex::Regex;

pub struct Inspirer {
    logger: slog::Logger,
    pub inspire: libinspire::Api,
}

pub enum Sources<'a> {
    Inspire(libinspire::RecID<'a>),
    Ads,
    Arxiv,
    None,
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

        regex
            .captures_iter(&input_data)
            .map(|c| c.get(2).unwrap().as_str().to_string())
            .collect()
    }

    /// # The blg2key function extracts missing references from bibtex logs
    pub fn blg2key(&self, input_data: String) -> Vec<String> {

        let regex = Regex::new(r#"(Warning--|WARN - )I didn't find a database entry for ["']([a-zA-Z]+:\d{4}[a-z]{2,3})["']"#)
            .unwrap();

        regex
            .captures_iter(&input_data)
            .map(|c| c.get(2).unwrap().as_str().to_string())
            .collect()
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
