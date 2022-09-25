use std::{fmt::Display, ops};

use crate::solutions::{answer::Answer, Solution};

pub struct Day19;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn orthogonal_directions(&self) -> [Self; 2] {
        match self {
            Direction::Up => [Direction::Left, Direction::Right],
            Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left => [Direction::Up, Direction::Down],
            Direction::Right => [Direction::Up, Direction::Down],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Position {
    fn get_x(&self) -> usize {
        self.x as usize
    }

    fn get_y(&self) -> usize {
        self.y as usize
    }

    fn next_in_dir(&self, dir: Direction) -> Self {
        match dir {
            Direction::Up => *self + (0, -1),
            Direction::Down => *self + (0, 1),
            Direction::Left => *self + (-1, 0),
            Direction::Right => *self + (1, 0),
        }
    }

    fn get_symbol(&self, map: &Vec<Vec<char>>) -> Option<char> {
        if 0 <= self.y
            && self.get_y() < map.len()
            && 0 <= self.x
            && self.get_x() < map[self.get_y()].len()
        {
            Some(map[self.get_y()][self.get_x()])
        } else {
            None
        }
    }
}

impl ops::Add<(i64, i64)> for Position {
    type Output = Position;

    fn add(self, rhs: (i64, i64)) -> Self::Output {
        Position {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl Solution for Day19 {
    fn solve_a(&self, input: &str) -> Answer {
        // println!("Map:\n{input}");
        let map: &Vec<Vec<char>> = &input.lines().map(|line| line.chars().collect()).collect();
        let mut pos = Position {
            x: map[0]
                .iter()
                .enumerate()
                .find(|(_, x)| **x == '|')
                .unwrap()
                .0 as i64,
            y: 0,
        };
        let mut dir = Direction::Down;
        let mut letters: String = String::new();

        // println!("Start: {pos:?}");

        loop {
            let next = pos.next_in_dir(dir);
            let symbol = next.get_symbol(map);
            // println!("Looking at {next:?} = '{symbol:?}'");

            match symbol {
                Some(' ') | None => {
                    // Need to change direction
                    if let Some(&(new_dir, next, symbol)) = dir
                        .orthogonal_directions()
                        .map(|d| (d, pos.next_in_dir(d)))
                        .map(|(d, p)| (d, p, p.get_symbol(map)))
                        .iter()
                        // .inspect(|x| println!("Check sides {x:?} with '{:?}'", x.1.get_symbol(map)))
                        .find(|(_, _, s)| s.map(|s| s != ' ').unwrap_or(false))
                    {
                        // println!("Now going {new_dir:?}");
                        pos = next;
                        dir = new_dir;

                        if let Some(s) = symbol {
                            push_letter(&mut letters, s);
                        }
                    } else {
                        break;
                    }
                }
                Some(c) => {
                    pos = next;
                    push_letter(&mut letters, c);
                }
            }
        }

        letters.into()
    }

    fn solve_b(&self, _input: &str) -> Answer {
        todo!()
    }
}

fn push_letter(letters: &mut String, c: char) {
    match c {
        '|' | '-' | '+' | ' ' => {}
        _ => letters.push(c),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "    |
    |  +--+
    A  |  C
F---|----E|--+
    |  |  |  D
    +B-+  +--+";

    #[test]
    fn test_a() {
        assert_eq!(Day19 {}.solve_a(INPUT), Answer::String("ABCDEF".to_owned()));
    }
}
