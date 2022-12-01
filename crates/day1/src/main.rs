use std::io::{self, BufRead};
// finds the elf carrying the most calories (aka Sugar) around.
fn find_the_food_elf<A: Iterator<Item = String>>(calories: A) -> (u32, u32) {
    let (_, _, number, calories) =
        calories.fold((1, 0, 0, 0), |mut acc: (u32, u32, u32, u32), line| {
            if line.is_empty() {
                acc.0 = acc.0 + 1;
                acc.1 = 0;
            } else {
                acc.1 = acc.1
                    + line
                        .parse::<u32>()
                        .expect(&format!("Failed to parse `{}` as u32", &line));
                if acc.3 < acc.1 {
                    acc.2 = acc.0;
                    acc.3 = acc.1;
                }
            }
            acc
        });
    (number, calories)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().into_iter().collect();
    let path = args
        .get(1)
        .cloned()
        .unwrap_or("./crates/day1/calories.txt".to_string());
    let file = std::fs::File::open(path)?;
    let lines = io::BufReader::new(file).lines();
    let (number, calories) = find_the_food_elf(lines.filter_map(|line| line.ok()));
    print!("Elf {} is carrying {} calories", number, calories);
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn the_food_elf_is_found() {
        let lines_of_sweets = ["12", "2323", "223", "", "433", "986", "", "1", "4"];
        let (line, calories) =
            super::find_the_food_elf(lines_of_sweets.into_iter().map(|c| c.to_string()));
        assert_eq!(1, line);
        assert_eq!(2558, calories);
    }
}
