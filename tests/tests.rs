use std::env;
use std::fs::{copy, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use nom_bibtex::Bibtex;

use tempdir::TempDir;

mod text;

fn read_file(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).unwrap();
    let mut content = Vec::new();
    file.read_to_end(&mut content).unwrap();

    content
}

fn get_bin_dir() -> PathBuf {
    env::current_exe()
        .expect("test bin's directory")
        .parent()
        .expect("test bin's parent directory")
        .parent()
        .expect("executable's directory")
        .to_path_buf()
}

fn cmd_aux2bib() -> Command {
    let path = if cfg!(not(windows)) {
        get_bin_dir().join("aux2bib")
    } else {
        get_bin_dir().join("aux2bib.exe")
    };
    if !path.is_file() {
        panic!("aux2bib binary {:?} was not found", path);
    }
    let mut cmd = Command::new(path);
    cmd.env_clear()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped());

    cmd
}

fn cmd_blg2bib() -> Command {
    let path = if cfg!(not(windows)) {
        get_bin_dir().join("blg2bib")
    } else {
        get_bin_dir().join("blg2bib.exe")
    };
    if !path.is_file() {
        panic!("blg2bib binary {:?} was not found", path);
    }
    let mut cmd = Command::new(path);
    cmd.env_clear()
        .stderr(Stdio::piped())
        .stdout(Stdio::piped());

    cmd
}

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
    let child = cmd_aux2bib()
        .arg("--help")
        .spawn()
        .expect("Failed to execute aux2bib");

    let output = child.wait_with_output().expect("Failed to wait on aux2bib");

    println!("{:?}", output);
    assert!(output.status.success());
}

#[cfg(not(windows))]
#[test]
fn blg2bib_runs() {
    let child = cmd_blg2bib()
        .arg("--help")
        .spawn()
        .expect("Failed to execute blg2bib");

    let output = child.wait_with_output().expect("Failed to wait on aux2bib");

    println!("{:?}", output);
    assert!(output.status.success());
}

