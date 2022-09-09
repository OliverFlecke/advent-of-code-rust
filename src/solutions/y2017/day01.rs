use itertools::Itertools;

use crate::solutions::{Answer, Solution};

pub struct Day01 {}

impl Solution for Day01 {
    fn solve_a(&self, input: &str) -> Answer {
        fn compare(a: u32, b: u32) -> u32 {
            if a == b {
                a
            } else {
                0
            }
        }

        let (mut first, mut last) = (None::<u32>, None::<u32>);
        let sum = input
            .chars()
            .into_iter()
            .filter_map(|c| c.to_digit(10))
            .tuple_windows()
            .fold(0, |sum, (a, b)| {
                last = if first.is_some() { Some(b) } else { None };
                first = first.or(Some(a));

                sum + compare(a, b)
            })
            + (first
                .map(|a| last.map(|b| if a == b { a } else { 0 }).unwrap_or_default())
                .unwrap_or_default());

        sum.into()
    }

    fn solve_b(&self, input: &str) -> Answer {
        let digits = input
            .chars()
            .into_iter()
            .filter_map(|c| c.to_digit(10))
            .collect_vec();

        let mut sum = 0;
        for i in 0..digits.len() {
            if digits[i] == digits[(i + digits.len() / 2) % digits.len()] {
                sum += digits[i];
            }
        }

        sum.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(Day01 {}.solve_a("1122"), Answer::UInt(3));
        assert_eq!(Day01 {}.solve_a("1111"), Answer::UInt(4));
        assert_eq!(Day01 {}.solve_a("1234"), Answer::UInt(0));
        assert_eq!(Day01 {}.solve_a("91212129"), Answer::UInt(9));
    }

    #[test]
    fn test_b() {
        assert_eq!(Day01 {}.solve_b("1212"), Answer::UInt(6));
        assert_eq!(Day01 {}.solve_b("1221"), Answer::UInt(0));
        assert_eq!(Day01 {}.solve_b("123425"), Answer::UInt(4));
        assert_eq!(Day01 {}.solve_b("123123"), Answer::UInt(12));
        assert_eq!(Day01 {}.solve_b("12131415"), Answer::UInt(4));
    }
}
