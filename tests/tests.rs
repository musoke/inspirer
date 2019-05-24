use std::fs::{copy, File};
use std::io::Read;
use std::path::Path;
use std::process::Command;

use assert_cmd::prelude::*;
use nom_bibtex::Bibtex;

use tempdir::TempDir;

mod text;

fn check_output_aux_bibtex(bibtex: &Bibtex<'_>) {
    let bib = bibtex.bibliographies();
    assert_eq!(2, bib.len());

    // Output could conceivably change in future, so just check that some things are right
    assert_eq!(bib[0].entry_type(), "article");
    assert_eq!(bib[0].citation_key(), "Higgs:2014aqa");
    assert_eq!(
        bib[0].tags()[0],
        ("author".into(), "Higgs, Peter W.".into())
    );
    assert_eq!(
        bib[0].tags()[1],
        (
            "title".into(),
            "{Nobel Lecture: Evading the Goldstone theorem}".into(),
        )
    );
    assert_eq!(bib[0].tags()[4], ("year".into(), "2014".into()));

    assert_eq!(bib[1].entry_type(), "article");
    assert_eq!(bib[1].citation_key(), "Higgs:2015mei");
    assert_eq!(bib[1].tags()[0], ("author".into(), "Higgs, P. W.".into()));
    assert_eq!(
        bib[1].tags()[1],
        ("title".into(), "{Evading the Goldstone theorem}".into(),)
    );
    assert_eq!(bib[1].tags()[4], ("year".into(), "2015".into()));
}

fn check_output_aux_biblatex(bibtex: &Bibtex<'_>) {
    let bib = bibtex.bibliographies();
    assert_eq!(4, bib.len());

    // Output could conceivably change in future, so just check that some things are right
    assert_eq!(bib[2].entry_type(), "article");
    assert_eq!(bib[2].citation_key(), "Guth:1980zm");
    assert_eq!(bib[2].tags()[0], ("author".into(), "Guth, Alan H.".into()));
    assert_eq!(bib[2].tags()[1], (
        "title".into(),
        "{The Inflationary Universe: A Possible Solution to the\n                        Horizon and Flatness Problems}"
            .into(),
    ));
    assert_eq!(bib[2].tags()[4], ("year".into(), "1981".into()));

    assert_eq!(bib[0].entry_type(), "ARTICLE");
    assert_eq!(bib[0].citation_key(), "1982PhRvL..48.1220A");
    assert_eq!(
        bib[0].tags()[1],
        (
            "title".into(),
            "{Cosmology for grand unified theories with radiatively induced symmetry breaking}"
                .into(),
        )
    );
    assert_eq!(bib[0].tags()[4], ("year".into(), "1982".into()));
}

fn check_output_blg_bibtex(bibtex: &Bibtex<'_>) {
    let bib = bibtex.bibliographies();
    assert_eq!(2, bib.len());

    // Output could conceivably change in future, so just check that some things are right
    assert_eq!(bib[0].entry_type(), "article");
    assert_eq!(bib[0].citation_key(), "Higgs:2014aqa");
    assert_eq!(
        bib[0].tags()[0],
        ("author".into(), "Higgs, Peter W.".into())
    );
    assert_eq!(
        bib[0].tags()[1],
        (
            "title".into(),
            "{Nobel Lecture: Evading the Goldstone theorem}".into(),
        )
    );
    assert_eq!(bib[0].tags()[4], ("year".into(), "2014".into()));

    assert_eq!(bib[1].entry_type(), "article");
    assert_eq!(bib[1].citation_key(), "Higgs:2015mei");
    assert_eq!(bib[1].tags()[0], ("author".into(), "Higgs, P. W.".into()));
    assert_eq!(
        bib[1].tags()[1],
        ("title".into(), "{Evading the Goldstone theorem}".into(),)
    );
    assert_eq!(bib[1].tags()[4], ("year".into(), "2015".into()));
}

fn check_output_blg_biblatex(bibtex: &Bibtex<'_>) {
    let bib = bibtex.bibliographies();
    assert_eq!(4, bib.len());

    // Output could conceivably change in future, so just check that some things are right
    assert_eq!(bib[2].entry_type(), "article");
    assert_eq!(bib[2].citation_key(), "Guth:1980zm");
    assert_eq!(bib[2].tags()[0], ("author".into(), "Guth, Alan H.".into()));
    assert_eq!(bib[2].tags()[1], (
        "title".into(),
        "{The Inflationary Universe: A Possible Solution to the\n                        Horizon and Flatness Problems}"
            .into(),
    ));
    assert_eq!(bib[2].tags()[4], ("year".into(), "1981".into()));

    assert_eq!(bib[0].entry_type(), "ARTICLE");
    assert_eq!(bib[0].citation_key(), "1982PhRvL..48.1220A");
    assert_eq!(
        bib[0].tags()[1],
        (
            "title".into(),
            "{Cosmology for grand unified theories with radiatively induced symmetry breaking}"
                .into(),
        )
    );
    assert_eq!(bib[0].tags()[4], ("year".into(), "1982".into()));
}

#[cfg(not(windows))]
#[test]
fn aux2bib_runs() {
    let mut cmd = Command::cargo_bin("aux2bib").unwrap();
    cmd.arg("--help");
    cmd.assert().success();
}

#[cfg(not(windows))]
#[test]
fn blg2bib_runs() {
    let mut cmd = Command::cargo_bin("blg2bib").unwrap();
    cmd.arg("--help");
    cmd.assert().success();
}

