use lazy_static::lazy_static;
use regex::Regex;

use crate::solutions::{answer::Answer, Solution};

pub struct Day02;

impl Solution for Day02 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let limits = Set {
            red: 12,
            green: 13,
            blue: 14,
        };

        let answer = input
            .trim()
            .lines()
            .filter_map(parse_game)
            .filter(|(_, sets)| sets.iter().all(|s| s.is_within(&limits)))
            .map(|(id, _)| id)
            .sum::<usize>();

        Some(answer.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let answer = input
            .trim()
            .lines()
            .filter_map(parse_game)
            .filter_map(|(_, sets)| {
                sets.into_iter()
                    .reduce(|prev, current| prev.minimum_containing_set(&current))
            })
            .map(|set| set.power())
            .sum::<usize>();

        Some(answer.into())
    }
}

type Game = (usize, Vec<Set>);

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?<count>\d+) (?<color>blue|red|green)").unwrap();
}

fn parse_game(line: &str) -> Option<Game> {
    let Some((head, sets)) = line.split_once(':') else {
        return None;
    };

    let id = head
        .strip_prefix("Game ")
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap();

    let sets = sets
        .split(';')
        .map(|s| {
            RE.captures_iter(s)
                .map(|m| (m["color"].to_string(), m["count"].parse::<usize>().unwrap()))
                .fold(Set::default(), |set, (color, count)| match color.as_str() {
                    "blue" => Set { blue: count, ..set },
                    "red" => Set { red: count, ..set },
                    "green" => Set {
                        green: count,
                        ..set
                    },
                    _ => unreachable!(),
                })
        })
        .collect::<Vec<_>>();

    Some((id, sets))
}

#[derive(Debug, Default)]
struct Set {
    red: usize,
    blue: usize,
    green: usize,
}

impl Set {
    fn is_within(&self, other: &Set) -> bool {
        self.red <= other.red && self.blue <= other.blue && self.green <= other.green
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }

    fn minimum_containing_set(&self, other: &Self) -> Self {
        Set {
            red: self.red.max(other.red),
            blue: self.blue.max(other.blue),
            green: self.green.max(other.green),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{client::AocClient, Year};

    use super::*;

    const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_a() {
        assert_eq!(Day02 {}.solve_a(INPUT), Some(Answer::UInt(8)));
    }

    #[test]
    fn solve_a() {
        let input = AocClient::default()
            .get_input((Year::Y2023, 2).into())
            .unwrap();
        assert_eq!(Day02 {}.solve_a(&input), Some(Answer::UInt(2795)));
    }

    #[test]
    fn test_b() {
        assert_eq!(Day02 {}.solve_b(INPUT), Some(Answer::UInt(2286)));
    }

    #[test]
    fn solve_b() {
        let input = AocClient::default()
            .get_input((Year::Y2023, 2).into())
            .unwrap();
        assert_eq!(Day02 {}.solve_b(&input), Some(Answer::UInt(75561)));
    }
}
