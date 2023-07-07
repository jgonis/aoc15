use assert_cmd::Command;
use predicates::prelude::predicate;
use std::error::Error;

type TestResult = Result<(), Box<dyn Error>>;

const DAY2_INPUT: &str = "./tests/inputs/input2.txt";

#[test]
fn problem1() -> TestResult {
    Command::cargo_bin("aoc15")?
        .arg("-p")
        .arg("2")
        .arg("-s")
        .arg("1")
        .arg("-i")
        .arg(DAY2_INPUT)
        .assert()
        .stdout(predicate::str::contains("1606483"));
    Ok(())
}

#[test]
fn problem2() -> TestResult {
    Command::cargo_bin("aoc15")?
        .arg("-p")
        .arg("2")
        .arg("-s")
        .arg("2")
        .arg("-i")
        .arg(DAY2_INPUT)
        .assert()
        .stdout(predicate::str::contains("3842356"));
    Ok(())
}
