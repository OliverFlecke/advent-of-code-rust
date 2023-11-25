use std::collections::HashSet;

use crate::solutions::{answer::Answer, Solution};

pub struct Day03;

impl Solution for Day03 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        Some(
            input
                .lines()
                .map(|l| l.split_at(l.len() / 2))
                .map(|(a, b)| {
                    (
                        a.chars().collect::<HashSet<_>>(),
                        b.chars().collect::<HashSet<_>>(),
                    )
                })
                .map(|(a, b)| *a.intersection(&b).next().unwrap())
                .map(|c| char_to_priority(c) as u64)
                .sum::<u64>()
                .into(),
        )
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        Some(
            input
                .lines()
                .collect::<Vec<_>>()
                .as_slice()
                .chunks(3)
                .map(|chunk| {
                    chunk
                        .iter()
                        .map(|l| l.chars().collect::<HashSet<_>>())
                        .reduce(|acc, item| {
                            acc.intersection(&item).cloned().collect::<HashSet<_>>()
                        })
                        .map(|set| *set.iter().next().unwrap())
                        .map(|c| char_to_priority(c) as u64)
                        .unwrap()
                })
                .sum::<u64>()
                .into(),
        )
    }
}

fn char_to_priority(c: char) -> u8 {
    if c.is_uppercase() {
        c as u8 - b'A' + 27
    } else {
        c as u8 - b'a' + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn priority() {
        assert_eq!(char_to_priority('a'), 1);
        assert_eq!(char_to_priority('z'), 26);
        assert_eq!(char_to_priority('A'), 27);
        assert_eq!(char_to_priority('Z'), 52);
    }

    #[test]
    fn test_a() {
        assert_eq!(Day03 {}.solve_a(SAMPLE_INPUT), Some(Answer::UInt(157)));
    }

    #[test]
    fn test_b() {
        assert_eq!(Day03 {}.solve_b(SAMPLE_INPUT), Some(Answer::UInt(70)))
    }
}
