use crate::solutions::{answer::Answer, Solution};

pub struct Day09 {}

impl Solution for Day09 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let chars: Vec<char> = input.chars().collect();
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
                '!' if garbage => {
                    i += 1;
                }
                '<' => {
                    garbage = true;
                }
                '>' => {
                    garbage = false;
                }
                _ => {}
            }
            i += 1;
        }

        Some(score.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let chars: Vec<char> = input.chars().collect();
        let mut count: u64 = 0;
        let mut i = 0;
        let mut garbage = false;

        while i < chars.len() {
            match chars[i] {
                '!' if garbage => {
                    i += 1;
                }
                '<' if !garbage => {
                    garbage = true;
                }
                '>' => {
                    garbage = false;
                }
                _ if garbage => {
                    count += 1;
                }
                _ => {}
            }
            i += 1;
        }

        Some(count.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(Day09 {}.solve_a("{}"), Some(Answer::UInt(1)));
        assert_eq!(Day09 {}.solve_a("{{{}}}"), Some(Answer::UInt(6)));
        assert_eq!(Day09 {}.solve_a("{{},{}}"), Some(Answer::UInt(5)));
        assert_eq!(Day09 {}.solve_a("{{{},{},{{}}}}"), Some(Answer::UInt(16)));
        assert_eq!(Day09 {}.solve_a("{<a>,<a>,<a>,<a>}"), Some(Answer::UInt(1)));
        assert_eq!(
            Day09 {}.solve_a("{{<ab>},{<ab>},{<ab>},{<ab>}}"),
            Some(Answer::UInt(9)),
        );
        assert_eq!(
            Day09 {}.solve_a("{{<!!>},{<!!>},{<!!>},{<!!>}}"),
            Some(Answer::UInt(9)),
        );
        assert_eq!(
            Day09 {}.solve_a("{{<a!>},{<a!>},{<a!>},{<ab>}}"),
            Some(Answer::UInt(3)),
        );
    }

    #[test]
    fn test_b() {
        assert_eq!(Day09 {}.solve_b("<>"), Some(Answer::UInt(0)));
        assert_eq!(
            Day09 {}.solve_b("<random characters>"),
            Some(Answer::UInt(17))
        );
        assert_eq!(Day09 {}.solve_b("<<<<>"), Some(Answer::UInt(3)));
        assert_eq!(Day09 {}.solve_b("<{!>}>"), Some(Answer::UInt(2)));
        assert_eq!(Day09 {}.solve_b("<!!>"), Some(Answer::UInt(0)));
        assert_eq!(Day09 {}.solve_b("<!!!>>"), Some(Answer::UInt(0)));
        assert_eq!(Day09 {}.solve_b("<{o\"i!a,<{i<a>"), Some(Answer::UInt(10)));
    }
}
