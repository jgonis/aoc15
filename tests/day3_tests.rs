use assert_cmd::Command;
use predicates::prelude::predicate;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const DAY3_INPUT: &str = "./tests/inputs/input3.txt";

#[test]
fn problem1() -> TestResult {
    Command::cargo_bin("aoc15")?
        .arg("-p")
        .arg("3")
        .arg("-s")
        .arg("1")
        .arg("-i")
        .arg(DAY3_INPUT)
        .assert()
        .stdout(predicate::str::contains("2081"));
    Ok(())
}

#[test]
fn problem2() -> TestResult {
    Command::cargo_bin("aoc15")?
        .arg("-p")
        .arg("3")
        .arg("-s")
        .arg("2")
        .arg("-i")
        .arg(DAY3_INPUT)
        .assert()
        .stdout(predicate::str::contains("2341"));
    Ok(())
}
