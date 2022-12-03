use std::{collections::HashMap, str::FromStr};

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

    fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
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

#[derive(Debug, PartialEq)]
enum State {
    Cleaned,
    Infected,
    Weakened,
    Flagged,
}

struct World {
    grid: HashMap<Position, State>,
}

impl FromStr for World {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let size = (s.lines().count() / 2) as i32;
        let mut grid = HashMap::new();

        s.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    grid.insert(
                        Position {
                            x: x as i32 - size,
                            y: y as i32 - size,
                        },
                        State::Infected,
                    );
                }
            })
        });

        Ok(World { grid })
    }
}

impl World {
    fn get_cell(&self, pos: &Position) -> &State {
        self.grid.get(pos).unwrap_or(&State::Cleaned)
    }

    fn is_infected(&self, pos: &Position) -> bool {
        *self.get_cell(pos) == State::Infected
    }

    fn clean(&mut self, pos: &Position) {
        self.grid.remove(pos);
    }

    fn infect(&mut self, pos: &Position) {
        self.grid.insert(*pos, State::Infected);
    }

    fn update_cell(&mut self, pos: &Position) {
        match self.get_cell(pos) {
            State::Flagged => {
                self.grid.insert(*pos, State::Cleaned);
            }
            State::Infected => {
                self.grid.insert(*pos, State::Flagged);
            }
            State::Weakened => {
                self.grid.insert(*pos, State::Infected);
            }
            State::Cleaned => {
                self.grid.insert(*pos, State::Weakened);
            }
        }
    }
}

pub struct Day22;

impl Solution for Day22 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        Some(Self::run(input, 10_000).into())
    }

    fn solve_b(&self, _input: &str) -> Option<Answer> {
        const BURSTS: usize = 10_000_000;
        Some(Self::run_b(_input, BURSTS).into())
    }
}

impl Day22 {
    fn run(input: &str, bursts: usize) -> usize {
        let mut world: World = input.parse().unwrap();
        let mut pos: Position = Position::default();
        let mut dir: Direction = Direction::Up;
        let mut infections = 0;

        for _ in 0..bursts {
            // Step 1
            dir = if world.is_infected(&pos) {
                dir.turn_right()
            } else {
                dir.turn_left()
            };

            // Step 2
            if world.is_infected(&pos) {
                world.clean(&pos);
            } else {
                world.infect(&pos);
                infections += 1;
            }

            // Step 3
            pos = pos.forward(&dir);
        }

        infections
    }

    fn run_b(input: &str, bursts: usize) -> usize {
        let mut grid: World = input.parse().unwrap();
        let mut pos: Position = Position::default();
        let mut dir: Direction = Direction::Up;
        let mut infections = 0;

        for _ in 0..bursts {
            // Step 1
            dir = match grid.get_cell(&pos) {
                State::Cleaned => dir.turn_left(),
                State::Infected => dir.turn_right(),
                State::Weakened => dir,
                State::Flagged => dir.reverse(),
            };

            // Step 2
            grid.update_cell(&pos);
            if grid.is_infected(&pos) {
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
        let w: World = INPUT.parse().unwrap();
        assert_eq!(w.grid.len(), 2);
        assert!(w.grid.contains_key(&(-1, 0).into()));
        assert!(w.grid.contains_key(&(1, -1).into()));
    }

    #[test]
    fn run() {
        assert_eq!(Day22::run(INPUT, 7), 5);
        assert_eq!(Day22::run(INPUT, 70), 41);
    }

    #[test]
    fn test_a() {
        assert_eq!(Day22 {}.solve_a(INPUT), Some(Answer::UInt(5587)))
    }

    #[test]
    fn run_b() {
        assert_eq!(Day22::run_b(INPUT, 100), 26);
    }

    #[test]
    fn test_b() {
        assert_eq!(Day22 {}.solve_b(INPUT), Some(Answer::UInt(2511944)))
    }
}
