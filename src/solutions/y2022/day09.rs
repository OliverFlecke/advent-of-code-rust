use std::collections::HashSet;

use crate::solutions::{answer::Answer, Solution};

pub struct Day09;

impl Solution for Day09 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let moves = parse(input);
        let mut visited: Visited = HashSet::new();
        visited.insert((0, 0));

        let mut head = (0, 0);
        let mut tail = (0, 0);
        moves.iter().for_each(|m| {
            m.perform(&mut head, &mut tail, &mut visited);
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

    fn solve_b(&self, _input: &str) -> Option<Answer> {
        None
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

type Location = (isize, isize);

// fn manhattan_distance(a: &Location, b: &Location) -> usize {
//     a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
// }

fn close_enough(a: &Location, b: &Location) -> bool {
    a.0.abs_diff(b.0) <= 1 && a.1.abs_diff(b.1) <= 1
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl Move {
    fn perform(&self, head: &mut Location, tail: &mut Location, visited: &mut Visited) {
        match self {
            Move::Up(times) => {
                for _ in 0..*times {
                    head.1 += 1;
                    if !close_enough(head, tail) {
                        *tail = (head.0, head.1 - 1);
                        visited.insert(*tail);
                    }
                }
            }
            Move::Down(times) => {
                for _ in 0..*times {
                    head.1 -= 1;
                    if !close_enough(head, tail) {
                        *tail = (head.0, head.1 + 1);
                        visited.insert(*tail);
                    }
                }
            }
            Move::Left(times) => {
                for _ in 0..*times {
                    head.0 -= 1;
                    if !close_enough(head, tail) {
                        *tail = (head.0 + 1, head.1);
                        visited.insert(*tail);
                    }
                }
            }
            Move::Right(times) => {
                for _ in 0..*times {
                    head.0 += 1;
                    if !close_enough(head, tail) {
                        *tail = (head.0 - 1, head.1);
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

    #[test]
    fn test() {
        assert_eq!(Day09.solve_a(SAMPLE_INPUT), Some(Answer::UInt(13)))
    }
}
