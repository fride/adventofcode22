use anyhow::{anyhow, Error};
use std::ops::RangeInclusive;

pub fn part_one(input: Vec<String>) -> Result<String, Error> {
    let pairs_that_contain_each_other: u32 = input
        .into_iter()
        .map(|line| {
            parse_ranges(&line)
                .unwrap_or_else(|err| panic!("Failed to parse range in line '{}': {}", &line, err))
        })
        .filter_map(|(range_one, range_two)| {
            if fully_contains(range_one, range_two) {
                Some(1)
            } else {
                None
            }
        })
        .sum();
    Ok(format!(
        "Formart In Day 4,part one we found {} pairs that fully contain each other",
        pairs_that_contain_each_other
    ))
}
pub fn part_two(input: Vec<String>) -> Result<String, Error> {
    let pairs_that_contain_each_other: u32 = input
        .into_iter()
        .map(|line| {
            parse_ranges(&line)
                .unwrap_or_else(|err| panic!("Failed to parse range in line '{}': {}", &line, err))
        })
        .filter_map(|(range_one, range_two)| {
            if has_overlap(range_one, range_two) {
                Some(1)
            } else {
                None
            }
        })
        .sum();
    Ok(format!(
        "Formart In Day 4,part two we found {} pairs that overlap each other",
        pairs_that_contain_each_other
    ))
}

pub fn parse_range(text: &str) -> Result<RangeInclusive<u8>, Error> {
    let mut split = text.split("-");
    if let (Some(from), Some(to)) = (split.next(), split.next()) {
        Ok(from.parse()?..=to.parse()?)
    } else {
        Err(anyhow!("Could not parse {} as range", text))
    }
}
pub fn parse_ranges(text: &str) -> Result<(RangeInclusive<u8>, RangeInclusive<u8>), Error> {
    let mut split = text.split(",");
    if let (Some(a), Some(b)) = (split.next(), split.next()) {
        Ok((parse_range(a)?, parse_range(b)?))
    } else {
        Err(anyhow!("Invalid range input! {}", text))
    }
}

pub fn fully_contains(range_one: RangeInclusive<u8>, range_two: RangeInclusive<u8>) -> bool {
    range_one.contains(&range_two.start()) && range_one.contains(range_two.end())
        || range_two.contains(&range_one.start()) && range_two.contains(range_one.end())
}

pub fn has_overlap(range_one: RangeInclusive<u8>, range_two: RangeInclusive<u8>) -> bool {
    sub_range(range_one, range_two).is_some()
}

//
// .2345678.  2-8
// ..34567..  3-7
// should return [3,6]
//
pub fn sub_range(
    range_one: RangeInclusive<u8>,
    range_two: RangeInclusive<u8>,
) -> Option<RangeInclusive<u8>> {
    let max_start = if range_one.start() >= range_two.start() {
        range_one.start()
    } else {
        range_two.start()
    };
    let min_end = if range_one.end() <= range_two.end() {
        range_one.end()
    } else {
        range_two.end()
    };
    if max_start <= min_end {
        Some(RangeInclusive::new(max_start.clone(), min_end.clone()))
    } else {
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = r###"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
        "###;

    #[test]
    pub fn test() {
        assert_eq!(RangeInclusive::new(2, 4), parse_range("2-4").unwrap());
        assert_eq!(
            (RangeInclusive::new(2, 4), RangeInclusive::new(6, 8)),
            parse_ranges("2-4,6-8").unwrap()
        );
    }
    #[test]
    pub fn test_fully_contains() {
        let fully_contained: Vec<&str> = TEST_INPUT
            .split("\n")
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                let (range_one, range_two) = parse_ranges(line).unwrap();
                if fully_contains(range_one, range_two) {
                    Some(line)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(vec!["2-8,3-7", "6-6,4-6"], fully_contained);
    }
    #[test]
    pub fn test_sub_range() {
        let range = sub_range(RangeInclusive::new(2, 8), RangeInclusive::new(3, 7));
        assert_eq!(Some(RangeInclusive::new(3, 7)), range);
        let range = sub_range(RangeInclusive::new(6, 6), RangeInclusive::new(4, 6));
        assert_eq!(Some(RangeInclusive::new(6, 6)), range);
    }
    #[test]
    pub fn test_has_overlap() {
        let fully_contained: Vec<&str> = TEST_INPUT
            .split("\n")
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                let (range_one, range_two) = parse_ranges(line).unwrap();
                if has_overlap(range_one, range_two) {
                    Some(line)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(
            vec!["5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8"],
            fully_contained
        );
    }
}
