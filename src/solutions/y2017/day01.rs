use itertools::Itertools;

use crate::solutions::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    fn solve_a(&self, input: &str) -> String {
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

        sum.to_string()
    }

    fn solve_b(&self, input: &str) -> String {
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

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(Day01 {}.solve_a("1122"), "3");
        assert_eq!(Day01 {}.solve_a("1111"), "4");
        assert_eq!(Day01 {}.solve_a("1234"), "0");
        assert_eq!(Day01 {}.solve_a("91212129"), "9");
    }

    #[test]
    fn test_b() {
        assert_eq!(Day01 {}.solve_b("1212"), "6");
        assert_eq!(Day01 {}.solve_b("1221"), "0");
        assert_eq!(Day01 {}.solve_b("123425"), "4");
        assert_eq!(Day01 {}.solve_b("123123"), "12");
        assert_eq!(Day01 {}.solve_b("12131415"), "4");
    }
}
