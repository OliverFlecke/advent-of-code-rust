use std::{collections::HashSet, iter::FromIterator};

use itertools::Itertools;

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

    fn solve_b(&self, input: &str) -> Answer {
        fn is_valid_passphrase(phrase: &str) -> bool {
            let mut letter_sets = HashSet::new();

            for word in phrase.split(' ') {
                let letters = String::from_iter(word.chars().sorted());

                if letter_sets.contains(&letters) {
                    return false;
                }

                letter_sets.insert(letters);
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(Day04 {}.solve_a("aa bb cc dd ee"), Answer::UInt(1));
        assert_eq!(Day04 {}.solve_a("aa bb cc dd aa"), Answer::UInt(0));
        assert_eq!(Day04 {}.solve_a("aa bb cc dd aaa"), Answer::UInt(1));
    }

    #[test]
    fn test_b() {
        assert_eq!(Day04 {}.solve_b("abcde fghij"), Answer::UInt(1));
        assert_eq!(Day04 {}.solve_b("abcde xyz ecdab"), Answer::UInt(0));
        assert_eq!(Day04 {}.solve_b("a ab abc abd abf abj"), Answer::UInt(1));
        assert_eq!(
            Day04 {}.solve_b("iiii oiii ooii oooi oooo"),
            Answer::UInt(1)
        );
        assert_eq!(Day04 {}.solve_b("oiii ioii iioi iiio"), Answer::UInt(0));
    }

    #[test]
    fn hast_set_equality() {
        let mut a = HashSet::new();
        a.insert('a');
        a.insert('b');
        a.insert('c');
        let mut b = HashSet::new();
        b.insert('a');
        b.insert('b');
        b.insert('c');

        assert_eq!(a, b);
    }
}
