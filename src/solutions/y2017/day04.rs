use std::collections::HashSet;

use crate::solutions::{answer::Answer, Solution};

pub struct Day04 {}

impl Solution for Day04 {
    fn solve_a(&self, input: &str) -> Answer {
        fn is_valid_passphrase(passphrase: &str) -> bool {
            let mut words: HashSet<&str> = HashSet::new();

            for word in passphrase.split(' ') {
                if words.contains(word) {
                    return false;
                }

                words.insert(word);
            }

            true
        }

        input
            .split('\n')
            .filter(|phrase| !phrase.is_empty())
            .filter(|phrase| is_valid_passphrase(&phrase))
            .count()
            .into()
    }

    fn solve_b(&self, _input: &str) -> Answer {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(Day04 {}.solve_a("aa bb cc dd ee"), Answer::UInt(1));
        assert_eq!(Day04 {}.solve_a("aa bb cc dd aa"), Answer::UInt(0));
        assert_eq!(Day04 {}.solve_a("aa bb cc dd aaa"), Answer::UInt(1));
    }
}
