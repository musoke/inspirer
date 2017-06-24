use std::env;
use std::process::Command;
use std::path::PathBuf;

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
    let path = get_bin_dir().join("aux2bib");
    if !path.is_file() {
        panic!("aux2bib binary not found");
    }
    let mut cmd = Command::new(path);

    cmd.env_clear();

    cmd
}

#[test]
fn aux2bib_runs() {
    let mut cmd = cmd_aux2bib().arg("--help").spawn().expect(
        "Failed to execute aux2bib",
    );

    let error_code = cmd.wait().expect("Failed to wait on aux2bib");

    assert!(error_code.success());

}
