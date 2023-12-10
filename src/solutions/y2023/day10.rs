//! General approach is to first find the starting neighbours by checking which
//! direct neighbours to 'S' is a connected pipe. The position together with the
//! direction which the pipe was entered from is used to calculate the next
//! position. This can be described with a set of rules; see `step`. After this,
//! it is just following the path until we reach the `start` position again.
//! Instead of searching in both direction, we can just search one to the end
//! and divide the length of the pipe by half.
//!
//! For part B, we scan each row individually and keep track of the parity of
//! how many vertical pipes we have passed by, given that we follow the pipe in
//! one direction. If we have passed an odd number of vertical pipes, we are
//! inside, even is outside.
//! While `|` is clearly included, `FJ` and `L7` also needs to be considered as
//! a vertical pipe, while `F7` and `LJ` should not (note the choice of the two
//! pairs here is arbirary)
use std::convert::Into;

use array2d::Array2D;
use rustc_hash::FxHashSet;

use crate::{
    solutions::{answer::Answer, Solution},
    utils::map2d::{Direction, Position},
};

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
const START: char = 'S';

impl Solution for Day10 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let map = build_map(input);
        let start = find_start(&map);
        let &(pos, dir) = find_starting_neighbours(&map, start).first().unwrap();

        let answer = {
            let mut i: usize = 1;
            traverse(&map, start, pos, dir, |_| i += 1);
            i
        };

        Some((answer / 2).into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let mut map = build_map(input);
        let start = find_start(&map);

        let neighbours = find_starting_neighbours(&map, start);
        assert!(neighbours.len() == 2);

        // Translate the start symbol `S` into its correct pipe kind.
        find_start_symbol(neighbours.iter().map(|x| x.1))
            .inspect(|symbol| map.set(start.row, start.col, *symbol).unwrap());

        let pipe_positions = {
            let &(pos, dir) = neighbours.first().unwrap();
            let mut set = FxHashSet::default();
            set.insert(start);
            set.insert(pos);

            traverse(&map, start, pos, dir, |pos| {
                set.insert(*pos);
            });
            set
        };

        let answer: usize = map
            .rows_iter()
            .enumerate()
            .map(|(row, elements)| {
                elements
                    .enumerate()
                    .fold((0, false, None), |(sum, is_inside, last), (col, &kind)| {
                        // Check if a pipe is vertical, based on the current pipe and last `F` or `7`.
                        fn is_vertical_pipe(c: char, last: Option<char>) -> bool {
                            c == NORTH_SOUTH
                                || (last == Some(SOUTH_EAST) && c == NORTH_WEST)
                                || (last == Some(NORTH_EAST) && c == SOUTH_WEST)
                        }

                        match (pipe_positions.contains(&(row, col).into()), is_inside) {
                            (true, _) if is_vertical_pipe(kind, last) => (sum, !is_inside, None),
                            (true, _) if kind == SOUTH_EAST || kind == NORTH_EAST => {
                                (sum, is_inside, Some(kind))
                            }
                            (true, _) => (sum, is_inside, last),
                            (_, true) => (sum + 1, is_inside, last),
                            (_, false) => (sum, is_inside, last),
                        }
                    })
                    .0
            })
            .sum();

        Some(answer.into())
    }
}

type Map = Array2D<char>;

/// Traverse along the pipe.
fn traverse<F: FnMut(&Position)>(
    map: &Map,
    start: Position,
    mut pos: Position,
    mut dir: Direction,
    mut fun: F,
) {
    while pos != start {
        (pos, dir) = step(map, pos, dir);
        // println!("next: {:?} {:?} {:?}", pos, dir, map.get(pos.0, pos.1));

        fun(&pos);
    }
}

fn step(map: &Map, pos: Position, from_dir: Direction) -> (Position, Direction) {
    use Direction::*;
    fn next(pos: Position, dir: Direction) -> (Position, Direction) {
        (pos.move_direction(dir), dir.inverse())
    }

    match *map.get(pos.row, pos.col).unwrap() {
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
}

fn find_start_symbol(directions: impl Iterator<Item = Direction>) -> Option<char> {
    use Direction::*;
    directions
        .map_windows(|[a, b]| match (a, b) {
            (North, East) => SOUTH_WEST,
            (North, South) => NORTH_SOUTH,
            (North, West) => SOUTH_EAST,
            (East, West) | (West, East) => EAST_WEST,
            (South, East) => NORTH_WEST,
            (South, West) => NORTH_EAST,

            _ => unreachable!("Combination should not be possible {a:?}/{b:?}"),
        })
        .next()
}

fn build_map(input: &str) -> Map {
    let cells: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    Array2D::from_rows(&cells).expect("Invalid input. Could not build map")
}

fn find_start(map: &Map) -> Position {
    map.enumerate_row_major()
        .find(|(_, c)| **c == START)
        .map(|(pos, _)| pos.into())
        .expect("No starting position marked with 'S'")
}

/// Find the neighbours next to the given position, which is connected with a
/// pipe.
fn find_starting_neighbours(map: &Map, pos: Position) -> Vec<(Position, Direction)> {
    use Direction::*;

    let Position { row, col } = pos;
    // Note (row, col) / (y, x) pairs
    // Using `wrapping_sub` to overflow and get `None` at `map.get`.
    [
        ((row + 1, col), North, [NORTH_SOUTH, NORTH_EAST, NORTH_WEST]),
        (
            (row.wrapping_sub(1), col),
            South,
            [NORTH_SOUTH, SOUTH_EAST, SOUTH_WEST],
        ),
        ((row, col + 1), West, [EAST_WEST, NORTH_WEST, SOUTH_WEST]),
        (
            (row, col.wrapping_sub(1)),
            East,
            [EAST_WEST, NORTH_EAST, SOUTH_EAST],
        ),
    ]
    .iter()
    .filter(|(pos, _, allowed)| {
        map.get(pos.0, pos.1)
            .filter(|c| allowed.contains(c))
            .is_some()
    })
    .map(|(pos, dir, _)| (Into::<Position>::into(*pos), *dir))
    .collect()
}

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

    #[test]
    fn test_b() {
        let input = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;
        assert_eq!(Day10 {}.solve_b(input), Some(Answer::UInt(4)));
        let input = r#"..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
.........."#;
        assert_eq!(Day10 {}.solve_b(input), Some(Answer::UInt(4)));

        let input = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;
        assert_eq!(Day10 {}.solve_b(input), Some(Answer::UInt(8)));

        let input = r#"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#;
        assert_eq!(Day10 {}.solve_b(input), Some(Answer::UInt(10)));
    }

    #[test]
    fn solve_b() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day10 {}.solve_b(&input), Some(Answer::UInt(491)));
    }
}
