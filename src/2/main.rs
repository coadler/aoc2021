#[macro_use]
extern crate lazy_static;

use std::str::FromStr;

lazy_static! {
    pub static ref INPUT: Vec<(Command, i32)> = include_str!("day_two")
        .lines()
        .map(|str| str.split_ascii_whitespace().collect::<Vec<&str>>())
        .map(|v| (
            Command::from_str(v[0]).unwrap(),
            v[1].parse::<i32>().unwrap()
        ))
        .collect();
}

pub enum Command {
    Forward,
    Up,
    Down,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Command::Forward),
            "up" => Ok(Command::Up),
            "down" => Ok(Command::Down),
            _ => Err(format!("unknown command '{}'", s)),
        }
    }
}

fn main() {
    let pos_part1 = INPUT.iter().fold((0, 0), |pos, (cmd, dist)| match cmd {
        Command::Forward => (pos.0 + dist, pos.1),
        Command::Up => (pos.0, pos.1 - dist),
        Command::Down => (pos.0, pos.1 + dist),
    });

    println!("part 1 output is {}", pos_part1.0 * pos_part1.1);

    let pos_part2 = INPUT.iter().fold((0, 0, 0), |pos, (cmd, dist)| match cmd {
        Command::Forward => (pos.0, pos.1 + dist, pos.2 + (pos.0 * dist)),
        Command::Up => (pos.0 - dist, pos.1, pos.2),
        Command::Down => (pos.0 + dist, pos.1, pos.2),
    });

    println!("part 2 output is {}", pos_part2.1 * pos_part2.2);
}
