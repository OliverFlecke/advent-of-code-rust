use std::collections::VecDeque;

use array2d::Array2D;

use crate::solutions::{answer::Answer, Solution};

const ROCK_ROUND: char = 'O';
const ROCK_CUBE: char = '#';
const EMPTY_SPACE: char = '.';
const CYCLES: usize = 1_000_000_000;

pub struct Day14;

impl Solution for Day14 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let world = parse_world(input);
        let answer = weight_a(&world);

        Some(answer.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let world = parse_world(input);
        let answer = find_cycle(world);
        Some(answer.into())
    }
}

type World = Array2D<char>;

fn weight_a(world: &World) -> usize {
    let column_length = world.num_rows();
    world
        .columns_iter()
        .map(|col| {
            let mut weight = 0;
            let mut empty = VecDeque::new();
            for (i, c) in col.enumerate() {
                match *c {
                    ROCK_CUBE => empty.clear(),
                    ROCK_ROUND => {
                        let value = if let Some(first_empty) = empty.pop_front() {
                            empty.push_back(i);
                            first_empty
                        } else {
                            i
                        };
                        weight += column_length - value;
                    }
                    EMPTY_SPACE => empty.push_back(i),
                    _ => unreachable!(),
                }
            }

            weight
        })
        .sum()
}

// fn print_world(world: &World) {
//     world.rows_iter().for_each(|row| {
//         row.for_each(|c| print!("{c}"));
//         println!();
//     });
// }

/// Parse the input string into a grid world.
fn parse_world(input: &str) -> World {
    let grid: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    Array2D::from_rows(&grid).unwrap()
}

fn weight(world: &World) -> usize {
    world
        .columns_iter()
        .map(|col| {
            col.enumerate()
                .filter(|(_, c)| **c == ROCK_ROUND)
                .map(|(i, _)| world.num_columns() - i)
                .sum::<usize>()
        })
        .sum()
}

fn find_cycle(mut world: World) -> usize {
    let mut seen = vec![world.clone()];
    for _ in 0..CYCLES {
        world = cycle(world);

        if let Some(idx) = seen.iter().position(|x| x == &world) {
            let cycle_len = seen.len() - idx;
            let final_idx = idx + (CYCLES - idx) % cycle_len;
            let final_world = &seen[final_idx];

            return weight(final_world);
        }

        seen.push(world.clone());
    }

    unreachable!()
}

fn cycle(mut world: World) -> World {
    for _ in 0..4 {
        slide(&mut world);
        world = rotate_clockwise(&world);
    }

    world
}

fn slide(world: &mut World) {
    for col in 0..world.num_rows() {
        let mut empty = VecDeque::new();
        for row in 0..world.num_columns() {
            match world.get(row, col) {
                Some(&ROCK_CUBE) => empty.clear(),
                Some(&ROCK_ROUND) => {
                    if let Some(first_empty) = empty.pop_front() {
                        empty.push_back(row);

                        if let Some(c) = world.get_mut(first_empty, col) {
                            *c = ROCK_ROUND;
                        }
                        if let Some(c) = world.get_mut(row, col) {
                            *c = EMPTY_SPACE;
                        }
                    };
                }
                Some(&EMPTY_SPACE) => empty.push_back(row),
                _ => unreachable!(),
            }
        }
    }
}

fn rotate_clockwise(world: &World) -> World {
    // Assumption: rows and columns must always be the same length.
    let mut new = World::filled_with('.', world.num_rows(), world.num_columns());
    world.rows_iter().enumerate().for_each(|(row, elements)| {
        elements.enumerate().for_each(|(col, c)| {
            if let Some(x) = new.get_mut(col, world.num_rows() - 1 - row) {
                *x = *c;
            }
        })
    });
    new
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

    #[test]
    fn test_b() {
        assert_eq!(Day14 {}.solve_b(INPUT), Some(Answer::UInt(64)));
    }

    #[test]
    fn solve_b() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day14 {}.solve_b(&input), Some(Answer::UInt(94255)));
    }
}
