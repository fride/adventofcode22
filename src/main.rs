use std::{
    io::{self, BufRead},
};
use std::str::FromStr;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().into_iter().collect();
    let path = args.get(1)
        .cloned()
        .unwrap_or("./inputs/day1/calories.txt".to_string());

    let file = std::fs::File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    let (_, _, number, calories) = lines
        .filter_map(|line| line.ok())
        .fold((1, 0, 0, 0), |mut acc: (u32, u32, u32, u32), line| {
            if line.is_empty() {
                acc.0 = acc.0 + 1;
                acc.1 = 0;
            } else {
                acc.1 = acc.1 + line.parse::<u32>().expect(&format!("Failed to parse `{}` as u32", &line));
                if acc.3 < acc.1 {
                    acc.2 = acc.0;
                    acc.3 = acc.1;
                }
            }
            acc
        });
    print!("Elf {} is carrying {} calories", number, calories);
    Ok(())
}
