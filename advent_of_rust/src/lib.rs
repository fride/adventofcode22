extern crate core;

use std::io::{self, BufRead};

use std::path::Path;

use anyhow::anyhow;
use anyhow::Error;

pub fn get_file_path() -> String {
    let args: Vec<String> = std::env::args().into_iter().collect();
    args.get(1)
        .cloned()
        .unwrap_or_else(|| panic!("Give me an input file please"))
}

pub fn get_input<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Error> {
    let file = std::fs::File::open(&path).map_err(|e| anyhow!("Failed to open file {}", e))?;
    let lines = io::BufReader::new(file).lines();
    Ok(lines.filter_map(|line| line.ok()).collect::<Vec<String>>())
}

pub fn run_solution<A, B>(file_path: &'static str, part_one: A, part_tow: B) -> Result<(), Error>
where
    A: Fn(Vec<String>) -> Result<String, Error>,
    B: Fn(Vec<String>) -> Result<String, Error>,
{
    println!("{}", part_one(get_input(file_path)?)?);
    println!("{}", part_tow(get_input(file_path)?)?);
    Ok(())
}
