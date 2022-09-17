use itertools::Itertools;

use crate::solutions::{answer::Answer, Solution};

pub struct Day09 {}

impl Solution for Day09 {
    fn solve_a(&self, input: &str) -> Answer {
        let chars = input.chars().collect_vec();
        let mut score: u64 = 0;
        let mut depth: u64 = 0;
        let mut i = 0;
        let mut garbage = false;

        while i < chars.len() {
            match chars[i] {
                '{' if !garbage => {
                    depth += 1;
                }
                '}' if !garbage => {
                    score += depth;
                    depth -= 1;
                }
                '<' => {
                    garbage = true;
                }
                '>' => {
                    garbage = false;
                }
                '!' => {
                    if garbage {
                        i += 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        score.into()
    }

    fn solve_b(&self, input: &str) -> Answer {
        let chars = input.chars().collect_vec();
        let mut count: u64 = 0;
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                '{' => {}
                '}' => {}
                '<' => {
                    i += 1;
                    let mut amount = 0;

                    while i < chars.len() && chars[i] != '>' {
                        if chars[i] == '!' {
                            i += 1;
                        } else {
                            amount += 1;
                        }
                        i += 1;
                    }

                    count += amount;
                }
                _ => {}
            }
            i += 1;
        }

        count.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(Day09 {}.solve_a("{}"), Answer::UInt(1));
        assert_eq!(Day09 {}.solve_a("{{{}}}"), Answer::UInt(6));
        assert_eq!(Day09 {}.solve_a("{{},{}}"), Answer::UInt(5));
        assert_eq!(Day09 {}.solve_a("{{{},{},{{}}}}"), Answer::UInt(16));
        assert_eq!(Day09 {}.solve_a("{<a>,<a>,<a>,<a>}"), Answer::UInt(1));
        assert_eq!(
            Day09 {}.solve_a("{{<ab>},{<ab>},{<ab>},{<ab>}}"),
            Answer::UInt(9),
        );
        assert_eq!(
            Day09 {}.solve_a("{{<!!>},{<!!>},{<!!>},{<!!>}}"),
            Answer::UInt(9),
        );
        assert_eq!(
            Day09 {}.solve_a("{{<a!>},{<a!>},{<a!>},{<ab>}}"),
            Answer::UInt(3),
        );
    }

    #[test]
    fn test_b() {
        assert_eq!(Day09 {}.solve_b("<>"), Answer::UInt(0));
        assert_eq!(Day09 {}.solve_b("<random characters>"), Answer::UInt(17));
        assert_eq!(Day09 {}.solve_b("<<<<>"), Answer::UInt(3));
        assert_eq!(Day09 {}.solve_b("<{!>}>"), Answer::UInt(2));
        assert_eq!(Day09 {}.solve_b("<!!>"), Answer::UInt(0));
        assert_eq!(Day09 {}.solve_b("<!!!>>"), Answer::UInt(0));
        assert_eq!(Day09 {}.solve_b("<{o\"i!a,<{i<a>"), Answer::UInt(10));
    }
}
