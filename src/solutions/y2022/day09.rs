#![allow(dead_code)]

use std::collections::HashSet;

use crate::solutions::{answer::Answer, Solution};

pub struct Day09;

impl Solution for Day09 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let moves = parse(input);
        let mut visited: Visited = HashSet::new();
        visited.insert(Location::default());

        let mut head = Location::default();
        let mut tail = Location::default();
        moves.iter().for_each(|m| {
            match m {
                Move::Up(_) => head = head.up(),
                Move::Down(_) => head = head.down(),
                Move::Left(_) => head = head.left(),
                Move::Right(_) => head = head.right(),
            };
            // m.perform(&mut head, &mut tail, &mut visited);
            // println!("{m:?}  \tHead: {:?}. Tail: {:?}", head, tail);
        });

        // for row in (0..5).rev() {
        //     for col in 0..5 {
        //         if visited.contains(&(col, row)) {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }

        Some(visited.len().into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let moves = parse(input);
        let mut visited: Visited = HashSet::new();
        visited.insert(Location::default());

        const KNOTS_COUNT: usize = 10;
        let mut knots = [Location::default(); KNOTS_COUNT];
        moves.iter().for_each(|m| {
            for _ in 0..m.get_amount() {
                match m {
                    Move::Up(_) => knots[0] = knots[0].up(),
                    Move::Down(_) => knots[0] = knots[0].down(),
                    Move::Left(_) => knots[0] = knots[0].left(),
                    Move::Right(_) => knots[0] = knots[0].right(),
                };

                for i in 1..KNOTS_COUNT {
                    knots[i] = knots[i].follow(&knots[i - 1]);
                }
                visited.insert(knots[9]);
            }

            // println!("{m:?}  \tHead: {:?}. Tail: {:?}", knots[0], knots[9]);
        });

        Some(visited.len().into())
    }
}

fn parse(input: &str) -> Vec<Move> {
    input
        .trim_end()
        .lines()
        .map(|l| Move::try_from(l).expect("line to be move"))
        .collect()
}

type Visited = HashSet<Location>;

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

// fn manhattan_distance(a: &Location, b: &Location) -> usize {
//     a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
// }

fn close_enough(a: &Location, b: &Location) -> bool {
    a.x.abs_diff(b.x) <= 1 && a.y.abs_diff(b.y) <= 1
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl Move {
    fn get_amount(&self) -> usize {
        match self {
            Move::Up(x) => *x,
            Move::Down(x) => *x,
            Move::Left(x) => *x,
            Move::Right(x) => *x,
        }
    }

    fn do_move(&self, head: &mut Location) {
        match self {
            Move::Up(_) => head.y += 1,
            Move::Down(_) => head.y -= 1,
            Move::Left(_) => head.x -= 1,
            Move::Right(_) => head.x += 1,
        }
    }

    fn perform(&self, head: &mut Location, tail: &mut Location, visited: &mut Visited) {
        match self {
            Move::Up(times) => {
                for _ in 0..*times {
                    head.y += 1;
                    if !close_enough(head, tail) {
                        tail.y -= 1;
                        visited.insert(*tail);
                    }
                }
            }
            Move::Down(times) => {
                for _ in 0..*times {
                    head.y -= 1;
                    if !close_enough(head, tail) {
                        tail.y += 1;
                        visited.insert(*tail);
                    }
                }
            }
            Move::Left(times) => {
                for _ in 0..*times {
                    head.x -= 1;
                    if !close_enough(head, tail) {
                        tail.x += 1;
                        visited.insert(*tail);
                    }
                }
            }
            Move::Right(times) => {
                for _ in 0..*times {
                    head.x += 1;
                    if !close_enough(head, tail) {
                        tail.x -= 1;
                        visited.insert(*tail);
                    }
                }
            }
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split(' ');
        match split.next().unwrap().chars().next() {
            Some('R') => Ok(Move::Right(
                split.next().and_then(|x| x.parse::<usize>().ok()).unwrap(),
            )),
            Some('L') => Ok(Move::Left(
                split.next().and_then(|x| x.parse::<usize>().ok()).unwrap(),
            )),
            Some('D') => Ok(Move::Down(
                split.next().and_then(|x| x.parse::<usize>().ok()).unwrap(),
            )),
            Some('U') => Ok(Move::Up(
                split.next().and_then(|x| x.parse::<usize>().ok()).unwrap(),
            )),
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

    // #[test]
    // fn test_a() {
    //     assert_eq!(Day09.solve_a(SAMPLE_INPUT), Some(Answer::UInt(13)))
    // }

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
