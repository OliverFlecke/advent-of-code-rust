use std::collections::VecDeque;

use array2d::Array2D;

use crate::solutions::{answer::Answer, Solution};

const ROCK_ROUND: char = 'O';
const ROCK_CUBE: char = '#';
const EMPTY_SPACE: char = '.';

pub struct Day14;

impl Solution for Day14 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let grid: Vec<Vec<_>> = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let world = Array2D::from_rows(&grid).unwrap();

        let mut total = 0;
        let column_length = world.num_rows();
        for col in world.columns_iter() {
            // println!("{:?}", col.collect::<Vec<_>>());

            let mut sum = 0;
            let mut empty = VecDeque::new();
            for (i, c) in col.enumerate() {
                match *c {
                    ROCK_CUBE => empty.clear(),
                    ROCK_ROUND => {
                        print!("Empty: {:?} ", empty);
                        let value = if let Some(first_empty) = empty.pop_front() {
                            empty.push_back(i);
                            first_empty
                        } else {
                            i
                        };
                        let value = column_length - value;
                        println!("Adding at {}", value);
                        // empty.push_back(i);
                        sum += value;
                    }
                    EMPTY_SPACE => {
                        // println!("Found empty space at {}", i);
                        empty.push_back(i);
                    }
                    _ => unreachable!(),
                }
            }

            println!("Sum: {sum}\n");
            total += sum;
        }

        println!("total: {total}");

        Some(total.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        None
    }
}

#[cfg(test)]
mod test {
    use advent_of_code_client::{AocClient, Problem, Year};

    use super::*;

    const PROBLEM: Problem = Problem::new(Year::Y2023, 14);
    const INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#;

    #[test]
    fn test_a() {
        assert_eq!(Day14 {}.solve_a(INPUT), Some(Answer::UInt(136)));
    }

    #[test]
    fn solve_a() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day14 {}.solve_a(&input), Some(Answer::UInt(113078)));
    }

    // #[test]
    // fn test_b() {
    //     assert_eq!(Day14 {}.solve_b(INPUT), Some(Answer::UInt(todo!())));
    // }

    // #[test]
    // fn solve_b() {
    //     let input = AocClient::default().get_input(PROBLEM).unwrap();
    //     assert_eq!(Day14 {}.solve_b(&input), Some(Answer::UInt(todo!())));
    // }
}
