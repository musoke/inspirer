extern crate regex;
use regex::Regex;

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

    let keys: Vec<String> = regex.captures_iter(&input_data).map(
                            |c| c.get(1).unwrap().as_str().to_string()).collect();

    keys
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
