extern crate regex;
use regex::Regex;

extern crate reqwest;
use reqwest::Url;

extern crate select;
use select::document::Document;
use select::predicate::Name;

use std::io::Read;

/// The `aux2key` function extracts TeX keys from LaTeX .aux files. These can be for either BibTeX
/// or BibLaTeX.
///
/// # Examples
/// ```
/// use inspirer::aux2key;
///
/// let input =
/// r"\relax
/// \citation{Abramovici:1992ah}".to_string();
///
/// assert_eq!(aux2key(input), vec!("Abramovici:1992ah"));
/// ```
pub fn aux2key(input_data: String) -> Vec<String> {

    let regex = Regex::new(r"\\citation\{([a-zA-Z]+:\d{4}[a-z]{2,3})\}").unwrap();
        // Below regex is for Biber. Check correctness and write tests.
        // r"\\abx@aux@cite\{([a-zA-Z]+:\d{4}[a-z]{2,3})\}"

    regex.captures_iter(&input_data)
         .map(|c| c.get(1).unwrap().as_str().to_string())
         .collect()
}

/// Fetches bibtex entries from inspire.net.
///
/// # Examples
///
/// ```
/// use inspirer::fetch_bibtex_with_key;
///
/// println!("{}", fetch_bibtex_with_key("Abramovici:1992ah".to_string()).expect("Error"));
/// ```
pub fn fetch_bibtex_with_key(key: String) -> Option<String> {

    let api_url = Url::parse(&format!(r"https://inspirehep.net/search?of=hx&p={}/", key)).unwrap();

    let mut response = reqwest::get(api_url).expect("Failed to send get request");

    let mut html = String::new();
    response.read_to_string(&mut html).expect("Failed to read response.");

    let document = Document::from(html.as_str());

    Some(document.find(Name("pre")).first().expect("No text found.").text())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aux_bibtex_0_citations() {
        let input =
            r"\relax ".to_string();

        let output: Vec<String> = Vec::new();
        assert_eq!(aux2key(input), output);
    }

    #[test]
    fn test_aux_bibtex_1_citation() {
        let input =
            r"\relax
            \citation{Abramovici:1992ah}".to_string();

        assert_eq!(aux2key(input), vec!("Abramovici:1992ah"));
    }

    #[test]
    fn test_aux_bibtex_2_citation() {
        let input =
            r"\relax
            \citation{Abramovici:1992ah}
            \citation{Thorne:1992sdb}".to_string();

        assert_eq!(aux2key(input), vec!("Abramovici:1992ah", "Thorne:1992sdb"));
    }
}
