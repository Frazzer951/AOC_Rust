use std::str::CharIndices;

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

pub fn run() {
    let input = crate::utils::read_input(2022, 6)[0].to_owned();

    println!(" Day 06:");

    let p1 = part_1(input.clone());
    println!("    Part 1 - {p1}");

    let p2 = part_2(input);
    println!("    Part 2 - {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_owned()), 7);
        assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned()), 5);
        assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg".to_owned()), 6);
        assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_owned()), 10);
        assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_owned()), 11);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_owned()), 19);
        assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned()), 23);
        assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg".to_owned()), 23);
        assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_owned()), 29);
        assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_owned()), 26);
    }
}
