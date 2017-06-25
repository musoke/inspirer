use std::env;
use std::process::{Command, Stdio};
use std::path::PathBuf;
use std::io::Write;

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
    // Output could conceivably change in future, so just check that some things are there
    assert!(
        std::str::from_utf8(&output.stdout).unwrap().contains(
            "@article{Higgs:2014aqa,",
        ) &&
            std::str::from_utf8(&output.stdout).unwrap().contains(
                "Higgs, Peter W.",
            ) &&
            std::str::from_utf8(&output.stdout).unwrap().contains(
                "Nobel Lecture: Evading the Goldstone theorem",
            ) &&
            std::str::from_utf8(&output.stdout).unwrap().contains(
                "2014",
            )
    );
    assert!(
        std::str::from_utf8(&output.stdout).unwrap().contains(
            "@article{Higgs:2015mei,",
        ) &&
            std::str::from_utf8(&output.stdout).unwrap().contains(
                "Higgs, P. W.",
            ) &&
            std::str::from_utf8(&output.stdout).unwrap().contains(
                "Evading the Goldstone theorem",
            ) &&
            std::str::from_utf8(&output.stdout).unwrap().contains(
                "2015",
            )
    );
}
