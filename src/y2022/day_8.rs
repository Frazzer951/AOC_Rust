use itertools::Itertools;

fn part_1(input: Vec<String>) -> usize {
    let trees: Vec<Vec<u32>> = input
        .iter()
        .map(|l| l.chars().map(|c: char| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let height = trees.len();
    let width = trees[0].len();

    let mut visible_trees = vec![vec![false; width]; height];

    for i in 0..width {
        visible_trees[0][i] = true;
        visible_trees[height - 1][i] = true;
    }

    for row in visible_trees.iter_mut() {
        row[0] = true;
        row[width - 1] = true;
    }

    // rows
    for i in 1..(height - 1) {
        // left to right
        let mut largest = trees[i][0];
        for j in 1..(width - 1) {
            if trees[i][j] > largest {
                visible_trees[i][j] = true;
                largest = trees[i][j];
                if largest >= 9 {
                    break;
                }
            }
        }

        // right to left
        let mut largest = trees[i][width - 1];
        for j in (1..(width - 1)).rev() {
            if trees[i][j] > largest {
                visible_trees[i][j] = true;
                largest = trees[i][j];
                if largest >= 9 {
                    break;
                }
            }
        }
    }

    // cols
    for i in 1..(width - 1) {
        // top to bottom
        let mut largest = trees[0][i];
        for j in 1..(height - 1) {
            if trees[j][i] > largest {
                visible_trees[j][i] = true;
                largest = trees[j][i];
                if largest >= 9 {
                    break;
                }
            }
        }

        // bottom to top
        let mut largest = trees[height - 1][i];
        for j in (1..(height - 1)).rev() {
            if trees[j][i] > largest {
                visible_trees[j][i] = true;
                largest = trees[j][i];
                if largest >= 9 {
                    break;
                }
            }
        }
    }

    visible_trees.iter().map(|t| t.iter().filter(|&b| *b).count()).sum()
}

fn part_2(input: Vec<String>) -> i32 {
    let trees: Vec<Vec<u32>> = input
        .iter()
        .map(|l| l.chars().map(|c: char| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let height = trees.len();
    let width = trees[0].len();

    let mut tree_scores = vec![vec![0; width]; height];

    for row in 1..(height - 1) {
        for col in 1..(width - 1) {
            let tree_height = trees[row][col];

            let mut left = 0;
            for i in (0..col).rev() {
                left += 1;
                if trees[row][i] >= tree_height {
                    break;
                }
            }

            let mut right = 0;
            for i in (col + 1)..width {
                right += 1;
                if trees[row][i] >= tree_height {
                    break;
                }
            }

            let mut up = 0;
            for i in (0..row).rev() {
                up += 1;
                if trees[i][col] >= tree_height {
                    break;
                }
            }

            let mut down = 0;
            for row in trees.iter().take(height).skip(row + 1) {
                down += 1;
                if row[col] >= tree_height {
                    break;
                }
            }

            tree_scores[row][col] = left * right * up * down;
        }
    }

    *tree_scores.iter().map(|r| r.iter().max().unwrap()).max().unwrap()
}

pub fn run() {
    let input = crate::utils::read_input(2022, 8);

    println!(" Day 08:");

    let p1 = part_1(input.clone());
    println!("    Part 1 - {p1}");

    let p2 = part_2(input);
    println!("    Part 2 - {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<String> {
        ["30373", "25512", "65332", "33549", "35390"]
            .iter()
            .map(|s| String::from(*s))
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_p1() {
        let input = example_input();
        assert_eq!(part_1(input), 21);
    }

    #[test]
    fn test_p2() {
        let input = example_input();
        assert_eq!(part_2(input), 8);
    }
}
