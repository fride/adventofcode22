use crate::part_one_with_types::{Game, Round};
use anyhow::{anyhow, Context, Error};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::str::{FromStr, Split};

pub mod part1;
pub mod part_one_with_types;

fn get_input<P: AsRef<Path>>(path: P) -> Result<Lines<BufReader<File>>, Error> {
    let file = std::fs::File::open(&path).map_err(|e| anyhow!("Failed to open file {}", e))?;
    let lines = io::BufReader::new(file).lines();
    Ok(lines)
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().into_iter().collect();
    let path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "./inputs/day2/secret_strategy.txt".to_string());
    let lines = get_input(path.clone())?;
    let game: Game = lines
        .into_iter()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.parse()
                .unwrap_or_else(|err| panic!("Failed to parse round: {}", err))
        })
        .collect::<Vec<Round>>()
        .into();
    println!("Sum for this game is {}", game.score());

    let lines = get_input(path.clone())?;
    let score = part1::calculate_scores(lines.into_iter().filter_map(|l| l.ok()))?;

    println!("Sum for this game is {}", score);
    Ok(())
}
