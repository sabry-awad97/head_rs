use std::error::Error;

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult<T> = Result<T, Box<dyn Error>>;

const PROGRAM_NAME: &'static str = "head_rs";

#[test]
fn test_head_command() -> TestResult<()> {
    let mut cmd = Command::cargo_bin(PROGRAM_NAME)?;
    cmd.arg("tests/data/sample.txt").arg("-n").arg("3");

    let expected_output = "1\n2\n3\n";

    cmd.assert()
        .success()
        .stdout(predicate::str::starts_with(expected_output));

    Ok(())
}
