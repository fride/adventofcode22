use anyhow::Error;

pub fn part1(lines: Vec<String>) -> Result<String, Error> {
    let calories = find_the_food_elf(lines.into_iter());
    let (numbers, _) = calories.split_at(3);
    Ok(format!(
        "Elves: {:?}",
        numbers
            .iter()
            .map(|elf| format!("{} has {}", elf.0, elf.1))
            .collect::<Vec<String>>()
            .join(",")
    ))
}

pub fn part2(lines: Vec<String>) -> Result<String, Error> {
    let calories = find_the_food_elf(lines.into_iter());
    let (numbers, _) = calories.split_at(3);
    Ok(format!(
        "Available calories: {}",
        numbers.iter().fold(0, |acc, c| acc + c.1)
    ))
}

fn find_the_food_elf<A: Iterator<Item = String>>(calories: A) -> Vec<(usize, u32)> {
    let mut sums = calories.fold(vec![(1, 0)], |mut acc, line| {
        let last = acc.len() - 1;
        if line.is_empty() {
            acc.push((last + 2, 0)); // we count starting with one, not zero. ;)
        } else {
            let calories = line
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("Failed to parse `{}` as u32", &line));
            if let Some(last) = acc.last_mut() {
                last.1 += calories;
            }
        }
        acc
    });
    sums.sort_by(|a, b| b.1.cmp(&a.1));
    sums
}

#[cfg(test)]
mod tests {

    #[test]
    fn the_food_elves_are_found() {
        let lines_of_sweets = [
            "1", "1", "1", "", "12", "2323", "223", "", "433", "986", "", "1", "4",
        ];
        let calories = super::find_the_food_elf(lines_of_sweets.into_iter().map(|c| c.to_string()));
        assert_eq!(calories, vec![(2, 2558), (3, 1419), (4, 5), (1, 3),]);
    }
}
