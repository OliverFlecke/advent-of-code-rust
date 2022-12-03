use std::collections::HashSet;

use crate::solutions::{answer::Answer, Solution};

pub struct Day03;

impl Solution for Day03 {
    fn solve_a(&self, input: &str) -> Answer {
        input
            .lines()
            .map(|l| l.split_at(l.len() / 2))
            .map(|(a, b)| {
                (
                    a.chars().collect::<HashSet<_>>(),
                    b.chars().collect::<HashSet<_>>(),
                )
            })
            .map(|(a, b)| a.intersection(&b).cloned().collect::<HashSet<_>>())
            .map(|set| {
                debug_assert_eq!(set.len(), 1);
                set.iter().next().unwrap().clone()
            })
            .map(|c| char_to_priority(c) as u64)
            .sum::<u64>()
            .into()
    }

    fn solve_b(&self, _input: &str) -> Answer {
        todo!()
    }
}

fn char_to_priority(c: char) -> u8 {
    if c.is_uppercase() {
        c as u8 - ('A' as u8) + 27
    } else {
        c as u8 - ('a' as u8) + 1
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
        assert_eq!(Day03 {}.solve_a(SAMPLE_INPUT), Answer::UInt(157));
    }
}
