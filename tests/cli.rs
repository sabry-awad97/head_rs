use std::error::Error;

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult<T> = Result<T, Box<dyn Error>>;

const PROGRAM_NAME: &'static str = "head_rs";

#[test]
fn test_no_file_or_stdin() -> TestResult<()> {
    let mut cmd = Command::cargo_bin(PROGRAM_NAME)?;
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("No file or stdin specified."));

    Ok(())
}

#[test]
fn test_with_file() -> TestResult<()> {
    let mut cmd = Command::cargo_bin(PROGRAM_NAME)?;
    cmd.arg("tests/data/sample.txt")
        .arg("-n")
        .arg("3")
        .assert()
        .success()
        .stdout(predicate::str::contains("1"))
        .stdout(predicate::str::contains("2"))
        .stdout(predicate::str::contains("3"));

    Ok(())
}

#[test]
fn test_with_stdin() -> TestResult<()> {
    let mut cmd = Command::cargo_bin(PROGRAM_NAME)?;
    cmd.write_stdin("hello\nworld\nfoo\nbar\n")
        .arg("-n")
        .arg("2")
        .arg("-s")
        .assert()
        .success()
        .stdout(predicate::str::contains("hello"))
        .stdout(predicate::str::contains("world"));

    Ok(())
}

#[test]
fn test_with_line_numbers() -> TestResult<()> {
    let mut cmd = Command::cargo_bin(PROGRAM_NAME)?;
    cmd.arg("tests/data/sample.txt")
        .arg("-n")
        .arg("3")
        .arg("-l")
        .assert()
        .success()
        .stdout(predicate::str::contains("1 1"))
        .stdout(predicate::str::contains("2 2"))
        .stdout(predicate::str::contains("3 3"));

    Ok(())
}
