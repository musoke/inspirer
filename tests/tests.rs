use std::env;
use std::process::{Command, Stdio};
use std::path::PathBuf;
use std::io::Write;

extern crate nom_bibtex;
use nom_bibtex::Bibtex;

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
    let path;
    if cfg!(not(windows)) {
        path = get_bin_dir().join("aux2bib");
    } else {
        path = get_bin_dir().join("aux2bib.exe");
    }
    if !path.is_file() {
        panic!("aux2bib binary {:?} was not found", path);
    }
    println!("{:?}", path);
    let mut cmd = Command::new(path);
    cmd.env_clear().stderr(Stdio::piped()).stdout(
        Stdio::piped(),
    );

    cmd
}

fn cmd_blg2bib() -> Command {
    let path;
    if cfg!(not(windows)) {
        path = get_bin_dir().join("blg2bib");
    } else {
        path = get_bin_dir().join("blg2bib.exe");
    }
    if !path.is_file() {
        panic!("blg2bib binary {:?} was not found", path);
    }
    let mut cmd = Command::new(path);
    cmd.env_clear().stderr(Stdio::piped()).stdout(
        Stdio::piped(),
    );

    cmd
}

#[cfg(not(windows))]
#[test]
fn aux2bib_runs() {
    let mut cmd = cmd_aux2bib().arg("--help").spawn().expect(
        "Failed to execute aux2bib",
    );

    let error_code = cmd.wait().expect("Failed to wait on aux2bib");

    assert!(error_code.success());
}

#[cfg(not(windows))]
#[test]
fn blg2bib_runs() {
    let mut cmd = cmd_blg2bib().arg("--help").spawn().expect(
        "Failed to execute blg2bib",
    );

    let error_code = cmd.wait().expect("Failed to wait on blg2bib");

    assert!(error_code.success());
}

#[cfg(not(windows))]
#[test]
fn aux2bib_stdin_empty() {
    let mut child = cmd_aux2bib().stdin(Stdio::piped()).spawn().expect(
        "Failed to execute aux2bib",
    );

    {
        let stdin = child.stdin.as_mut().expect("Failed to get stdin");
        stdin.write_all(b"").expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to wait on aux2bib");

    assert!(output.status.success());
    assert_eq!(output.stdout, []);
}

#[cfg(not(windows))]
#[test]
fn blg2bib_stdin_empty() {
    let mut child = cmd_blg2bib().stdin(Stdio::piped()).spawn().expect(
        "Failed to execute blg2bib",
    );

    {
        let stdin = child.stdin.as_mut().expect("Failed to get stdin");
        stdin.write_all(b"").expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to wait on aux2bib");

    assert!(output.status.success());
    assert_eq!(output.stdout, []);
}

#[cfg(not(windows))]
#[test]
fn aux2bib_stdin_bibtex() {
    let mut child = cmd_aux2bib().stdin(Stdio::piped()).spawn().expect(
        "Failed to execute aux2bib",
    );

    {
        let stdin = child.stdin.as_mut().expect("Failed to get stdin");
        stdin
            .write_all(
                b"\
\\relax 
\\citation{Higgs:2014aqa}
\\citation{Higgs:2015mei}
\\bibstyle{unsrt}
\\bibdata{test_bibtex}
        ",
            )
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to wait on aux2bib");

    assert!(output.status.success());

    let bibtex = Bibtex::parse(std::str::from_utf8(&output.stdout).unwrap()).unwrap();
    let bib = bibtex.bibliographies();

    // Output could conceivably change in future, so just check that some things are right
    assert_eq!(bib[0].entry_type(), "article");
    assert_eq!(bib[0].citation_key(), "Higgs:2014aqa");
    assert_eq!(
        bib[0].tags()[0],
        ("author".into(), "Higgs, Peter W.".into())
    );
    assert_eq!(bib[0].tags()[1], (
        "title".into(),
        "{Nobel Lecture: Evading the Goldstone theorem}"
            .into(),
    ));
    assert_eq!(bib[0].tags()[4], ("year".into(), "2014".into()));

    assert_eq!(bib[1].entry_type(), "article");
    assert_eq!(bib[1].citation_key(), "Higgs:2015mei");
    assert_eq!(bib[1].tags()[0], ("author".into(), "Higgs, P. W.".into()));
    assert_eq!(bib[1].tags()[1], (
        "title".into(),
        "{Evading the Goldstone theorem}".into(),
    ));
    assert_eq!(bib[1].tags()[4], ("year".into(), "2015".into()));
}
