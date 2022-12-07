use crate::day07::{Day07, Expr};

use radix_trie::{Trie, TrieCommon};

// we are using a trie for fast look up.
pub struct FileTree(Trie<String, u64>);

impl FileTree {
    // adds the size of all files and directories below the given dir
    pub fn dir_size<A: AsRef<str>>(&self, dir: A) -> u64 {
        let dir = dir.as_ref().to_string();
        match self.0.subtrie(&dir) {
            None => 0,
            Some(sub) => sub.iter().fold(0, |acc, c| acc + *c.1),
        }
    }
    // get all directories!
    pub fn dirs(&self) -> impl Iterator<Item = &String> + '_ {
        self.0
            .iter()
            .filter_map(|(k, v)| if *v == 0 { Some(k) } else { None })
    }

    pub fn dirs_sized(&self) -> impl Iterator<Item = (&String, u64)> + '_ {
        self.dirs().map(|dir| (dir, self.dir_size(dir)))
    }

    pub fn debugs(&self) {
        for (a, b) in self.0.iter() {
            println!("{} == {}", a, b);
        }
    }
}

impl Day07 for FileTree {
    fn part_one(&self) -> String {
        let sum: u64 = self
            .dirs_sized()
            .filter_map(
                |(_name, size)| {
                    if size <= 100000 {
                        Some(size)
                    } else {
                        None
                    }
                },
            )
            .sum();
        format!("Sum of dirs is {}", sum)
    }

    fn part_two(&self) -> String {
        let space_used = self.dir_size("/");
        let space_available = 70000000 - space_used;
        let space_needed = 30000000 - space_available;
        let minimum_size = self
            .dirs_sized()
            .filter_map(|(_name, num)| {
                if num >= space_needed {
                    Some(num.clone())
                } else {
                    None
                }
            })
            .min()
            .unwrap();
        format!("Min sufficient size is: {}", minimum_size)
    }
}

impl From<Vec<Expr>> for FileTree {
    fn from(s: Vec<Expr>) -> Self {
        from_expressions(s)
    }
}

fn from_expressions(exprs: Vec<Expr>) -> FileTree {
    let mut trie: Trie<String, u64> = radix_trie::Trie::new();
    let pwd: Vec<String> = vec![];
    fn pwd_to_str(pwd: &Vec<String>) -> String {
        if pwd.is_empty() {
            "".to_string()
        } else {
            format!("/{}", pwd.join("/"))
        }
    }
    trie.insert("/".to_string(), 0);
    FileTree(
        exprs
            .into_iter()
            .fold((trie, pwd), |(mut trie, mut pwd), expr| {
                match expr {
                    Expr::Cd(dir) if dir == ".." => (trie, {
                        pwd.pop();
                        pwd
                    }),
                    Expr::Cd(dir) if dir == "/" => (trie, {
                        pwd.clear();
                        pwd
                    }),
                    Expr::Cd(dir) => (trie, {
                        pwd.push(dir);
                        pwd
                    }),
                    Expr::Ls => (trie, pwd),
                    Expr::Dir(directory) => {
                        let key = format!("{}/{}", pwd_to_str(&pwd), directory); // this is silly!?
                        trie.insert(key, 0); // needed for subDirs!?
                        (trie, pwd) // really !? or zero!?
                    }
                    Expr::File(size, name) => {
                        let key = format!("{}/{}", pwd_to_str(&pwd), name);
                        trie.insert(key, size);
                        (trie, pwd)
                    }
                }
            })
            .0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day07::parse_input;

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
    pub fn tries_try() {
        let res: Vec<Expr> =
            parse_input(TEST_INPUT.split("\n").map(|c| c.to_string()).collect()).unwrap();
        let file_tree: FileTree = res.into();

        let size = file_tree.dir_size("/");
        file_tree.debugs();
        file_tree
            .dirs()
            .map(|dir| (dir.clone(), file_tree.dir_size(dir)))
            .for_each(|(dir, size)| println!("{} :: {}", dir, size));

        let part_one: u64 = file_tree
            .dirs_sized()
            .filter_map(
                |(name, size)| {
                    if size <= 100000 {
                        Some(size)
                    } else {
                        None
                    }
                },
            )
            .sum();
        println!("{}", part_one);
        assert_eq!(95437, part_one);
    }
}
