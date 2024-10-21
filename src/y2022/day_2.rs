use crate::years::Day;

pub struct AocDay;

impl Day for AocDay {
    fn run(&self) {
        let input = crate::utils::read_input(2022, 2);

        println!(" Day 02:");

        let p1 = part_1(input.clone());
        println!("    Part 1 - {p1}");

        let p2 = part_2(input);
        println!("    Part 2 - {p2}");
    }
}

#[derive(Debug, Clone, Copy)]
enum Moves {
    Rock,
    Paper,
    Scissors,
}

impl Moves {
    fn new(c: &str) -> Self {
        match c {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("invalid move"),
        }
    }

    fn get_move(opponent: &Moves, result: &GameResult) -> Self {
        match (result, opponent) {
            (GameResult::Tie, mv) => *mv,
            (GameResult::Win, Moves::Rock) => Moves::Paper,
            (GameResult::Win, Moves::Paper) => Moves::Scissors,
            (GameResult::Win, Moves::Scissors) => Moves::Rock,
            (GameResult::Loss, Moves::Rock) => Moves::Scissors,
            (GameResult::Loss, Moves::Paper) => Moves::Rock,
            (GameResult::Loss, Moves::Scissors) => Moves::Paper,
        }
    }
}

#[derive(Debug)]
enum GameResult {
    Win,
    Tie,
    Loss,
}

impl GameResult {
    fn new(c: &str) -> Self {
        match c {
            "Z" => Self::Win,
            "Y" => Self::Tie,
            "X" => Self::Loss,
            _ => panic!("invalid move"),
        }
    }
}

fn get_result(p1: &Moves, p2: &Moves) -> GameResult {
    match (p1, p2) {
        (Moves::Rock, Moves::Rock)
        | (Moves::Paper, Moves::Paper)
        | (Moves::Scissors, Moves::Scissors) => GameResult::Tie,
        (Moves::Rock, Moves::Scissors)
        | (Moves::Scissors, Moves::Paper)
        | (Moves::Paper, Moves::Rock) => GameResult::Win,
        _ => GameResult::Loss,
    }
}

fn score_moves(p1: Moves, p2: Moves) -> i32 {
    let result = get_result(&p1, &p2);
    let mut score = 0;

    score += match p1 {
        Moves::Rock => 1,
        Moves::Paper => 2,
        Moves::Scissors => 3,
    };

    score += match result {
        GameResult::Win => 6,
        GameResult::Tie => 3,
        GameResult::Loss => 0,
    };

    score
}

fn part_1(input: Vec<String>) -> i32 {
    let mut score = 0;

    for line in input {
        let moves = line.split(' ').collect::<Vec<_>>();
        //println!("Moves: {moves:?}");
        let opponent = Moves::new(moves[0]);
        let mine = Moves::new(moves[1]);
        score += score_moves(mine, opponent);
    }

    score
}

fn part_2(input: Vec<String>) -> i32 {
    let mut score = 0;

    for line in input {
        let moves = line.split(' ').collect::<Vec<_>>();
        //println!("Moves: {moves:?}");
        let opponent = Moves::new(moves[0]);
        let result = GameResult::new(moves[1]);
        let mine = Moves::get_move(&opponent, &result);
        score += score_moves(mine, opponent);
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_e1() {
        let input = ["A Y", "B X", "C Z"]
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<_>>();

        assert_eq!(part_1(input), 15);
    }

    #[test]
    fn test_p2_e1() {
        let input = ["A Y", "B X", "C Z"]
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<_>>();

        assert_eq!(part_2(input), 12);
    }
}
