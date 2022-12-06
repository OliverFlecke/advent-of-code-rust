use itertools::Itertools;

use crate::solutions::{answer::Answer, Solution};

pub struct Day06;

impl Solution for Day06 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let input = input.trim_end();

        for i in 0..input.len() - 4 {
            if is_unique(&input[i..i + 4]) {
                return Some((i + 4).into());
            }
        }

        None
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let input = input.trim_end();
        const DIST: usize = 14;

        for i in 0..input.len() - DIST {
            if is_unique(&input[i..i + DIST]) {
                return Some((i + DIST).into());
            }
        }

        None
    }
}

fn is_unique(input: &str) -> bool {
    input.chars().all_unique()
    // for i in 0..input.len() - 1 {
    //     if input[i] == input[i + 1] {
    //         return false
    //     }
    // }

    // true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(
            Day06.solve_a("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            Some(Answer::UInt(7))
        );
        assert_eq!(
            Day06.solve_a("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            Some(Answer::UInt(5))
        );
        assert_eq!(
            Day06.solve_a("nppdvjthqldpwncqszvftbrmjlhg"),
            Some(Answer::UInt(6))
        );
        assert_eq!(
            Day06.solve_a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            Some(Answer::UInt(10))
        );
        assert_eq!(
            Day06.solve_a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            Some(Answer::UInt(11))
        );
    }

    #[test]
    fn test_b() {
        assert_eq!(
            Day06.solve_b("mjqjpqmgbljsphdztnvjfqwrcgsmlb"),
            Some(Answer::UInt(19))
        );
        assert_eq!(
            Day06.solve_b("bvwbjplbgvbhsrlpgdmjqwftvncz"),
            Some(Answer::UInt(23))
        );
        assert_eq!(
            Day06.solve_b("nppdvjthqldpwncqszvftbrmjlhg"),
            Some(Answer::UInt(23))
        );
        assert_eq!(
            Day06.solve_b("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"),
            Some(Answer::UInt(29))
        );
        assert_eq!(
            Day06.solve_b("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"),
            Some(Answer::UInt(26))
        );
    }
}
