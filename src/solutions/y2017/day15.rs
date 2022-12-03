use std::{convert::TryInto, iter::Scan, ops::Range};

use regex::Regex;

use crate::solutions::{answer::Answer, Solution};

pub struct Day15 {}

impl Solution for Day15 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let size = 40_000_000;
        let start = Self::parse(input);

        Some(
            Self::higher_order_count_matches(
                start,
                size,
                |a| Self::generate_next(a, Self::A_FACTOR),
                |b| Self::generate_next(b, Self::B_FACTOR),
            )
            .into(),
        )
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let size = 5_000_000;
        let start = Self::parse(input);

        Some(
            Self::higher_order_count_matches(
                start,
                size,
                |a| Self::generate_next_b(a, Self::A_FACTOR, 4),
                |b| Self::generate_next_b(b, Self::B_FACTOR, 8),
            )
            .into(),
        )
    }
}

type GeneratorSequence = Scan<Range<u32>, Pair, fn(&mut Pair, u32) -> Option<Pair>>;

#[allow(dead_code)]
impl Day15 {
    const A_FACTOR: u32 = 16807;
    const B_FACTOR: u32 = 48271;

    fn parse(input: &str) -> (u32, u32) {
        let re = Regex::new(r"(?P<value>\d+)").expect("is hardcoded to a valid regex");
        let get_start_value = |line: &str| -> u32 {
            match re.captures(line) {
                Some(captures) => captures["value"].parse().expect("value should be a number"),
                None => panic!("Regex could not be matched"),
            }
        };

        let mut lines = input.lines();

        (
            get_start_value(lines.next().unwrap()),
            get_start_value(lines.next().unwrap()),
        )
    }

    fn generate_seq(initial_state: Pair, size: u32) -> GeneratorSequence {
        (0..size).into_iter().scan(initial_state, |state, _| {
            *state = (
                Self::generate_next(state.0, Self::A_FACTOR),
                Self::generate_next(state.1, Self::B_FACTOR),
            );

            Some(*state)
        })
    }

    fn generate_next(current: u32, factor: u32) -> u32 {
        ((current as u64) * (factor as u64) % (i32::MAX as u64))
            .try_into()
            .unwrap()
    }

    fn lower_bits_matches(a: u32, b: u32) -> bool {
        let mask: u32 = u16::MAX as u32;
        let a_lower = (a & mask) as u16;
        let b_lower = (b & mask) as u16;

        a_lower == b_lower
    }

    fn count_matches(initial: (u32, u32), size: u32) -> usize {
        Self::generate_seq(initial, size)
            .filter(|(a, b)| Self::lower_bits_matches(*a, *b))
            .count()
    }

    fn generate_seq_b(initial_state: Pair, size: u32) -> GeneratorSequence {
        (0..size).into_iter().scan(initial_state, |state, _| {
            *state = (
                Self::generate_next_b(state.0, Self::A_FACTOR, 4),
                Self::generate_next_b(state.1, Self::B_FACTOR, 8),
            );

            Some(*state)
        })
    }

    fn generate_next_b(current: u32, factor: u32, criteria: u32) -> u32 {
        let mut next = Self::generate_next(current, factor);
        while next % criteria != 0 {
            next = Self::generate_next(next, factor);
        }

        next
    }

    fn count_matches_b(initial: (u32, u32), size: u32) -> usize {
        Self::generate_seq_b(initial, size)
            .filter(|(a, b)| Self::lower_bits_matches(*a, *b))
            .count()
    }

    /// This is a refactor of the above code to allow for higher order functions
    /// to be passed for calculating the sequence. When splitting this into
    /// smaller methods, it causes issues with capture of variables when calling
    /// the passed function pointer.
    fn higher_order_count_matches(
        initial_state: Pair,
        size: u32,
        next_a: fn(u32) -> u32,
        next_b: fn(u32) -> u32,
    ) -> usize {
        (0..size)
            .into_iter()
            .scan(initial_state, |state, _| {
                *state = (next_a(state.0), next_b(state.1));

                Some(*state)
            })
            .filter(|(a, b)| Self::lower_bits_matches(*a, *b))
            .count()
    }
}

type Pair = (u32, u32);

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Generator A starts with 65
Generator B starts with 8921";

    #[test]
    fn parsing() {
        assert_eq!(Day15::parse(INPUT), (65, 8921));
    }

    #[test]
    fn generate_sequence() {
        let mut x = Day15::generate_seq((65, 8921), 5);
        assert_eq!(x.next().unwrap(), (1092455, 430625591));
        assert_eq!(x.next().unwrap(), (1181022009, 1233683848));
        assert_eq!(x.next().unwrap(), (245556042, 1431495498));
        assert_eq!(x.next().unwrap(), (1744312007, 137874439));
        assert_eq!(x.next().unwrap(), (1352636452, 285222916));
    }

    #[test]
    fn count_matches() {
        assert_eq!(Day15::count_matches((65, 8921), 5), 1);
    }

    #[test]
    fn test_a() {
        assert_eq!(Day15 {}.solve_a(INPUT), Some(Answer::UInt(588)))
    }

    #[test]
    fn generate_sequence_b() {
        let mut x = Day15::generate_seq_b((65, 8921), 5);

        assert_eq!(x.next().unwrap(), (1352636452, 1233683848));
        assert_eq!(x.next().unwrap(), (1992081072, 862516352));
        assert_eq!(x.next().unwrap(), (530830436, 1159784568));
        assert_eq!(x.next().unwrap(), (1980017072, 1616057672));
        assert_eq!(x.next().unwrap(), (740335192, 412269392));
    }

    #[test]
    fn count_matches_b() {
        assert_eq!(Day15::count_matches_b((65, 8921), 1056), 1);
    }

    #[test]
    fn test_b() {
        assert_eq!(Day15 {}.solve_b(INPUT), Some(Answer::UInt(309)))
    }
}
