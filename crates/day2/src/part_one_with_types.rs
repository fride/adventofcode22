use anyhow::{anyhow, Error};


#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Strategy {
    Win,
    Draw,
    Lose,
}
impl Strategy {
    pub fn get_shape(&self, opponents_shape: Shape) -> Shape {
        match self {
            Strategy::Win => opponents_shape.looses_against(),
            Strategy::Draw => opponents_shape.clone(),
            Strategy::Lose => opponents_shape.wins_against(),
        }
    }
}

impl TryFrom<char> for Strategy {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Strategy::Lose),
            'Y' => Ok(Strategy::Draw),
            'Z' => Ok(Strategy::Win),
            _ => Err(anyhow!("Invalid strategy: {}", value)),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum RoundResult {
    TheOtherElfWon = 0,
    Draw = 3,
    MeWon = 6,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
impl Shape {
    pub fn wins_against(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }
    pub fn looses_against(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    pub fn result(&self, other: &Shape) -> RoundResult {
        match &self {
            Shape::Rock => match other {
                Shape::Rock => RoundResult::Draw,
                Shape::Paper => RoundResult::TheOtherElfWon,
                Shape::Scissors => RoundResult::MeWon,
            },
            Shape::Paper => match other {
                Shape::Rock => RoundResult::MeWon,
                Shape::Paper => RoundResult::Draw,
                Shape::Scissors => RoundResult::TheOtherElfWon,
            },
            Shape::Scissors => match other {
                Shape::Rock => RoundResult::TheOtherElfWon,
                Shape::Paper => RoundResult::MeWon,
                Shape::Scissors => RoundResult::Draw,
            },
        }
    }
}
impl TryFrom<char> for Shape {
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Shape::Rock),
            'X' => Ok(Shape::Rock),
            'B' => Ok(Shape::Paper),
            'Y' => Ok(Shape::Paper),
            'C' => Ok(Shape::Scissors),
            'Z' => Ok(Shape::Scissors),
            _ => Err(anyhow!("'{}' is not a shape", value)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Round {
    pub opponent: Shape,
    pub player: Shape,
}

impl Round {
    pub fn score(&self) -> (RoundResult, u8) {
        let result = self.player.result(&self.opponent);
        let _score_for_shape = self.player as u8;
        (result, self.player as u8 + result as u8)
    }
}

#[derive(Debug)]
pub struct Game(Vec<Round>);
impl Game {
    pub fn score(&self) -> u32 {
        self.0.iter().map(|r| r.score().1 as u32).sum()
    }
    pub fn parse<P, A>(lines: A, round_parser: P) -> Result<Self, Error>
    where
        A: Iterator<Item = String>,
        P: Fn(String) -> Result<Round, Error>,
    {
        let mut rounds = vec![];
        for line in lines {
            // using for as we can use the nice ? macro then!
            rounds.push(round_parser(line)?);
        }
        Ok(Game(rounds))
    }
}

impl From<Vec<Round>> for Game {
    fn from(rounds: Vec<Round>) -> Self {
        Game(rounds)
    }
}
pub fn parse_part_one(line: String) -> Result<Round, Error> {
    let opponent: Shape = line
        .chars()
        .nth(0)
        .ok_or(anyhow!("First player not found"))
        .and_then(|c| c.try_into())?;
    let player: Shape = line
        .chars()
        .nth(2)
        .ok_or(anyhow!("Second player not found"))
        .and_then(|c| c.try_into())?;
    Ok(Round { opponent, player })
}

pub fn parse_part_two(line: String) -> Result<Round, Error> {
    let opponent: Shape = line
        .chars()
        .nth(0)
        .ok_or(anyhow!("First player not found"))
        .and_then(|c| c.try_into())?;
    let strategy: Strategy = line
        .chars()
        .nth(2)
        .ok_or(anyhow!("Second player not found"))
        .and_then(|c| c.try_into())?;
    Ok(Round {
        opponent,
        player: strategy.get_shape(opponent),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<&'static str> {
        vec!["A Y", "B X", "C Z"]
    }

    #[test]
    fn parsing_works() {
        let game = Game::parse(test_data().iter().map(|s| s.to_string()), parse_part_one).unwrap();
        assert_eq!(
            vec![
                Round {
                    opponent: Shape::Rock,
                    player: Shape::Paper
                },
                Round {
                    opponent: Shape::Paper,
                    player: Shape::Rock
                },
                Round {
                    opponent: Shape::Scissors,
                    player: Shape::Scissors
                },
            ],
            game.0
        );
    }

    #[test]
    fn scoring_works() {
        let game = Game::parse(test_data().iter().map(|s| s.to_string()), parse_part_one).unwrap();
        let results: Vec<(RoundResult, u8)> = game.0.iter().map(|r| r.score()).collect();
        assert_eq!(
            vec![
                (RoundResult::MeWon, 8),
                (RoundResult::TheOtherElfWon, 1),
                (RoundResult::Draw, 6),
            ],
            results
        );
        assert_eq!(15, game.score())
    }

    #[test]
    pub fn part_two_test() {
        let input = vec!["A Y", "B X", "C Z"];
        let game = Game::parse(input.iter().map(|s| s.to_string()), parse_part_two).unwrap();
        let results: Vec<(RoundResult, u8)> = game.0.iter().map(|r| r.score()).collect();
        assert_eq!(
            vec![
                (RoundResult::Draw, 4),
                (RoundResult::TheOtherElfWon, 1),
                (RoundResult::MeWon, 7),
            ],
            results
        );

        assert_eq!(12, game.score());
    }
}
