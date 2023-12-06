use std::collections::HashSet;

use itertools::Itertools;
use regex::Regex;

use crate::solutions::{answer::Answer, Solution};

pub struct Day03;

impl Solution for Day03 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let numbers = find_numbers(input);
        let symbols = find_symbols(input, r"[^\w.]");

        let answer = numbers
            .iter()
            .filter(|num| adjencent(num).iter().any(|v| symbols.contains(v)))
            .map(|num| num.value)
            .sum::<usize>();

        Some(answer.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let symbols = find_symbols(input, r"\*");
        let numbers = find_numbers(input);

        let answer = symbols
            .iter()
            .filter_map(|s| {
                let numbers = numbers
                    .iter()
                    .filter(|num| adjencent(num).iter().any(|v| v == s))
                    .map(|num| num.value)
                    .collect::<Vec<_>>();
                if numbers.len() == 2 {
                    Some(numbers[0] * numbers[1])
                } else {
                    None
                }
            })
            .sum::<usize>();

        Some(answer.into())
    }
}

fn find_symbols(input: &str, re: &str) -> HashSet<(usize, usize)> {
    let re_symbols = Regex::new(re).unwrap();
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            re_symbols
                .find_iter(l)
                .map(|m| (y, m.start()))
                .collect::<Vec<_>>()
        })
        .collect()
}

struct Number {
    value: usize,
    y: usize,
    x: usize,
    length: usize,
}

fn find_numbers(input: &str) -> Vec<Number> {
    let re_numbers = Regex::new(r"\d+").unwrap();
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            re_numbers
                .find_iter(l)
                .map(|m| Number {
                    value: m.as_str().parse::<usize>().unwrap(),
                    y,
                    x: m.start(),
                    length: m.len(),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}
fn adjencent(number: &Number) -> Vec<(usize, usize)> {
    (number.y.saturating_sub(1)..=number.y + 1)
        .cartesian_product(number.x.saturating_sub(1)..=number.x + number.length)
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{client::get_input, Year};

    use super::*;
    const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_a() {
        assert_eq!(Day03 {}.solve_a(INPUT), Some(Answer::UInt(4361)))
    }

    #[test]
    fn test_b() {
        assert_eq!(Day03 {}.solve_b(INPUT), Some(Answer::UInt(467835)))
    }

    #[test]
    fn answer_a() {
        let input = get_input(Year::Y2023, 3).unwrap();
        assert_eq!(Day03 {}.solve_a(input.as_str()), Some(Answer::UInt(509115)));
    }

    #[test]
    fn answer_b() {
        let input = get_input(Year::Y2023, 3).unwrap();
        assert_eq!(
            Day03 {}.solve_b(input.as_str()),
            Some(Answer::UInt(75220503))
        );
    }
}
