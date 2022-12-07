use anyhow::{Error};









mod parser;
mod with_map;
mod with_trie;

pub trait Day07 {
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
}

pub fn part1(input: Vec<String>) -> Result<String, Error> {
    let expressions: Vec<Expr> = parse_input(input).unwrap();

    let files: with_trie::FileTree = expressions.into();
    //
    // let files = create_file_tree(expressions);
    // println!("{}", &files);
    // let sum: u64 = files.part1();
    Ok(files.part_one())
}

pub fn part2(input: Vec<String>) -> Result<String, Error> {
    let expressions: Vec<Expr> = parse_input(input).unwrap();
    // let files = create_file_tree(expressions);
    // let sum: u64 = files.part2();
    let files: with_trie::FileTree = expressions.into();
    Ok(files.part_two())
}

use crate::day07::parser::parse_input;



#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Expr {
    Cd(String),
    Ls,
    Dir(String),
    File(u64, String),
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = r###"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"###;

    #[test]
    pub fn parser_test() {
        let res = super::parser().parse("$ cd /").unwrap();
        assert_eq!(Expr::Cd("/".to_string()), res);
    }
    #[test]
    pub fn schmu() {
        let res: Vec<Expr> =
            parse_input(TEST_INPUT.split("\n").map(|c| c.to_string()).collect()).unwrap();
        let _sizes = create_file_tree(res);
    }
    #[test]
    pub fn parse_example_test() {
        let res: Vec<Expr> =
            parse_input(TEST_INPUT.split("\n").map(|c| c.to_string()).collect()).unwrap();
        let file_tree = create_file_tree(res);
        println!("{}", file_tree);
        let sizes = file_tree.part1();
        println!("{}", sizes);
        assert_eq!(95437, sizes);
    }
}
