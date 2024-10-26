use crate::years::Day;

pub struct AocDay;

impl Day for AocDay {
    fn run(&self) {
        let input = crate::utils::read_input(2015, 1)[0].to_owned();

        println!(" Day 01:");

        let p1 = part_1(input.clone());
        println!("    Part 1 - {p1}");

        let p2 = part_2(input);
        println!("    Part 2 - {p2}");
    }
}

fn part_1(input: String) -> i32 {
    let mut height: i32 = 0;

    for char in input.chars() {
        match char {
            '(' => height += 1,
            ')' => height -= 1,
            _ => panic!("Invalid Char"),
        }
    }

    height
}

fn part_2(input: String) -> i32 {
    let mut height: i32 = 0;
    let mut position: i32 = 0;

    for char in input.chars() {
        position += 1;
        match char {
            '(' => height += 1,
            ')' => height -= 1,
            _ => panic!("Invalid Char"),
        }
        if height < 0 {
            return position;
        }
    }

    position
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec!["(())", "()()"], 0 ; "0")]
    #[test_case(vec!["(((", "(()(()(", "))((((("], 3 ; "3")]
    #[test_case(vec!["())", "))("], -1 ; "neg1")]
    #[test_case(vec![")))", ")())())"], -3 ; "neg3")]
    fn test_p1(inputs: Vec<&str>, expected: i32) {
        for input in inputs {
            assert_eq!(part_1(input.to_owned()), expected);
        }
    }

    #[test_case(")", 1)]
    #[test_case("()())", 5)]
    fn test_p2(input: &str, expected: i32) {
        assert_eq!(part_2(input.to_owned()), expected);
    }
}
