use anyhow::{anyhow, Error};

use chumsky::Parser;
use std::collections::btree_map::{Iter};

use std::collections::{BTreeMap};
use std::fmt::{Display, Formatter};

use std::ops::Bound;

pub fn part1(input: Vec<String>) -> Result<String, Error> {
    let expressions: Vec<Expr> = parse_input(input).unwrap();
    let files = create_file_tree(expressions);
    println!("{}", &files);
    let sum: u64 = files.part1();
    Ok(format!("Sum of dirs is {}", sum))
}
pub fn part2(input: Vec<String>) -> Result<String, Error> {
    let expressions: Vec<Expr> = parse_input(input).unwrap();
    let files = create_file_tree(expressions);
    println!("{}", &files);
    let sum: u64 = files.part2();
    Ok(format!("Min sufficient size is: {}", sum))
}


use chumsky::prelude::*;


#[derive(Clone, Eq, PartialEq, Debug)]
enum Expr {
    Cd(String),
    Ls,
    Dir(String),
    File(u64, String),
}

#[derive(Debug, Clone, Default)]
pub struct Files(BTreeMap<String, u64>);
impl Files {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
    pub fn add_file(&mut self, name: String, size: u64) {
        self.0.insert(name, size);
    }
    pub fn size(&self) -> u64 {
        self.0.values().sum()
    }
    pub fn iter(&self) -> Iter<'_, String, u64> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, Default)]
struct FileTree {
    directories: BTreeMap<String, Files>,
}

impl Display for FileTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for (dir, files) in &self.directories {
            out.extend(dir.chars());
            out.extend("\n".chars());
            for (name, size) in files.iter() {
                out.extend(format!("\t\t{} - {}\n", name, size).chars());
            }
        }
        write!(f, "{}", out)
    }
}

impl FileTree {
    pub fn size_of_dir(&self, name: &str) -> Option<u64> {
        Some(
            self.ls(name)
                .into_iter()
                .map(|(_name, files)| files.size())
                .sum(),
        )
    }

    pub fn directories(&self) -> impl Iterator<Item = &String> {
        self.directories.keys()
    }

    pub fn ls<A: Display>(&self, path: A) -> Vec<(&String, &Files)> {
        let path = format!("{}", path);
        let (from, to): (Bound<String>, Bound<String>) =
            (Bound::Included(path.to_string()), Bound::Unbounded);
        self.directories
            .range((from, to))
            .take_while(|(dir, _)| dir.starts_with(&path))
            .collect()
    }

    pub fn add_directory(&mut self, path: &Vec<String>, name: String) {
        let path = if path.is_empty() {
            "/".to_string()
        } else {
            format!("/{}/{}", path.join("/"), name)
        };
        if !self.directories.contains_key(&path) {
            self.directories.insert(path, Files::new());
        }
    }
    pub fn add_file(&mut self, path: &Vec<String>, name: String, size: u64) {
        let path = if path.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", path.join("/"))
        };
        let files = self.directories.entry(path).or_insert(Files::new());
        files.add_file(name, size);
    }

    pub fn find_files_smaller_then(&self, _max_size: u64) -> impl Iterator<Item = u64> + '_ {
        self.ls("/").into_iter().filter_map(|(name, _)| {
            let size = self.size_of_dir(&name).unwrap();
            if size <= 100000 {
                Some(size)
            } else {
                None
            }
        })
    }
    pub fn part1(&self) -> u64 {
        self.find_files_smaller_then(100000).sum()
    }
    pub fn part2(&self) -> u64 {
        let space_used = self.size_of_dir("/").unwrap();
        let space_available = 70000000 - space_used;
        let space_needed = 30000000 - space_available;
        self.ls("/")
            .into_iter()
            .map(|(a, _b)| self.size_of_dir(a).unwrap())
            .filter(|num| *num >= space_needed)
            .min()
            .unwrap()
    }
}

fn create_file_tree(expressions: Vec<Expr>) -> FileTree {
    let mut file_tree = FileTree {
        ..Default::default()
    };
    let mut current_path = vec![];
    for expression in expressions {
        match expression {
            Expr::Cd(dir) if dir == "/" => {
                current_path.clear();
            }
            Expr::Cd(dir) if dir == ".." => {
                if current_path.len() < 1 {
                    panic!("Invalid input")
                }
                current_path.pop();
            }
            Expr::Cd(dir) => {
                current_path.push(dir);
            }
            Expr::Ls => {
                // nothing to do ;)
            }
            Expr::Dir(name) => {
                file_tree.add_directory(&current_path, name);
            }
            Expr::File(size, name) => {
                file_tree.add_file(&current_path, name, size);
            }
        }
    }
    file_tree
}

fn parse_input(input: Vec<String>) -> Result<Vec<Expr>, Error> {
    let mut res = vec![];
    let parser = parser();
    for (idx, line) in input.iter().enumerate() {
        res.push(
            parser
                .parse(line.trim())
                .map_err(|_err| anyhow!("Failed to parse line {}:{}", idx, &line))?,
        );
    }
    Ok(res)
}

// TODO _ I dont get how to repeat this with newlines!?
fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    let command = just('$').padded();

    let file_name = filter(|c: &char| !c.is_ascii_whitespace())
        .repeated()
        .at_least(1)
        .then_ignore(end()) // why?
        .labelled("file-name")
        .collect::<String>();

    let int = text::int(10)
        .map(|x: String| x.parse::<u64>().unwrap())
        .labelled("integer");

    let cd = command
        .clone()
        .then(text::keyword("cd"))
        .padded()
        .ignore_then(file_name.clone())
        .labelled("cd")
        .map(|file_name| Expr::Cd(file_name));

    let ls = command
        .clone()
        .then(text::keyword("ls"))
        .labelled("ls")
        .map(|_| Expr::Ls);

    let dir = text::keyword("dir")
        .padded()
        .ignore_then(file_name.clone())
        .labelled("dir")
        .map(|file| Expr::Dir(file));

    let file = int
        .padded()
        .then(file_name.clone())
        .map(|(size, name)| Expr::File(size, name));
    choice((cd, ls, dir, file))
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
