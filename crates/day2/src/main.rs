use crate::part_one_with_types::{Game};
use anyhow::{anyhow, Error};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;


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
        .unwrap_or_else(|| "./inputs/day2/day2.txt".to_string());
    let lines: Vec<String> = get_input(path.clone())?
        .filter_map(|res| match res {
            Ok(line) => Some(line),
            Err(err) => panic!("Failed to read input file: {}", err),
        })
        .collect();

    // part one
    let game = Game::parse(
        lines.clone().into_iter(),
        part_one_with_types::parse_part_one,
    )?;
    println!("Sum for this game is {}", game.score());

    // part one with the minimalistic code
    let score = part1::calculate_scores(lines.clone().into_iter())?;
    println!("Sum for this game is {}", score);

    // part tow
    let game = Game::parse(
        lines.clone().into_iter(),
        part_one_with_types::parse_part_two,
    )?;
    println!(
        "Sum for this game when using the awesome strategy is {}",
        game.score()
    );

    Ok(())
}
