use assert_cmd::Command;
use predicates::prelude::predicate;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const DAY1_INPUT: &str = "./tests/inputs/input1.txt";

#[test]
fn problem1() -> TestResult {
    Command::cargo_bin("aoc15")?
        .arg("-p")
        .arg("1")
        .arg("-s")
        .arg("1")
        .arg("-i")
        .arg(DAY1_INPUT)
        .assert()
        .stdout(predicate::str::contains("280"));
    Ok(())
}

#[test]
fn problem2() -> TestResult {
    Command::cargo_bin("aoc15")?
        .arg("-p")
        .arg("1")
        .arg("-s")
        .arg("2")
        .arg("-i")
        .arg(DAY1_INPUT)
        .assert()
        .stdout(predicate::str::contains("1797"));
    Ok(())
}
