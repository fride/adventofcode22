use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use anyhow::{anyhow, Error};
use iter_tools::Itertools;

pub fn part1(input: Vec<String>) -> Result<String, Error> {
    let (mut crates, moves) = parse_input(input);
    crates.transform(moves, true);
    Ok(crates.get_top_crates())
}
pub fn part2(input: Vec<String>) -> Result<String, Error> {
    let (mut crates, moves) = parse_input(input);
    crates.transform(moves, false);
    Ok(crates.get_top_crates())
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Move {
    pub count: usize,
    pub from: usize,
    pub to: usize,
}

pub struct Crates(HashMap<usize, Vec<char>>);

impl Display for Crates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = self
            .0
            .iter()
            .map(|(row, creates)| {
                format!(
                    "{} : {}",
                    row,
                    creates
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", str)
    }
}
impl Crates {
    pub fn move_crates(&mut self, crates_move: Move, keep_order: bool) {
        println!("{} : {}", crates_move.from, crates_move.to);
        let [source, taget] = self
            .0
            .get_many_mut([&crates_move.from, &crates_move.to])
            .expect("Invalid move!");
        let removed = source.drain(0..crates_move.count);
        if keep_order {
            removed.for_each(|c| taget.insert(0, c));
        } else {
            removed.rev().for_each(|c| taget.insert(0, c));
        }
    }
    pub fn transform(&mut self, moves: Vec<Move>, keep_order: bool) {
        for muve in moves {
            self.move_crates(muve, keep_order);
        }
    }

    pub fn get_top_crates(&self) -> String {
        let mut cols: Vec<usize> = self.0.keys().cloned().collect();
        cols.sort();
        let mut res = String::new();
        for colum in cols {
            res.push(self.0[&colum].first().unwrap().clone())
        }
        res
    }
}

pub fn parse_move(line: &str) -> Result<Move, Error> {
    let parts = line.split(" ").into_iter().collect::<Vec<&str>>();
    match parts[..] {
        [_, count, _, from, _, to] => Ok(Move {
            count: count.parse()?,
            from: from.parse()?,
            to: to.parse()?,
        }),
        _ => Err(anyhow!("Invalid move: {}", line)),
    }
}

pub fn parse_stack(line: &str) -> Vec<(usize, char)> {
    let line: Vec<(usize, char)> = line
        .chars()
        .into_iter()
        .enumerate()
        .filter_map(|(idx, char)| {
            if char.is_alphabetic() {
                Some(((idx / 4) + 1, char))
            } else {
                None
            }
        })
        .collect();
    line
}

pub fn parse_stacks<'a, A: Iterator<Item = &'a String>>(input: A) -> HashMap<usize, Vec<char>> {
    input.fold(HashMap::new(), |mut crates, line| {
        for (idx, name) in parse_stack(&line) {
            let stack = crates.entry(idx).or_insert(Vec::new());
            stack.push(name);
        }
        crates
    })
}

pub fn parse_moves<'a, A: Iterator<Item = &'a String>>(input: A) -> Vec<Move> {
    input
        .map(|line| parse_move(&line).unwrap_or_else(|e| panic!("Failed to parse move {}", e)))
        .collect()
}

fn parse_input(input: Vec<String>) -> (Crates, Vec<Move>) {
    let mut lines = input.split(|line| line.is_empty());
    let stacks = parse_stacks(lines.next().unwrap().iter());
    let moves = parse_moves(lines.next().unwrap().iter());
    (Crates(stacks), moves)
}

#[cfg(test)]
mod tests {
    use super::*;
    use iter_tools::Itertools;

    const TEST_INPUT: &'static str = r###"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"###;

    #[test]
    pub fn test_parser() {
        let results = TEST_INPUT
            .split("\n")
            .into_iter()
            .map(parse_stack)
            .filter(|l| !l.is_empty())
            .collect::<Vec<Vec<(usize, char)>>>();

        assert_eq!(results[0], vec![(2, 'D')]);
        assert_eq!(results[1], vec![(1, 'N'), (2, 'C')]);
        assert_eq!(results[2], vec![(1, 'Z'), (2, 'M'), (3, 'P')]);
    }

    #[test]
    pub fn parse_moves_test() {
        let Move { count, from, to } = parse_move("move 1 from 2 to 1").unwrap();
        assert_eq!(1, count);
        assert_eq!(2, from);
        assert_eq!(1, to);
    }

    #[test]
    pub fn test_example() {
        let (mut crates, moves) = super::parse_input(
            TEST_INPUT
                .split("\n")
                .into_iter()
                .map(|l| l.to_string())
                .collect(),
        );
        println!("{}", crates);
        println!("{}", crates.get_top_crates());
        crates.transform(moves, true);
        println!("{}", crates);
        println!("{}", crates.get_top_crates());
    }
}
