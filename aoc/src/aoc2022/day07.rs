#![allow(dead_code, unused_variables)]

use std::{collections::HashMap, path::PathBuf, str::FromStr};

use crate::{
    error::{AoCError, Result},
    Solution,
};

pub fn run(input: &str) -> Result<Solution> {
    let fs = HashMap::<PathBuf, Entity>::new();

    todo!()
}

enum Entity {
    File(File),
    Dir(PathBuf),
}

struct File {
    size: usize,
    name: PathBuf,
}

#[derive(Debug)]
enum Command {
    List,
    ChangeDirectory(ChangeDirectoryCommand),
}

impl FromStr for Command {
    type Err = AoCError;

    fn from_str(s: &str) -> Result<Command> {
        todo!()
    }
}

#[derive(Debug)]
enum ChangeDirectoryCommand {
    Root,
    In(PathBuf),
    Out,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn matches_examples() {
        let example = r"$ cd /
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
7214296 k";

        let solution = run(example).unwrap();

        assert_eq!(solution.part_one, "");
        assert_eq!(solution.part_two, "");
    }

    #[test]
    fn matches_offical_results() {
        let input = include_str!("./input/day07");

        let solution = run(input).unwrap();

        assert_eq!(solution.part_one, "");
        assert_eq!(solution.part_two, "");
    }
}
