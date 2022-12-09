use std::collections::HashSet;

use crate::solutions::{answer::Answer, Solution};

pub struct Day09;

impl Solution for Day09 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        Some(simulate(parse(input), 2).into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        Some(simulate(parse(input), 10).into())
    }
}

fn simulate(moves: Vec<(Move, u32)>, number_of_knots: usize) -> usize {
    moves
        .iter()
        .fold(
            (
                HashSet::<Location>::with_capacity(6_500),
                vec![Location::default(); number_of_knots],
            ),
            |(mut visited, mut knots), (m, amount)| {
                for _ in 0..*amount {
                    knots.first_mut().unwrap().perform_mut(m);
                    for i in 1..number_of_knots {
                        knots[i] = knots[i].follow(&knots[i - 1]);
                    }
                    visited.insert(*knots.last().unwrap());
                }
                (visited, knots)
            },
        )
        .0
        .len()
}

fn parse(input: &str) -> Vec<(Move, u32)> {
    input
        .trim_end()
        .lines()
        .map(|l| {
            let mut split = l.split(' ');

            let m = split
                .next()
                .and_then(|x| Move::try_from(x.chars().next().unwrap()).ok())
                .expect("line to be move");
            let amount = split
                .next()
                .and_then(|x| x.parse::<u32>().ok())
                .expect("to be number");
            (m, amount)
        })
        .collect()
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    x: isize,
    y: isize,
}

impl Location {
    fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn perform(&self, m: &Move) -> Self {
        match m {
            Move::Up => self.up(),
            Move::Down => self.down(),
            Move::Left => self.left(),
            Move::Right => self.right(),
        }
    }

    fn perform_mut(&mut self, m: &Move) {
        *self = self.perform(m);
    }

    fn follow(self, pos: &Self) -> Self {
        if pos.x == self.x && pos.y > self.y + 1 {
            self.down()
        } else if pos.x == self.x && pos.y < self.y - 1 {
            self.up()
        } else if pos.x > self.x + 1 && pos.y == self.y {
            self.right()
        } else if pos.x < self.x - 1 && pos.y == self.y {
            self.left()
        } else if (pos.x - self.x).abs() == 1 && (pos.y - self.y).abs() == 1 {
            self.clone()
        } else if pos.x < self.x && pos.y < self.y {
            self.left().up()
        } else if pos.x > self.x && pos.y < self.y {
            self.right().up()
        } else if pos.x > self.x && pos.y > self.y {
            self.right().down()
        } else if pos.x < self.x && pos.y > self.y {
            self.left().down()
        } else {
            self.clone()
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Move {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Move::Right),
            'L' => Ok(Move::Left),
            'U' => Ok(Move::Up),
            'D' => Ok(Move::Down),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_a() {
        assert_eq!(Day09.solve_a(SAMPLE_INPUT), Some(Answer::UInt(13)))
    }

    #[test]
    fn test_b() {
        assert_eq!(Day09.solve_b(SAMPLE_INPUT), Some(Answer::UInt(1)));
        assert_eq!(
            Day09.solve_b(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            ),
            Some(Answer::UInt(36))
        );
    }
}
