use std::collections::BTreeMap;
use anyhow::{anyhow, Error};

fn shape_points(shape: char) -> Result<u32, Error> {
    match shape {
        'X' => Ok(1),
        'Y' => Ok(2),
        'Z' => Ok(3),
        _ => Err(anyhow!("'{}' is not a valid score", shape))
    }
}

fn win_points(opponent: char, player: char) -> Result<u32, Error> {
    match (opponent, player) {
        ('A', 'X') => Ok(3),
        ('A', 'Y') => Ok(6),
        ('A', 'Z') => Ok(0),
        ('B', 'X') => Ok(0),
        ('B', 'Y') => Ok(3),
        ('B', 'Z') => Ok(6),
        ('C', 'X') => Ok(6),
        ('C', 'Y') => Ok(0),
        ('C', 'Z') => Ok(3),
        _ => Err(anyhow!("Invalid round. Opponent {}, player: {}", opponent, player))
    }
}

fn score_round(round: &str) -> Result<u32, Error> {
    let mut chars = round.chars();
    let opponent = chars.nth(0).ok_or(anyhow!("Invalid round, opponent not found in '{}'", &round))?;
    // nth changes the char iter - so we need to skip only the whitespace
    let player = chars.nth(1).ok_or(anyhow!("Invalid round, player not found in '{}'", &round))?;
    let score = shape_points(player)? + win_points(opponent, player)?;
    Ok(score)
}

pub fn calculate_scores<A : Iterator<Item=String>>(lines: A) -> Result<u32, Error> {
    let mut score = 0;
    for (count, line) in lines.into_iter()
        .enumerate() {
        score += score_round(&line)?;
    }
    Ok(score)
}
#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<&'static str> {
        vec![
            "A Y",
            "B X",
            "C Z"
        ]
    }

    #[test]
    pub fn scoring_works() {
        let scores = calculate_scores(test_data().into_iter().map(|s| s.to_string())).unwrap();
        assert_eq!(15, scores)
    }
}
