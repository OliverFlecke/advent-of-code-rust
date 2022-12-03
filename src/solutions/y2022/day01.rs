use itertools::Itertools;

use crate::solutions::{answer::Answer, Solution};

pub struct Day01;

impl Solution for Day01 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        Some(
            input
                .trim_end()
                .split("\n\n")
                .map(|elf| elf.lines().map(|s| s.parse::<u32>().unwrap()).sum::<u32>())
                .max()
                .unwrap()
                .into(),
        )
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        Some(
            input
                .trim_end()
                .split("\n\n")
                .map(|elf| elf.lines().map(|s| s.parse::<u32>().unwrap()).sum::<u32>())
                .sorted()
                .rev()
                .take(3)
                .sum::<u32>()
                .into(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_a() {
        assert_eq!(Day01 {}.solve_a(SAMPLE_INPUT), Some(Answer::UInt(24000)))
    }

    #[test]
    fn test_b() {
        assert_eq!(Day01 {}.solve_b(SAMPLE_INPUT), Some(Answer::UInt(45000)))
    }
}
