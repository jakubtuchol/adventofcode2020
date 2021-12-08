extern crate clap;
extern crate regex;

use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::process::exit;

const APP_NAME: &str = "Advent of Code 2020";
const VERSION: &str = "0.1";

mod day_four;
mod day_one;
mod day_three;
mod day_two;

fn main() {
    let available_days: Vec<fn()> = vec![run_day_one, run_day_two, run_day_three, run_day_four];

    let matches = App::new(APP_NAME)
        .version(VERSION)
        .arg(
            Arg::new("days")
                .short('d')
                .long("day")
                .value_name("DAYS")
                .about("Enter a comma-separated list of days")
                .takes_value(true)
                .required(true)
                .multiple_values(true),
        )
        .get_matches();

    for day in matches.values_of_t::<usize>("days").unwrap().iter() {
        let idx = day - 1;
        if idx > available_days.len() {
            println!("day {} not available", day);
            exit(1);
        }
        available_days[idx]();
    }
}

fn run_day_one() {
    let day_one_file = match File::open("data/day_one.txt") {
        Ok(f) => f,
        Err(e) => panic!("failed to read day one file: {}", e),
    };
    let reader = BufReader::new(day_one_file);
    let v: Vec<i32> = reader
        .lines()
        .into_iter()
        .map(|l| l.ok().and_then(|s| s.parse().ok()).unwrap_or(0))
        .collect();
    println!(
        "Day one part one answer is {}",
        day_one::two_sum(v.clone(), 2020).unwrap(),
    );
    println!(
        "Day one part two answer is {}",
        day_one::three_sum(v, 2020).unwrap()
    );
}

fn run_day_two() {
    let day_two_file = match File::open("data/day_two.txt") {
        Ok(f) => f,
        Err(e) => panic!("failed to read day two file: {}", e),
    };
    let reader = BufReader::new(day_two_file);
    let policies: Vec<day_two::PasswordPolicy> = reader
        .lines()
        .map(|l| convert_line_to_policy(l.ok().unwrap()))
        .collect();
    println!(
        "Day two part one answer is {}",
        day_two::count_valid_passwords(policies.clone()),
    );
    println!(
        "Day two part two answer is {}",
        day_two::count_valid_position_passwords(policies),
    );
}

fn convert_line_to_policy(line: String) -> day_two::PasswordPolicy {
    let re = Regex::new(r"(?P<min>\d+)\-(?P<max>\d+) (?P<char>\S): (?P<password>\S+)").unwrap();
    let caps = re.captures(line.trim()).unwrap();

    let min: usize = caps["min"].parse().unwrap();
    let max: usize = caps["max"].parse().unwrap();
    let c: char = caps["char"].chars().next().unwrap();
    let password: String = caps["password"].to_owned();

    day_two::PasswordPolicy::new(c, min, max, password)
}

fn run_day_three() {
    let day_three_file = match File::open("data/day_three.txt") {
        Ok(f) => f,
        Err(e) => panic!("failed to read day three file: {}", e),
    };

    let reader = BufReader::new(day_three_file);
    let map = reader
        .lines()
        .map(|l| l.ok().unwrap().chars().collect())
        .collect::<Vec<Vec<char>>>();

    println!(
        "Day three part one answer is {}",
        day_three::get_trees_encountered((3, 1), &map),
    );
    println!(
        "Day three part one answer is {}",
        day_three::get_all_slopes(vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)], map),
    );
}

fn run_day_four() {
    let mut day_three_file = match File::open("data/day_four.txt") {
        Ok(f) => f,
        Err(e) => panic!("failed to read day four file: {}", e),
    };

    let mut contents: String = String::new();
    match day_three_file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => panic!("failed to read day four file: {}", e),
    };

    println!(
        "Day four part one answer is {}",
        day_four::check_valid_passports(&contents[..]),
    );
}
