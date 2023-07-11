use assert_cmd::Command;
use predicates::prelude::predicate;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const DAY6_INPUT: &str = "./tests/inputs/input6.txt";

#[test]
fn problem1() -> TestResult {
    Command::cargo_bin("aoc15")?
        .arg("-p")
        .arg("6")
        .arg("-s")
        .arg("1")
        .arg("-i")
        .arg(DAY5_INPUT)
        .assert()
        .stdout(predicate::str::contains("255"));
    Ok(())
}

#[test]
fn problem2() -> TestResult {
    Command::cargo_bin("aoc15")?
        .arg("-p")
        .arg("6")
        .arg("-s")
        .arg("2")
        .arg("-i")
        .arg(DAY5_INPUT)
        .assert()
        .stdout(predicate::str::contains("55"));
    Ok(())
}
