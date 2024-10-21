use crate::years::Day;
use std::str::CharIndices;

pub struct AocDay;

impl Day for AocDay {
    fn run(&self) {
        let input = crate::utils::read_input(2022, 6)[0].to_owned();

        println!(" Day 06:");

        let p1 = part_1(input.clone());
        println!("    Part 1 - {p1}");

        let p2 = part_2(input);
        println!("    Part 2 - {p2}");
    }
}

fn find_marker(mut char_stream: CharIndices, num_distinct: usize) -> usize {
    let mut buff = vec![(0, ' '); num_distinct];
    let mut index = 0;

    // init buffer
    for _ in 0..(num_distinct - 1) {
        let (pos, char) = char_stream.next().unwrap();
        buff[index] = (pos, char);
        index += 1;
    }

    // main process loop
    'main: loop {
        let (pos, char) = char_stream.next().unwrap();
        buff[index] = (pos, char);
        index = (index + 1) % num_distinct;

        for i in 0..(num_distinct - 1) {
            for j in (i + 1)..(num_distinct) {
                if buff[i].1 == buff[j].1 {
                    continue 'main;
                }
            }
        }
        return pos + 1;
    }
}

fn part_1(input: String) -> usize {
    find_marker(input.char_indices(), 4)
}

fn part_2(input: String) -> usize {
    find_marker(input.char_indices(), 14)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_p1(input: &str, index: usize) {
        assert_eq!(part_1(input.to_owned()), index);
    }

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_p2(input: &str, index: usize) {
        assert_eq!(part_2(input.to_owned()), index);
    }
}
