use std::io::{self, BufRead};

fn find_the_food_elf<A: Iterator<Item = String>>(calories: A) -> Vec<(usize,u32)> {
    let mut sums =
        calories.fold(vec![(1,0)], |mut acc, line| {
            let last = acc.len() -1;
            if line.is_empty() {
                acc.push((last+2, 0)); // we count starting with one, not zero. ;)
            } else {
                let calories= line
                    .parse::<u32>()
                    .unwrap_or_else(|_| panic!("Failed to parse `{}` as u32", &line));
                if let Some(last)  = acc.last_mut() {
                    last.1 += calories;
                }
            }
            acc
        });
    sums.sort_by(|a,b| b.1.cmp(&a.1));
    sums
}

//
// solve day one riddle https://adventofcode.com/2022/day/1
//
fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().into_iter().collect();
    let path = args
        .get(1)
        .cloned()
        .unwrap_or_else(|| "./crates/day1/calories.txt".to_string());
    let file = std::fs::File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    let calories = find_the_food_elf(lines.filter_map(|line| line.ok()));
    let (numbers,_) = calories.split_at(3);
    println!("Elves: {:?}", numbers.iter()
        .map(|elf| format!("{} has {}", elf.0, elf.1))
        .collect::<Vec<String>>()
        .join(","));
    println!("Available calories: {}", numbers
        .iter()
        .fold(0, |acc,c| acc + c.1)
    );
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn the_food_elves_are_found() {
        let lines_of_sweets = ["1", "1", "1", "", "12", "2323", "223", "", "433", "986", "", "1", "4"];
        let calories =
             super::find_the_food_elf(lines_of_sweets.into_iter().map(|c| c.to_string()));
        assert_eq!(calories, vec![
            (2,2558),
            (3,1419),
            (4,5),
            (1,3),
        ]);
    }
}
