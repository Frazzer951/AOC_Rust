fn parse_range(range_str: &str) -> (i32, i32) {
    let parts: Vec<&str> = range_str.split('-').collect();
    let start: i32 = parts[0].parse().unwrap();
    let end: i32 = parts[1].parse().unwrap();
    (start, end)
}

fn part_1(input: Vec<String>) -> i32 {
    let mut sum = 0;

    for line in input {
        let ranges: Vec<&str> = line.split(',').collect();
        let range_1 = parse_range(ranges[0]);
        let range_2 = parse_range(ranges[1]);

        if range_1.0 <= range_2.0 && range_1.1 >= range_2.1 || range_2.0 <= range_1.0 && range_2.1 >= range_1.1 {
            sum += 1;
        }
    }

    sum
}

fn part_2(input: Vec<String>) -> i32 {
    let mut sum = 0;

    for line in input {
        let ranges: Vec<&str> = line.split(',').collect();
        let range_1 = parse_range(ranges[0]);
        let range_2 = parse_range(ranges[1]);

        if range_1.0 <= range_2.1 && range_1.1 >= range_2.0 {
            sum += 1;
        }
    }

    sum
}

pub fn run() {
    let input = crate::utils::read_input(2022, 4);

    let p1 = part_1(input.clone());
    println!("2022 Day 04 - P1 - {p1}");

    let p2 = part_2(input);
    println!("2022 Day 04 - P2 - {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_e1() {
        let input = vec!["2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8"]
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<_>>();
        assert_eq!(part_1(input), 2);
    }

    #[test]
    fn test_p2_e1() {
        let input = vec!["2-4,6-8", "2-3,4-5", "5-7,7-9", "2-8,3-7", "6-6,4-6", "2-6,4-8"]
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<_>>();
        assert_eq!(part_2(input), 4);
    }
}
