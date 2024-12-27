
pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> u32 {
    let mut sum: u32 = 0;

    for line in input {
        let nums: String = line.chars().filter(|c| c.is_numeric()).collect();
        if let (Some(first), Some(last)) = (nums.chars().next(), nums.chars().last()) {
            let first_num = first.to_digit(10).unwrap() as u32;
            let last_num = last.to_digit(10).unwrap() as u32;

            sum += first_num * 10 + last_num;
        }
    }

    sum
}

pub fn part2(input: &[&str]) -> usize {
    let mut sum: usize = 0;

    for line in input {
        let mut numbers = Vec::new();
        let chars: Vec<char> = line.chars().collect();

        for (i, _) in chars.iter().enumerate() {
            // Check for digit
            if chars[i].is_numeric() {
                numbers.push(chars[i].to_digit(10).unwrap() as usize);
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
