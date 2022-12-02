use std::str::{FromStr};
use anyhow::{anyhow, Context, Error};

#[derive(Debug, Eq, PartialEq, Clone,Copy)]
pub enum RoundResult {
    TheOtherElfWon = 0,
    Draw = 3,
    MeWon = 6
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Shape {
    Rock = 1,
    Paper = 2 ,
    Scissors =3,
}

impl Shape {
    pub fn result(&self, other: &Shape) -> RoundResult {
        match &self {
            Shape::Rock => {
                match other {
                    Shape::Rock => RoundResult::Draw,
                    Shape::Paper => RoundResult::TheOtherElfWon,
                    Shape::Scissors => RoundResult::MeWon
                }
            }
            Shape::Paper => {
                match other {
                    Shape::Rock => RoundResult::MeWon,
                    Shape::Paper => RoundResult::Draw,
                    Shape::Scissors => RoundResult::TheOtherElfWon
                }
            }
            Shape::Scissors => {
                match other {
                    Shape::Rock => RoundResult::TheOtherElfWon,
                    Shape::Paper => RoundResult::MeWon,
                    Shape::Scissors => RoundResult::Draw
                }
            }
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
            _ => Err(anyhow!("'{}' is not a shape", value))
        }
    }
}


#[derive(Debug, Eq, PartialEq)]
pub struct Round{
    pub opponent: Shape,
    pub player: Shape
}

impl Round {
    pub fn score(&self) -> (RoundResult, u8) {
        let result = self.player.result(&self.opponent);
        let score_for_shape = self.player as u8;
        (result, self.player as u8 + result as u8)
    }
}
impl FromStr for Round {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let opponent : Shape = s.chars().nth(0).ok_or(anyhow!("First player not found")).and_then(|c| c.try_into())?;
        let player : Shape =  s.chars().nth(2).ok_or(anyhow!("Second player not found")).and_then(|c| c.try_into())?;
        Ok(Round{opponent, player})
    }
}

#[derive(Debug)]
pub struct Game(Vec<Round>);
impl Game {
    pub fn score(&self) -> u32 {
        self.0.iter()
            .map(|r| r.score().1 as u32)
            .sum()
    }
}
impl From<Vec<Round>> for Game {
    fn from(rounds: Vec<Round>) -> Self {
        Game(rounds)
    }
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
    fn parsing_works() {
        let rounds : Vec<Round> = test_data().into_iter().fold(vec![], |mut acc, line| {
            acc.push(line.parse::<Round>().unwrap());
            acc
        });
        assert_eq!(vec![
            Round{ opponent: Shape::Rock, player: Shape::Paper},
            Round{ opponent: Shape::Paper, player: Shape::Rock},
            Round{ opponent: Shape::Scissors, player: Shape::Scissors},
        ], rounds);
    }

    #[test]
    fn scoring_works() {
        let rounds : Vec<(RoundResult, u8)> = test_data().into_iter().fold(vec![], |mut acc, line| {
            acc.push(line.parse::<Round>().unwrap().score());
            acc
        });
        assert_eq!(vec![
            (RoundResult::MeWon, 8),
            (RoundResult::TheOtherElfWon, 1),
            (RoundResult::Draw, 6),

        ], rounds);
    }
}
