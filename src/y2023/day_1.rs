use crate::years::Day;

pub struct AocDay;

impl Day for AocDay {
    fn run(&self) {
        let input = crate::utils::read_input(2023, 1);

        println!(" Day 01:");

        let p1 = part_1(input.clone());
        println!("    Part 1 - {p1}");

        let p2 = part_2(input);
        println!("    Part 2 - {p2}");
    }
}

fn part_1(input: Vec<String>) -> i32 {
    let mut sum: i32 = 0;

    for line in input {
        let nums: String = line.chars().filter(|c| c.is_numeric()).collect();
        if let (Some(first), Some(last)) = (nums.chars().next(), nums.chars().last()) {
            let first_num = first.to_digit(10).unwrap() as i32;
            let last_num = last.to_digit(10).unwrap() as i32;

            sum += first_num * 10 + last_num;
        }
    }

    sum
}

fn part_2(input: Vec<String>) -> i32 {
    let mut sum: i32 = 0;

    for line in input {
        let mut numbers = Vec::new();
        let chars: Vec<char> = line.chars().collect();

        for (i, _) in chars.iter().enumerate() {
            // Check for digit
            if chars[i].is_numeric() {
                numbers.push(chars[i].to_digit(10).unwrap() as i32);
                continue;
            }

            // Check for word numbers starting at position i
            let remaining: String = chars[i..].iter().collect();
            if remaining.starts_with("one") {
                numbers.push(1);
            } else if remaining.starts_with("two") {
                numbers.push(2);
            } else if remaining.starts_with("three") {
                numbers.push(3);
            } else if remaining.starts_with("four") {
                numbers.push(4);
            } else if remaining.starts_with("five") {
                numbers.push(5);
            } else if remaining.starts_with("six") {
                numbers.push(6);
            } else if remaining.starts_with("seven") {
                numbers.push(7);
            } else if remaining.starts_with("eight") {
                numbers.push(8);
            } else if remaining.starts_with("nine") {
                numbers.push(9);
            }
        }

        if let (Some(&first), Some(&last)) = (numbers.first(), numbers.last()) {
            sum += first * 10 + last;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input_p1() -> Vec<String> {
        ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_p1() {
        let input = example_input_p1();
        assert_eq!(part_1(input), 142);
    }

    fn example_input_p2() -> Vec<String> {
        [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ]
        .iter()
        .map(|s| String::from(*s))
        .collect::<Vec<_>>()
    }

    #[test]
    fn test_p2() {
        let input = example_input_p2();
        assert_eq!(part_2(input), 281);
    }
}
