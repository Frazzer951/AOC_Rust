use std::collections::{HashMap, HashSet};

fn part_1(input: Vec<String>) -> i32 {
    let alphabet: HashMap<char, i32> = (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .enumerate()
        .map(|(index, ch)| (ch as char, index as i32 + 1))
        .collect();

    let mut sum = 0;
    for line in input {
        let len = line.len();
        let first: HashSet<char> = line[..(len / 2)].chars().collect();
        let second: HashSet<char> = line[(len / 2)..].chars().collect();

        let intersection: Vec<&char> = first.intersection(&second).collect();

        for ch in intersection {
            let value = alphabet[ch];
            sum += value;
        }
    }

    sum
}

fn part_2(input: Vec<String>) -> i32 {
    let alphabet: HashMap<char, i32> = (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .enumerate()
        .map(|(index, ch)| (ch as char, index as i32 + 1))
        .collect();
    let groups = input.chunks_exact(3).map(|x| x.to_vec()).collect::<Vec<_>>();
    let mut sum = 0;

    for group in groups {
        let elf_1: HashSet<char> = group[0].chars().collect();
        let elf_2: HashSet<char> = group[1].chars().collect();
        let elf_3: HashSet<char> = group[2].chars().collect();

        let intersection = elf_1
            .iter()
            .filter(|k| elf_2.contains(k))
            .filter(|k| elf_3.contains(k))
            .collect::<Vec<_>>();

        for ch in intersection {
            let value = alphabet[ch];
            sum += value;
        }
    }

    sum
}

pub fn run() {
    let input = crate::utils::read_input(2022, 3);

    println!(" Day 03:");

    let p1 = part_1(input.clone());
    println!("    Part 1 - {p1}");

    let p2 = part_2(input);
    println!("    Part 2 - {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_e1() {
        let input = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]
        .iter()
        .map(|s| String::from(*s))
        .collect::<Vec<_>>();
        assert_eq!(part_1(input), 157);
    }

    #[test]
    fn test_p2_e1() {
        let input = [
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ]
        .iter()
        .map(|s| String::from(*s))
        .collect::<Vec<_>>();
        assert_eq!(part_2(input), 70);
    }
}
