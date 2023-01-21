use camino::Utf8PathBuf;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};
use std::collections::HashMap;

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(take_while1(|c: char| "abcdefghijklmnopqrstuvwxyz./".contains(c)), Into::into)(i)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_ls: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_command(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(nom::character::complete::u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(preceded(tag("dir "), parse_path), Entry::Dir);

    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((map(parse_command, Line::Command), map(parse_entry, Line::Entry)))(i)
}

#[derive(Debug, Default)]
struct Directory {
    size: u64,
    children: HashMap<Utf8PathBuf, Directory>,
}

impl Directory {
    fn add_file(&mut self, cur_dir: &Utf8PathBuf, size: u64) {
        let mut parent_dir = self;
        for dir in cur_dir.iter() {
            parent_dir = parent_dir.children.entry(Utf8PathBuf::from(dir)).or_default();
        }
        parent_dir.size += size;
    }

    fn calculate_size(&mut self) {
        for dir in self.children.values_mut() {
            dir.calculate_size();
            self.size += dir.size;
        }
    }

    fn sum_size_less_than(&self, limit: u64) -> u64 {
        let mut sum = 0;

        if self.size <= limit {
            sum += self.size;
        }

        for dir in self.children.values() {
            sum += dir.sum_size_less_than(limit);
        }

        sum
    }

    fn find_file_to_delete(&self, needed_space: u64, mut cur_smallest_folder: u64) -> u64 {
        if self.size < cur_smallest_folder && self.size >= needed_space {
            cur_smallest_folder = self.size;
        }

        for dir in self.children.values() {
            cur_smallest_folder = dir.find_file_to_delete(needed_space, cur_smallest_folder)
        }

        cur_smallest_folder
    }
}

fn build_tree(input: Vec<String>) -> Directory {
    let lines = input.iter().map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let mut cur_dir = Utf8PathBuf::new();
    let mut dir_tree = Directory::default();

    for line in lines {
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {},
                Command::Cd(path) => match path.as_str() {
                    "/" => {},
                    ".." => {
                        cur_dir.pop();
                    },
                    _ => {
                        cur_dir.push(path);
                    },
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {},
                Entry::File(size, _) => dir_tree.add_file(&cur_dir, size),
            },
        }
    }

    dir_tree.calculate_size();
    dir_tree
}

fn part_1(input: Vec<String>) -> u64 {
    let dir_tree = build_tree(input);

    //println!("\nCur Tree:\n{dir_tree:#?}");

    dir_tree.sum_size_less_than(100000)
}

fn part_2(input: Vec<String>) -> u64 {
    let dir_tree = build_tree(input);

    let total_size = 70000000;
    let needed_unused_space = 30000000;
    let current_size = dir_tree.size;
    let cur_free_space = total_size - current_size;
    let needed_size = needed_unused_space - cur_free_space;

    dir_tree.find_file_to_delete(needed_size, current_size)
}

pub fn run() {
    let input = crate::utils::read_input(2022, 7);

    println!(" Day 07:");

    let p1 = part_1(input.clone());
    println!("    Part 1 - {p1}");

    let p2 = part_2(input);
    println!("    Part 2 - {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<String> {
        vec![
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ]
        .iter()
        .map(|s| String::from(*s))
        .collect::<Vec<_>>()
    }

    #[test]
    fn test_p1() {
        let input = example_input();
        assert_eq!(part_1(input), 95437);
    }

    #[test]
    fn test_p2() {
        let input = example_input();
        assert_eq!(part_2(input), 24933642);
    }
}