#[cfg(not(windows))]
#[test]
fn aux2bib_stdin_stdout_empty() {
    let mut cmd = Command::cargo_bin("aux2bib").unwrap();
    cmd.with_stdin().buffer("");

    cmd.assert().success().stdout("");
}

#[cfg(not(windows))]
#[test]
fn blg2bib_stdin_stdout_empty() {
    let mut cmd = Command::cargo_bin("blg2bib").unwrap();
    cmd.with_stdin().buffer("");

    cmd.assert().success().stdout("");
}

#[cfg(not(windows))]
#[test]
fn aux2bib_stdin_stdout_bibtex() {
    let mut cmd = Command::cargo_bin("aux2bib").unwrap();
    let path = Path::new("example_files").join("test_bibtex.aux");

    let assert = cmd
        .with_stdin()
        .path(path)
        .expect("example input file exists")
        .assert()
        .success();

    let bibtex = Bibtex::parse(std::str::from_utf8(&assert.get_output().stdout).unwrap())
        .expect("Valid bibtex file content");

    check_output_aux_bibtex(&bibtex);
}

#[cfg(not(windows))]
#[test]
fn aux2bib_stdin_stdout_biblatex() {
    let mut cmd = Command::cargo_bin("aux2bib").unwrap();
    let path = Path::new("example_files").join("test_biber.aux");

    let assert = cmd
        .with_stdin()
        .path(path)
        .expect("example input file exists")
        .assert()
        .success();

    let bibtex_raw = &[
        text::MONTH_STRINGS,
        std::str::from_utf8(&assert.get_output().stdout).unwrap(),
    ]
    .join("\n");
    let bibtex = Bibtex::parse(bibtex_raw).expect("Valid bibtex file content");

    check_output_aux_biblatex(&bibtex);
}

#[cfg(not(windows))]
#[test]
fn blg2bib_stdin_stdout_bibtex() {
    let mut cmd = Command::cargo_bin("blg2bib").unwrap();
    let path = Path::new("example_files").join("test_bibtex.blg");

    let assert = cmd
        .with_stdin()
        .path(path)
        .expect("example input file exists")
        .assert()
        .success();

    let bibtex = Bibtex::parse(std::str::from_utf8(&assert.get_output().stdout).unwrap())
        .expect("Valid bibtex file content");

    check_output_blg_bibtex(&bibtex);
}

#[cfg(not(windows))]
#[test]
fn blg2bib_stdin_stdout_biblatex() {
    let mut cmd = Command::cargo_bin("blg2bib").unwrap();
    let path = Path::new("example_files").join("test_biber.blg");

    let assert = cmd
        .with_stdin()
        .path(path)
        .expect("example input file exists")
        .assert()
        .success();

    let bibtex_raw = &[
        text::MONTH_STRINGS,
        std::str::from_utf8(&assert.get_output().stdout).unwrap(),
    ]
    .join("\n");
    let bibtex = Bibtex::parse(bibtex_raw).expect("Valid bibtex file content");

    check_output_blg_biblatex(&bibtex);
}

#[cfg(not(windows))]
#[test]
fn aux2bib_file_stdout_bibtex() {
    let filename_in = "test_bibtex.aux";

    let tmp_dir = TempDir::new("inspirer_test").expect("Failed to create tmp_dir");
    copy(
        Path::new("example_files").join(filename_in),
        tmp_dir.path().join(filename_in),
    )
    .expect("Failed to copy test input");

    let mut cmd = Command::cargo_bin("aux2bib").unwrap();
    cmd.current_dir(tmp_dir.path()).arg(filename_in);

    let assert = cmd.assert().success();

    let bibtex = Bibtex::parse(std::str::from_utf8(&assert.get_output().stdout).unwrap())
        .expect("Valid bibtex file content");

    check_output_aux_bibtex(&bibtex);
}

#[cfg(not(windows))]
#[test]
/// Test for panic when the input file does not exist
fn aux2bib_file_stdout_bibtex_input_no_exist() {
    let filename_in = "test_bibtex.aux";

    let tmp_dir = TempDir::new("inspirer_test").expect("Failed to create tmp_dir");

    let mut cmd = Command::cargo_bin("aux2bib").unwrap();
    cmd.current_dir(tmp_dir.path()).arg(filename_in);

    cmd.assert().failure().code(1).stdout("");
}

#[cfg(not(windows))]
#[test]
/// Test reading from file and outputting to another file
fn aux2bib_file_file_bibtex() {
    let filename_in = "test_bibtex.aux";
    let filename_out = "autobib.bib";

    let tmp_dir = TempDir::new("inspirer_test").expect("Failed to create tmp_dir");
    copy(
        Path::new("example_files").join(filename_in),
        tmp_dir.path().join(filename_in),
    )
    .expect("Failed to copy test input");

    let mut cmd = Command::cargo_bin("aux2bib").unwrap();
    cmd.current_dir(tmp_dir.path())
        .arg(filename_in)
        .arg(filename_out);

    cmd.assert().success().stdout("");

    let mut output_string = String::new();
    let mut output_file =
        File::open(tmp_dir.path().join(filename_out)).expect("Output file not written");
    output_file
        .read_to_string(&mut output_string)
        .expect("Failed to read output file");

    let bibtex = Bibtex::parse(&output_string).expect("Valid bibtex file content");

    check_output_aux_bibtex(&bibtex);
}
