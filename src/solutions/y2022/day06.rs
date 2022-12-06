use itertools::Itertools;

use crate::solutions::{answer::Answer, Solution};

pub struct Day06;

impl Solution for Day06 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        find_first_unique(input.trim_end(), 4).map(Answer::from)
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        find_first_unique(input.trim_end(), 14).map(Answer::from)
    }
}

fn find_first_unique(input: &str, len: usize) -> Option<usize> {
    for i in 0..input.len() - len {
        if input[i..i + len].chars().all_unique() {
            return Some((i + len).into());
        }
    }

    None
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