#[cfg(not(windows))]
#[test]
fn aux2bib_stdin_stdout_empty() {
    let mut child = cmd_aux2bib()
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to execute aux2bib");

    {
        let stdin = child.stdin.as_mut().expect("Failed to get stdin");
        stdin.write_all(b"").expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to wait on aux2bib");

    println!("{:?}", output);
    assert!(output.status.success());
    assert_eq!(output.stdout, []);
}

#[cfg(not(windows))]
#[test]
fn blg2bib_stdin_stdout_empty() {
    let mut child = cmd_blg2bib()
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to execute blg2bib");

    {
        let stdin = child.stdin.as_mut().expect("Failed to get stdin");
        stdin.write_all(b"").expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to wait on aux2bib");

    println!("{:?}", output);
    assert!(output.status.success());
    assert_eq!(output.stdout, []);
}

#[cfg(not(windows))]
#[test]
fn aux2bib_stdin_stdout_bibtex() {
    let mut child = cmd_aux2bib()
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to execute aux2bib");

    {
        let input = read_file("example_files/test_bibtex.aux");
        let stdin = child.stdin.as_mut().expect("Failed to get stdin");
        stdin.write_all(&input).expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to wait on aux2bib");

    println!("{:?}", output);
    assert!(output.status.success());

    let bibtex = Bibtex::parse(std::str::from_utf8(&output.stdout).unwrap())
        .expect("Valid bibtex file content");

    check_output_aux_bibtex(&bibtex);
}

#[cfg(not(windows))]
#[test]
fn aux2bib_stdin_stdout_biblatex() {
    let mut child = cmd_aux2bib()
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to execute aux2bib");

    {
        let input = read_file("example_files/test_biber.aux");
        let stdin = child.stdin.as_mut().expect("Failed to get stdin");
        stdin.write_all(&input).expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to wait on aux2bib");

    println!("{:?}", output);
    assert!(output.status.success());

    let bibtex_raw = &[
        text::MONTH_STRINGS,
        std::str::from_utf8(&output.stdout).unwrap(),
    ]
    .join("\n");
    let bibtex = Bibtex::parse(bibtex_raw).expect("Valid bibtex file content");

    check_output_aux_biblatex(&bibtex);
}

#[cfg(not(windows))]
#[test]
fn blg2bib_stdin_stdout_bibtex() {
    let mut child = cmd_blg2bib()
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to execute aux2bib");

    {
        let input = read_file("example_files/test_bibtex.blg");
        let stdin = child.stdin.as_mut().expect("Failed to get stdin");
        stdin.write_all(&input).expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to wait on aux2bib");

    println!("{:?}", output);
    assert!(output.status.success());

    let bibtex = Bibtex::parse(std::str::from_utf8(&output.stdout).unwrap())
        .expect("Valid bibtex file content");

    check_output_blg_bibtex(&bibtex);
}

#[cfg(not(windows))]
#[test]
fn blg2bib_stdin_stdout_biblatex() {
    let mut child = cmd_blg2bib()
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to execute blg2bib");

    {
        let input = read_file("example_files/test_biber.blg");
        let stdin = child.stdin.as_mut().expect("Failed to get stdin");
        stdin.write_all(&input).expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to wait on aux2bib");

    println!("{:?}", output);
    assert!(output.status.success());

    let bibtex_raw = &[
        text::MONTH_STRINGS,
        std::str::from_utf8(&output.stdout).unwrap(),
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

    let child = cmd_aux2bib()
        .current_dir(tmp_dir.path())
        .arg(filename_in)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to execute aux2bib");

    let output = child.wait_with_output().expect("Failed to wait on aux2bib");

    println!("{:?}", output);
    assert!(output.status.success());

    let bibtex = Bibtex::parse(std::str::from_utf8(&output.stdout).unwrap())
        .expect("Valid bibtex file content");

    check_output_aux_bibtex(&bibtex);
}

#[cfg(not(windows))]
#[test]
// Test for panic when the input file does not exist
fn aux2bib_file_stdout_bibtex_input_no_exist() {
    let filename_in = "test_bibtex.aux";

    let tmp_dir = TempDir::new("inspirer_test").expect("Failed to create tmp_dir");

    let child = cmd_aux2bib()
        .current_dir(tmp_dir.path())
        .arg(filename_in)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to execute aux2bib");

    let output = child.wait_with_output().expect("Failed to wait on aux2bib");

    println!("{:?}", output);
    assert_eq!(output.status.code().expect("Process exited"), 1);
    // Check that there is nothing written to stdout
    assert_eq!(output.stdout, []);
}

#[cfg(not(windows))]
#[test]
fn aux2bib_file_file_bibtex() {
    let filename_in = "test_bibtex.aux";
    let filename_out = "autobib.bib";

    let tmp_dir = TempDir::new("inspirer_test").expect("Failed to create tmp_dir");
    copy(
        Path::new("example_files").join(filename_in),
        tmp_dir.path().join(filename_in),
    )
    .expect("Failed to copy test input");

    let child = cmd_aux2bib()
        .current_dir(tmp_dir.path())
        .arg(filename_in)
        .arg(filename_out)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to execute aux2bib");

    let output = child.wait_with_output().expect("Failed to wait on aux2bib");
    println!("{:?}", output);
    assert!(output.status.success());
    // Check that there is nothing written to stdout
    assert_eq!(output.stdout, []);

    let mut output = Vec::new();
    let mut output_file =
        File::open(tmp_dir.path().join(filename_out)).expect("Output file not written");
    output_file
        .read_to_end(&mut output)
        .expect("Failed to read output file");

    let bibtex =
        Bibtex::parse(std::str::from_utf8(&output).unwrap()).expect("Valid bibtex file content");

    check_output_aux_bibtex(&bibtex);
}
