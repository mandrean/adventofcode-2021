use adventofcode::dive::*;
use adventofcode::submarine::Submarine;
use anyhow::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

const INPUT: &'static str = "./tests/fixtures/day2_input.txt";

#[test]
fn day2() {
    day2_part1();
    day2_part2();
}

#[test]
fn day2_part1() {
    let input = read_input(INPUT).expect("Could not read input file");
    let mut sub = Submarine::new();
    sub.dives(input);
    println!(
        "\n--- Product of final horizontal position & depth after dives: {}\n",
        sub.position.horizontal * sub.position.depth
    );
}

#[test]
fn day2_part2() {
    let input = read_input(INPUT).expect("Could not read input file");
    let mut sub = Submarine::new();
    sub.aim_dives(input);
    println!(
        "\n--- Product of final horizontal position & depth after aim dives: {}\n",
        sub.position.horizontal * sub.position.depth
    );
}

pub fn read_input<P: AsRef<Path>>(path: P) -> Result<Vec<Command>> {
    let cmds = io::BufReader::new(File::open(path)?)
        .lines()
        .map(|l| cmd(l.expect("Could not read line")).expect("Could not parse command"))
        .collect::<Vec<Command>>();
    Ok(cmds)
}
