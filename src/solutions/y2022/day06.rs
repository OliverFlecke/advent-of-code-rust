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

    fn solve_b(&self, _input: &str) -> Option<Answer> {
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
}
