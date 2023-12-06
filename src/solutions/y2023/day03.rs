use itertools::Itertools;
use regex::Regex;

use crate::solutions::{answer::Answer, Solution};

pub struct Day03;

impl Solution for Day03 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let re_numbers = Regex::new(r"\d+").unwrap();
        let re_symbols = Regex::new(r"[^\w.]").unwrap();
        let numbers = input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                re_numbers
                    .find_iter(l)
                    .map(|m| (m.as_str(), y, m.start(), m.len()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let symbols: Vec<(usize, usize)> = input
            .trim()
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                re_symbols
                    .find_iter(l)
                    .map(|m| (y, m.start()))
                    .collect::<Vec<_>>()
            })
            .collect();

        let adjencent = |x: usize, y: usize, length: usize| -> Vec<(usize, usize)> {
            (y.saturating_sub(1)..=y + 1)
                .cartesian_product(x.saturating_sub(1)..=x + length)
                .collect()
        };

        let answer = numbers
            .iter()
            .filter(|x| adjencent(x.2, x.1, x.3).iter().any(|v| symbols.contains(v)))
            .map(|x| x.0.parse::<usize>().unwrap())
            .sum::<usize>();

        Some(answer.into())
    }

    fn solve_b(&self, _input: &str) -> Option<Answer> {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::{client::get_input, Year};

    use super::*;

    #[test]
    fn test_a() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        assert_eq!(Day03 {}.solve_a(input), Some(Answer::UInt(4361)))
    }

    #[test]
    fn answer_a() {
        let input = get_input(Year::Y2023, 3).unwrap();
        assert_eq!(Day03 {}.solve_a(input.as_str()), Some(Answer::UInt(509115)))
    }
}
