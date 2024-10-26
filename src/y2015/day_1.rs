use crate::years::Day;

pub struct AocDay;

impl Day for AocDay {
    fn run(&self) {
        let input = crate::utils::read_input(2015, 1);

        println!(" Day 01:");

        let p1 = part_1(input.clone());
        println!("    Part 1 - {p1}");

        let p2 = part_2(input);
        println!("    Part 2 - {p2}");
    }
}

fn part_1(input: Vec<String>) -> i32 {
    0
}

fn part_2(input: Vec<String>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<String> {
        [""].iter().map(|s| String::from(*s)).collect::<Vec<_>>()
    }

    #[test]
    fn test_p1() {
        let input = example_input();
        assert_eq!(part_1(input), 1);
    }

    #[test]
    fn test_p2() {
        let input = example_input();
        assert_eq!(part_2(input), 1);
    }
}
