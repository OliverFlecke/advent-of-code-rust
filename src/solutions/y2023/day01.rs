use regex::Regex;

use crate::solutions::{answer::Answer, Solution};

pub struct Day01;

impl Solution for Day01 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let re = Regex::new(r"\d").unwrap();

        let answer: usize = input
            .trim()
            .lines()
            .map(|line| {
                let mut it = re
                    .find_iter(line)
                    .filter_map(|x| x.as_str().parse::<usize>().ok());
                let first = it.next().unwrap();
                let last = it.last().unwrap_or(first);
                (first, last)
            })
            .map(|(first, last)| first * 10 + last)
            .sum();

        Some(answer.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let Ok(re) = Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)") else {
            return None;
        };

        fn to_number(value: &str) -> usize {
            value
                .parse()
                .ok()
                .or(match value {
                    "one" => Some(1),
                    "two" => Some(2),
                    "three" => Some(3),
                    "four" => Some(4),
                    "five" => Some(5),
                    "six" => Some(6),
                    "seven" => Some(7),
                    "eight" => Some(8),
                    "nine" => Some(9),
                    _ => None,
                })
                .unwrap()
        }

        let answer: usize = input
            .trim()
            .lines()
            .map(|line| {
                let mut start = 0;
                let mut first = None;
                let mut last = None;
                while let Some(m) = re.find_at(line, start) {
                    if first.is_none() {
                        first = Some(m.as_str());
                    }

                    start = m.start() + 1;
                    last = Some(m.as_str());
                }

                (to_number(first.unwrap()), to_number(last.unwrap()))
            })
            .map(|(first, last)| first * 10 + last)
            .sum();

        Some(answer.into())
    }
}

#[cfg(test)]
mod test {
    use crate::{client::AocClient, Year};

    use super::*;

    #[test]
    fn test_a() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(Day01 {}.solve_a(input), Some(Answer::UInt(142)));
    }

    #[test]
    fn solve_a() {
        let input = AocClient::default().get_input(Year::Y2023, 1).unwrap();
        assert_eq!(Day01 {}.solve_a(&input), Some(Answer::UInt(54916)));
    }

    #[test]
    fn test_b() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        assert_eq!(Day01 {}.solve_b(input), Some(Answer::UInt(281)));
    }

    #[test]
    fn solve_b() {
        let input = AocClient::default().get_input(Year::Y2023, 1).unwrap();
        assert_eq!(Day01 {}.solve_b(&input), Some(Answer::UInt(54728)));
    }
}
