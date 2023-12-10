use std::collections::HashSet;

use array2d::Array2D;

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
// const GROUND: char = '.';
const START: char = 'S';

const PIPE: char = '#';

impl Solution for Day10 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let cells: Vec<Vec<_>> = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let map = Array2D::from_rows(&cells).unwrap();
        let start = map
            .enumerate_row_major()
            .find(|(_, c)| **c == START)
            .map(|(pos, _)| pos)
            .unwrap();
        let &(mut pos, mut dir) = find_starting_neighbours(&map, start).first().unwrap();

        let mut i: usize = 1;
        while pos != start {
            (pos, dir) = step(&map, pos, dir);
            println!("next: {:?} {:?} {:?}", pos, dir, map.get(pos.0, pos.1));

            i += 1;
        }

        Some((i / 2).into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let cells: Vec<Vec<_>> = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let mut map = Array2D::from_rows(&cells).unwrap();
        let start = map
            .enumerate_row_major()
            .find(|(_, c)| **c == START)
            .map(|(pos, _)| pos)
            .unwrap();

        let neighbours = find_starting_neighbours(&map, start);
        assert!(neighbours.len() == 2);
        let start_symbol = find_start_symbol(neighbours.iter().map(|x| x.1));
        println!("Start symbol: {start_symbol}");
        map.set(start.0, start.1, start_symbol).unwrap();

        let &(mut pos, mut dir) = neighbours.first().unwrap();
        let mut tracking = Array2D::filled_with(' ', map.num_rows(), map.num_columns());
        tracking.set(start.0, start.1, PIPE).unwrap();
        tracking.set(pos.0, pos.1, PIPE).unwrap();
        println!("Start pos: {pos:?}");

        while pos != start {
            (pos, dir) = step(&map, pos, dir);
            tracking.set(pos.0, pos.1, PIPE).unwrap();
        }

        fn is_pipe(c: char, last: Option<char>) -> bool {
            c == NORTH_SOUTH
                || (last == Some(SOUTH_EAST) && c == NORTH_WEST)
                || (last == Some(NORTH_EAST) && c == SOUTH_WEST)
        }

        let mut inside = HashSet::new();
        let answer: usize = tracking
            .rows_iter()
            .enumerate()
            .map(|(r, row)| {
                row.enumerate()
                    .fold((0, false, None), |(sum, is_inside, last), (c, &is_tube)| {
                        let kind = map[(r, c)];
                        match (is_tube, is_inside) {
                            (PIPE, _) if is_pipe(kind, last) => (sum, !is_inside, None),
                            (PIPE, _) if kind == SOUTH_EAST || kind == NORTH_EAST => {
                                (sum, is_inside, Some(kind))
                            }
                            (PIPE, _) => (sum, is_inside, last),
                            (_, true) => {
                                inside.insert((r, c));
                                // println!("Adding at {r},{c}. {kind} last: {last:?}");

                                (sum + 1, is_inside, last)
                            }
                            (_, false) => (sum, is_inside, last),
                        }
                    })
                    .0
            })
            .sum();

        print!(" ");
        (0..tracking.num_columns()).for_each(|c| print!("{}", c % 10));
        println!();
        tracking.rows_iter().enumerate().for_each(|(r, row)| {
            print!("{r: >3}");
            row.enumerate().for_each(|(c, &ch)| {
                if inside.contains(&(r, c)) {
                    print!("I");
                } else if ch == PIPE {
                    print!("{}", map[(r, c)]);
                } else {
                    print!(" ");
                }
            });
            println!();
        });

        Some(answer.into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const fn inverse(&self) -> Self {
        use Direction::*;
        match *self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
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

    (new_position, direction.inverse())
}

fn step(map: &Array2D<char>, pos: Position, from_dir: Direction) -> (Position, Direction) {
    use Direction::*;

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
}

fn find_start_symbol(directions: impl Iterator<Item = Direction>) -> char {
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
        .unwrap()
}

fn find_starting_neighbours(
    map: &Array2D<char>,
    (row_0, col_0): Position,
) -> Vec<(Position, Direction)> {
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
    neighbours
        .iter()
        .filter(|(pos, _, allowed)| {
            map.get(pos.0, pos.1)
                .filter(|c| allowed.contains(c))
                .is_some()
        })
        .map(|(pos, dir, _)| (*pos, *dir))
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
