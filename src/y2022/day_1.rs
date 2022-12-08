fn part_1(input: Vec<String>) -> i32 {
    let mut largest_sum = 0;
    let mut cur_sum = 0;

    for line in input {
        if line.is_empty() {
            largest_sum = largest_sum.max(cur_sum);
            cur_sum = 0;
        } else {
            cur_sum += line.parse::<i32>().unwrap();
        }
    }

    largest_sum.max(cur_sum)
}

fn part_2(input: Vec<String>) -> i32 {
    let mut calories = vec![];
    let mut cur_sum = 0;

    for line in input {
        if line.is_empty() {
            calories.push(cur_sum);
            cur_sum = 0;
        } else {
            cur_sum += line.parse::<i32>().unwrap();
        }
    }
    calories.push(cur_sum);

    calories.sort_by(|a, b| b.cmp(a));

    let top_3 = calories.iter().take(3).sum();

    top_3
}

pub fn run() {
    let input = crate::utils::read_input(2022, 1);

    let p1 = part_1(input.clone());
    println!("2022 Day 01 - P1 - {p1}");

    let p2 = part_2(input);
    println!("2022 Day 01 - P2 - {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_e1() {
        let input = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "", "10000",
        ]
        .iter()
        .map(|s| String::from(*s))
        .collect::<Vec<_>>();

        assert_eq!(part_1(input), 24000);
    }

    #[test]
    fn test_p2_e1() {
        let input = vec![
            "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "", "10000",
        ]
        .iter()
        .map(|s| String::from(*s))
        .collect::<Vec<_>>();

        assert_eq!(part_2(input), 45000);
    }
}
