

use chumsky::Parser;
use std::collections::btree_map::Iter;

use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

use crate::day07::Expr;
use std::ops::Bound;

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
pub struct FileTree {
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

pub fn create_file_tree(expressions: Vec<Expr>) -> FileTree {
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
