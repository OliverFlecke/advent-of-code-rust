use array2d::Array2D;
use rustc_hash::FxHashSet;

use crate::solutions::{answer::Answer, Solution};

pub struct Day10;

// The pipes are arranged in a two-dimensional grid of tiles:

//     | is a vertical pipe connecting north and south.
//     - is a horizontal pipe connecting east and west.
//     L is a 90-degree bend connecting north and east.
//     J is a 90-degree bend connecting north and west.
//     7 is a 90-degree bend connecting south and west.
//     F is a 90-degree bend connecting south and east.
//     . is ground; there is no pipe in this tile.
//     S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

const NORTH_SOUTH: char = '|';
const EAST_WEST: char = '-';
const NORTH_EAST: char = 'L';
const NORTH_WEST: char = 'J';
const SOUTH_WEST: char = '7';
const SOUTH_EAST: char = 'F';
const GROUND: char = '.';
const START: char = 'S';

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Solution for Day10 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let cells: Vec<Vec<_>> = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let map = Array2D::from_rows(&cells).unwrap();
        let (row_0, col_0) = map
            .enumerate_row_major()
            .find(|(_, c)| **c == START)
            .map(|(pos, _)| pos)
            .unwrap();
        println!("Start: {}, {}: {:?}", row_0, col_0, map.get(row_0, col_0));

        use Direction::*;
        // Note (row, col) / (y, x) pairs
        let neighbours = &[
            (
                (row_0 + 1, col_0),
                North,
                [NORTH_SOUTH, NORTH_EAST, NORTH_WEST],
            ),
            (
                (row_0.wrapping_sub(1), col_0),
                South,
                [NORTH_SOUTH, SOUTH_EAST, SOUTH_WEST],
            ),
            (
                (row_0, col_0 + 1),
                West,
                [EAST_WEST, NORTH_WEST, SOUTH_WEST],
            ),
            (
                (row_0, col_0.wrapping_sub(1)),
                East,
                [EAST_WEST, NORTH_EAST, SOUTH_EAST],
            ),
        ];
        let connected: Vec<_> = neighbours
            .iter()
            .filter(|(pos, _, allowed)| {
                map.get(pos.0, pos.1)
                    .filter(|c| allowed.contains(c))
                    .is_some()
            })
            .map(|(pos, dir, _)| (pos, dir))
            .collect();
        println!("connected: {:?}", connected);

        let step = |pos: Position, from_dir: Direction| -> (Position, Direction) {
            match *map.get(pos.0, pos.1).unwrap() {
                NORTH_SOUTH if from_dir == North => next(pos, South),
                NORTH_SOUTH if from_dir == South => next(pos, North),
                NORTH_EAST if from_dir == North => next(pos, East),
                NORTH_EAST if from_dir == East => next(pos, North),
                NORTH_WEST if from_dir == North => next(pos, West),
                NORTH_WEST if from_dir == West => next(pos, North),

                SOUTH_EAST if from_dir == South => next(pos, East),
                SOUTH_EAST if from_dir == East => next(pos, South),
                SOUTH_WEST if from_dir == South => next(pos, West),
                SOUTH_WEST if from_dir == West => next(pos, South),

                EAST_WEST if from_dir == East => next(pos, West),
                EAST_WEST if from_dir == West => next(pos, East),

                pipe => unreachable!(
                    "Got invalid pipe {pipe} at {:?} with dir {:?}",
                    pos, from_dir
                ),
            }
        };

        let mut i: usize = 1;
        for (&pos, &dir) in connected.iter().take(1) {
            let mut current = pos;
            let mut current_dir = dir;
            println!(
                "start: {:?} {:?} {:?}",
                current,
                current_dir,
                map.get(current.0, current.1)
            );
            let mut visited = FxHashSet::default();
            while current != (row_0, col_0) {
                // println!("Stepping: {:?} from {:?}", current, current_dir);
                let next = step(current, current_dir);
                current = next.0;
                current_dir = next.1;
                println!("next: {:?} {:?}", next, map.get(current.0, current.1));

                if visited.contains(&current) {
                    break;
                }

                visited.insert(current);
                i += 1;
            }
        }
        println!();

        Some((i / 2).into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        None
    }
}

type Position = (usize, usize);

fn next((row, col): Position, direction: Direction) -> (Position, Direction) {
    use Direction::*;
    let new_position = match direction {
        North => (row - 1, col),
        South => (row + 1, col),
        East => (row, col + 1),
        West => (row, col - 1),
    };
    let from_direction = match direction {
        North => South,
        East => West,
        South => North,
        West => East,
    };

    (new_position, from_direction)
}
// NORTH_SOUTH if **dir == North => ((pos.0 + 1, pos.1), North),
// NORTH_SOUTH if **dir == South => ((pos.0 - 1, pos.1), South),
// NORTH_EAST if **dir == North => ((pos.0, pos.1 + 1), East),
// NORTH_EAST if **dir == East => ((pos.0 - 1, pos.1), North),
// NORTH_WEST if **dir == North => ((pos.0, pos.1 - 1), West),
// NORTH_WEST if **dir == West => ((pos.0 - 1, pos.1), North),

// SOUTH_EAST if **dir == South => ((pos.0, pos.1 + 1), East),
// SOUTH_EAST if **dir == East => ((pos.0 + 1, pos.1), South),
// SOUTH_WEST if **dir == South => ((pos.0, pos.1 + 1), West),
// SOUTH_WEST if **dir == West => ((pos.0 + 1, pos.1), South),

// EAST_WEST if **dir == East => ((pos.0, pos.1 - 1), West),
// EAST_WEST if **dir == West => ((pos.0, pos.1 + 1), East),

#[cfg(test)]
mod test {
    use advent_of_code_client::{AocClient, Problem, Year};

    use super::*;

    const PROBLEM: Problem = Problem::new(Year::Y2023, 10);
    const INPUT: &str = r#".....
.S-7.
.|.|.
.L-J.
.....
"#;

    const INPUT_2: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;

    #[test]
    fn test_a() {
        assert_eq!(Day10 {}.solve_a(INPUT), Some(Answer::UInt(4)));
        assert_eq!(Day10 {}.solve_a(INPUT_2), Some(Answer::UInt(8)));
    }

    #[test]
    fn solve_a() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day10 {}.solve_a(&input), Some(Answer::UInt(6882)));
    }

    // #[test]
    // fn test_b() {
    //     assert_eq!(Day10 {}.solve_b(INPUT), Some(Answer::UInt(todo!())));
    // }

    // #[test]
    // fn solve_b() {
    //     let input = AocClient::default().get_input(PROBLEM).unwrap();
    //     assert_eq!(Day10 {}.solve_b(&input), Some(Answer::UInt(todo!())));
    // }
}
