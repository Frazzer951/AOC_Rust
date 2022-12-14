fn part_1(input: Vec<String>) -> i32 {
    0
}

fn part_2(input: Vec<String>) -> i32 {
    0
}

pub fn run() {
    let input = crate::utils::read_input({{ year }}, {{ day }});

    let p1 = part_1(input.clone());
    println!("{{ year }} Day {{ day | zero_pad(width=2) }} - P1 - {p1}");

    let p2 = part_2(input);
    println!("{{ year }} Day {{ day | zero_pad(width=2) }} - P2 - {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_e1() {
        let input = vec![""].iter().map(|s| String::from(*s)).collect::<Vec<_>>();
        assert_eq!(part_1(input), 0);
    }

    #[test]
    fn test_p2_e1() {
        let input = vec![""].iter().map(|s| String::from(*s)).collect::<Vec<_>>();
        assert_eq!(part_2(input), 0);
    }
}
