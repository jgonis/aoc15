use clap::{value_parser, Arg, Command};
use std::error::Error;
use std::path::PathBuf;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

mod day6;
mod day7;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = Command::new("aoc15")
        .version("0.0.1")
        .author("Jeff Gonis <jeffgonis@fastmail.com")
        .about("Going through Advent of Code 2015 in Rust")
        .arg(
            Arg::new("problem")
                .short('p')
                .long("problem")
                .required(true)
                .value_parser(value_parser!(u64)),
        )
        .arg(
            Arg::new("subproblem")
                .short('s')
                .long("subproblem")
                .required(true)
                .value_parser(value_parser!(u64)),
        )
        .arg(
            Arg::new("input_file")
                .short('i')
                .long("input")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        );
    let matches = cmd.get_matches();

    let problem = matches
        .get_one::<u64>("problem")
        .ok_or("didn't receive the problem argument")?;
    let sub_problem = matches
        .get_one::<u64>("subproblem")
        .ok_or("didn't receive a subproblem argument")?;
    let input_path = matches
        .get_one::<PathBuf>("input_file")
        .ok_or("didn't receive a input file argument")?;

    let contents = std::fs::read_to_string(input_path)?;
    let result = match (problem, sub_problem) {
        (1, 1) => day1::p1::solve(contents)?,
        (1, 2) => day1::p2::solve(contents)?,
        (2, 1) => day2::p1::solve(contents)?,
        (2, 2) => day2::p2::solve(contents)?,
        (3, 1) => day3::p1::solve(contents)?,
        (3, 2) => day3::p2::solve(contents)?,
        (4, 1) => day4::p1::solve(contents)?,
        (4, 2) => day4::p2::solve(contents)?,
        (5, 1) => day5::p1::solve(contents)?,
        (5, 2) => day5::p2::solve(contents)?,
        (6, 1) => day6::p1::solve(contents)?,
        (6, 2) => day6::p2::solve(contents)?,
        (7, 1) => day7::p1::solve(contents)?,
        (7, 2) => day7::p2::solve(contents)?,
        _ => Err("unimplemented problem")?,
    };
    println!("{}", result);
    Ok(())
}
