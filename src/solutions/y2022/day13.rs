use std::{cmp::Ordering, collections::VecDeque};

use crate::solutions::{answer::Answer, Solution};

pub struct Day13;

impl Solution for Day13 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        Some(
            parse(input)
                .iter()
                .enumerate()
                .filter(|(_, (l, r))| is_in_right_order(l, r))
                .map(|(i, _)| i + 1)
                .sum::<usize>()
                .into(),
        )
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        const DIVIDER_1: &str = "[[2]]";
        const DIVIDER_2: &str = "[[6]]";

        let pairs = parse(input);
        let packets = pairs.iter().flat_map(|(l, r)| [l, r]).collect::<Vec<_>>();

        let first = parse_packet(DIVIDER_1);
        let second = parse_packet(DIVIDER_2);

        Some(
            ((packets
                .iter()
                .filter(|p| is_in_right_order(p, &first))
                .count()
                + 1)
                * (packets
                    .iter()
                    .filter(|p| is_in_right_order(p, &second))
                    .count()
                    + 2))
                .into(),
        )
    }
}

fn parse(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .map(|pairs| {
            let mut ps = pairs.split('\n').map(parse_packet);
            (ps.next().unwrap(), ps.next().unwrap())
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Symbol {
    Left,
    Right,
    Value(u8),
}

type Packet = VecDeque<Symbol>;

fn is_in_right_order(left: &Packet, right: &Packet) -> bool {
    use Symbol::*;
    let mut left = left.to_owned();
    let mut right = right.to_owned();

    while !right.is_empty() {
        match left.pop_front() {
            Some(Left) => match right.pop_front().unwrap() {
                Left => (),
                Right => return false,
                Value(i) => {
                    right.push_front(Right);
                    right.push_front(Value(i));
                }
            },
            Some(Right) => match right.pop_front().unwrap() {
                Left | Value(_) => return true,
                Right => (),
            },
            Some(Value(l)) => match right.pop_front().unwrap() {
                Left => {
                    left.push_front(Right);
                    left.push_front(Value(l));
                }
                Right => return false,
                Value(r) => match l.cmp(&r) {
                    Ordering::Less => return true,
                    Ordering::Equal => (),
                    Ordering::Greater => return false,
                },
            },
            None => break,
        }
    }

    true
}

fn parse_packet(line: &str) -> Packet {
    let mut packet = VecDeque::with_capacity(line.len());
    let mut numbers = String::with_capacity(2);

    line.chars().for_each(|symbol| {
        if !symbol.is_ascii_digit() {
            if !numbers.is_empty() {
                packet.push_back(Symbol::Value(numbers.parse().unwrap()));
                numbers.clear();
            }

            match symbol {
                '[' => packet.push_back(Symbol::Left),
                ']' => packet.push_back(Symbol::Right),
                ',' => (),
                _ => unreachable!(),
            }
        } else {
            numbers.push(symbol);
        }
    });

    packet
}

#[cfg(test)]
mod test {
    use crate::{utils::load_sample, Year};

    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(
            Day13.solve_a(load_sample(Year::Y2022, "13.txt").unwrap().as_str()),
            Some(Answer::UInt(13))
        )
    }

    #[test]
    fn test_b() {
        assert_eq!(
            Day13.solve_b(load_sample(Year::Y2022, "13.txt").unwrap().as_str()),
            Some(Answer::UInt(140))
        )
    }
}
