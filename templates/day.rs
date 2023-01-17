fn part_1(input: Vec<String>) -> i32 {
    0
}

fn part_2(input: Vec<String>) -> i32 {
    0
}

pub fn run() {
    let input = crate::utils::read_input({{ year }}, {{ day }});

    println!(" Day {{ day | zero_pad(width=2) }}:");

    let p1 = part_1(input.clone());
    println!("    Part 1 - {p1}");

    let p2 = part_2(input);
    println!("    Part 2 - {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<String> {
        vec![""].iter().map(|s| String::from(*s)).collect::<Vec<_>>()
    }

    #[test]
    fn test_p1_e1() {
        let input = example_input();
        assert_eq!(part_1(input), 0);
    }

    #[test]
    fn test_p2_e1() {
        let input = example_input();
        assert_eq!(part_2(input), 0);
    }
}
