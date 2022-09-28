use std::{collections::HashSet, str::FromStr};

use crate::solutions::{answer::Answer, Solution};

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Default, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Position {
    fn from(tup: (i32, i32)) -> Self {
        Position { x: tup.0, y: tup.1 }
    }
}

impl std::ops::Add<(i32, i32)> for Position {
    type Output = Position;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Position {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl Position {
    fn forward(&self, current_dir: &Direction) -> Self {
        match current_dir {
            Direction::Up => *self + (0, -1),
            Direction::Right => *self + (1, 0),
            Direction::Down => *self + (0, 1),
            Direction::Left => *self + (-1, 0),
        }
    }
}

struct Grid {
    grid: HashSet<Position>,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size = (s.lines().count() / 2) as i32;
        let mut grid = HashSet::new();

        s.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| match c {
                '#' => {
                    grid.insert(Position {
                        x: x as i32 - size,
                        y: y as i32 - size,
                    });
                }
                _ => {}
            })
        });

        Ok(Grid { grid })
    }
}

pub struct Day22;

impl Solution for Day22 {
    fn solve_a(&self, input: &str) -> Answer {
        Self::run(input, 10_000).into()
    }

    fn solve_b(&self, _input: &str) -> Answer {
        todo!()
    }
}

impl Day22 {
    fn run(input: &str, bursts: usize) -> usize {
        let mut grid: Grid = input.parse().unwrap();
        let mut pos: Position = Position::default();
        let mut dir: Direction = Direction::Up;
        let mut infections = 0;

        for _ in 0..bursts {
            // Step 1
            dir = if grid.grid.contains(&pos) {
                dir.turn_right()
            } else {
                dir.turn_left()
            };

            // Step 2
            if grid.grid.contains(&pos) {
                grid.grid.remove(&pos);
            } else {
                grid.grid.insert(pos);
                infections += 1;
            }

            // Step 3
            pos = pos.forward(&dir);
        }

        infections
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "..#\n#..\n...";

    #[test]
    fn parse_grid() {
        let grid: Grid = INPUT.parse().unwrap();
        assert_eq!(grid.grid.len(), 2);
        assert!(grid.grid.contains(&(-1, 0).into()));
        assert!(grid.grid.contains(&(1, -1).into()));
    }

    #[test]
    fn run() {
        assert_eq!(Day22::run(INPUT, 7), 5);
        assert_eq!(Day22::run(INPUT, 70), 41);
    }

    #[test]
    fn test_a() {
        assert_eq!(Day22 {}.solve_a(INPUT), Answer::UInt(5587));
    }
}
