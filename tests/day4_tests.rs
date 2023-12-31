use assert_cmd::Command;
use predicates::prelude::predicate;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const DAY4_INPUT: &str = "./tests/inputs/input4.txt";

#[test]
fn problem1() -> TestResult {
    Command::cargo_bin("aoc15")?
        .arg("-p")
        .arg("4")
        .arg("-s")
        .arg("1")
        .arg("-i")
        .arg(DAY4_INPUT)
        .assert()
        .stdout(predicate::str::contains("282749"));
    Ok(())
}

#[test]
fn problem2() -> TestResult {
    Command::cargo_bin("aoc15")?
        .arg("-p")
        .arg("4")
        .arg("-s")
        .arg("2")
        .arg("-i")
        .arg(DAY4_INPUT)
        .assert()
        .stdout(predicate::str::contains("9962624"));
    Ok(())
}
