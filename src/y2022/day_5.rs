use crate::years::Day;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{all_consuming, map, map_res, opt},
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};
use std::fmt;

pub struct AocDay;

impl Day for AocDay {
    fn run(&self) {
        let input = crate::utils::read_input(2022, 5);

        println!(" Day 05:");

        let p1 = part_1(input.clone());
        println!("    Part 1 - {p1}");

        let p2 = part_2(input);
        println!("    Part 2 - {p2}");
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Crate(char);

impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

struct Piles(Vec<Vec<Crate>>);

impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }
        Ok(())
    }
}

impl Piles {
    fn apply(&mut self, ins: Instruction) {
        for _ in 0..ins.quantity {
            let el = self.0[ins.src].pop().unwrap();
            self.0[ins.dst].push(el);
        }
    }

    fn apply_p2(&mut self, ins: Instruction) {
        for cr in (0..ins.quantity)
            .map(|_| self.0[ins.src].pop().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            self.0[ins.dst].push(cr);
        }
    }
}

fn parse_crate(i: &str) -> IResult<&str, Crate> {
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_char)(i)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(i)
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

fn parse_pile_number(i: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(i)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (mut i, c) = parse_crate_or_hole(i)?;
    let mut v = vec![c];

    loop {
        let (next_i, maybe_c) = opt(preceded(tag(" "), parse_crate_or_hole))(i)?;
        match maybe_c {
            Some(c) => v.push(c),
            None => break,
        }
        i = next_i;
    }

    Ok((i, v))
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(i)
}

fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn parse_input(input: Vec<String>) -> (Vec<Vec<Crate>>, Vec<Instruction>) {
    let mut lines = input.iter();

    let crate_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();

    let crate_columns = transpose_rev(crate_lines);

    // we've consumed the "numbers line" but not the separating line
    assert!(lines.next().unwrap().is_empty());

    let instructions: Vec<_> = lines
        .map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1)
        .collect();

    (crate_columns, instructions)
}

fn part_1(input: Vec<String>) -> String {
    let (crate_columns, instructions) = parse_input(input);

    let mut piles = Piles(crate_columns);

    //println!("{piles:?}");

    for ins in instructions {
        //println!("{ins:?}");
        piles.apply(ins);
        //println!("{piles:?}");
    }

    piles.0.iter().map(|pile| *pile.last().unwrap()).join("")
}

fn part_2(input: Vec<String>) -> String {
    let (crate_columns, instructions) = parse_input(input);

    let mut piles = Piles(crate_columns);

    //println!("{piles:?}");

    for ins in instructions {
        //println!("{ins:?}");
        piles.apply_p2(ins);
        //println!("{piles:?}");
    }

    piles.0.iter().map(|pile| *pile.last().unwrap()).join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<String> {
        [
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
            "move 2 from 2 to 1",
            "move 1 from 1 to 2",
        ]
        .iter()
        .map(|s| String::from(*s))
        .collect::<Vec<_>>()
    }

    #[test]
    fn test_parse_input() {
        let input = example_input();

        let (crate_columns, instructions) = parse_input(input);

        assert_eq!(
            crate_columns,
            vec![
                vec![Crate('Z'), Crate('N')],
                vec![Crate('M'), Crate('C'), Crate('D')],
                vec![Crate('P')],
            ]
        );

        assert_eq!(
            instructions,
            vec![
                Instruction {
                    quantity: 1,
                    src: 1,
                    dst: 0
                },
                Instruction {
                    quantity: 3,
                    src: 0,
                    dst: 2
                },
                Instruction {
                    quantity: 2,
                    src: 1,
                    dst: 0
                },
                Instruction {
                    quantity: 1,
                    src: 0,
                    dst: 1
                }
            ]
        );
    }

    #[test]
    fn test_p1_e1() {
        let input = example_input();
        assert_eq!(part_1(input), "CMZ".to_owned());
    }

    #[test]
    fn test_p2_e1() {
        let input = example_input();
        assert_eq!(part_2(input), "MCD".to_owned());
    }
}
