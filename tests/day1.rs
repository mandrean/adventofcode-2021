use adventofcode::sonar::*;
use anyhow::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

const INPUT: &'static str = "./tests/fixtures/day1_input.txt";

#[test]
fn day1() {
    day1_part1();
    day1_part2();
}

#[test]
fn day1_part1() {
    println!(
        "\n--- Number of increments: {}\n",
        increments(read_input(INPUT).expect("Could not read input file"))
    );
}

#[test]
fn day1_part2() {
    println!(
        "\n--- Number of sliding increments: {}\n",
        sliding_increments(read_input(INPUT).expect("Could not read input file"))
    );
}

pub fn read_input<P: AsRef<Path>>(path: P) -> Result<Vec<usize>> {
    let file = File::open(path)?;
    let depths = io::BufReader::new(file)
        .lines()
        .map(|l| {
            l.expect("Could not read line")
                .parse::<usize>()
                .expect("Could not parse number")
        })
        .collect::<Vec<usize>>();
    Ok(depths)
}
