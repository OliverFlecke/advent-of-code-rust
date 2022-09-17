use itertools::Itertools;

use crate::solutions::{answer::Answer, Solution};

pub struct Day09 {}

impl Solution for Day09 {
    fn solve_a(&self, input: &str) -> Answer {
        let chars = input.chars().collect_vec();
        let mut score: u64 = 0;
        let mut depth: u64 = 0;
        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                '{' => {
                    depth += 1;
                }
                '}' => {
                    score += depth;
                    depth -= 1;
                }
                '<' => {
                    let mut next_negated = false;
                    while i < chars.len() && (chars[i] != '>' || next_negated) {
                        next_negated = !next_negated && chars[i] == '!';
                        i += 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        score.into()
    }

    fn solve_b(&self, _input: &str) -> Answer {
        todo!()
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
}
