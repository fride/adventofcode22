use anyhow::Error;
use iter_tools::Itertools;

pub fn part1(input: Vec<String>) -> Result<String, Error> {
    Ok(index_of_marker(&input.join(""), 4)
        .map(|idx| format!("Package starts at {}", idx))
        .unwrap_or("No package found".to_string()))
}
pub fn part2(input: Vec<String>) -> Result<String, Error> {
    Ok(index_of_marker(&input.join(""), 14)
        .map(|idx| format!("Message starts at {}", idx))
        .unwrap_or("No message found".to_string()))
}

fn index_of_marker(text: &str, length: usize) -> Option<usize> {
    for idx in 0..text.len() {
        if idx + length > text.len() {
            return None;
        } else if text[idx..(idx+length)].chars().sorted().dedup().count() == length {
            return Some(idx+length);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::day06::index_of_marker;

    #[test]
    pub fn test_examples_4() {
        assert_eq!(index_of_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some((5)));
        assert_eq!(index_of_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), Some((6)));
    }
    #[test]
    pub fn test_examples_14() {
        assert_eq!(index_of_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), Some((26)));
        assert_eq!(index_of_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), Some((23)));
    }
}
