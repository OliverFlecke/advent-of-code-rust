use itertools::Itertools;

use crate::solutions::{Answer, Solution};

pub struct Day03 {}

impl Solution for Day03 {
    fn solve_a(&self, input: &str) -> Answer {
        let n: i64 = input.parse().unwrap();
        if n == 1 {
            return Answer::UInt(0);
        }

        let level = (1..)
            .step_by(2)
            .map(|x| x)
            .take_while(|x| *x * *x < n)
            .map(|x| x + 2)
            .last()
            .unwrap_or(1);

        (0..4)
            .map(|k| level * level - k * (level - 1))
            .map(|p| p.abs_diff(n) as i64)
            .filter(|dist| *dist <= ((level - 1) / 2))
            .map(|dist| level - 1 - dist)
            .map(|x| x as u64)
            .find_or_first(|_| true)
            .unwrap()
            .into()
    }

    fn solve_b(&self, _input: &str) -> Answer {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::{Answer, Solution};

    use super::Day03;

    #[test]
    fn test_a() {
        assert_eq!(Day03 {}.solve_a("1"), Answer::UInt(0));
        assert_eq!(Day03 {}.solve_a("9"), Answer::UInt(2));
        assert_eq!(Day03 {}.solve_a("23"), Answer::UInt(2));
        assert_eq!(Day03 {}.solve_a("12"), Answer::UInt(3));
        assert_eq!(Day03 {}.solve_a("1024"), Answer::UInt(31));
    }
}
