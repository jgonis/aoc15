use assert_cmd::Command;
use predicates::prelude::predicate;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const DAY7_INPUT: &str = "./tests/inputs/input7.txt";

#[test]
fn problem1() -> TestResult {
    Command::cargo_bin("aoc15")?
        .arg("-p")
        .arg("7")
        .arg("-s")
        .arg("1")
        .arg("-i")
        .arg(DAY7_INPUT)
        .assert()
        .stdout(predicate::str::contains("377891"));
    Ok(())
}

#[test]
fn problem2() -> TestResult {
    Command::cargo_bin("aoc15")?
        .arg("-p")
        .arg("7")
        .arg("-s")
        .arg("2")
        .arg("-i")
        .arg(DAY7_INPUT)
        .assert()
        .stdout(predicate::str::contains("14110788"));
    Ok(())
}
