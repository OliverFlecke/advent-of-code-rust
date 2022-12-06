use std::collections::HashMap;


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
    // Optimized solution to keep track of the last `len` chars and remove/add them manually.
    // This is instead of going through each `len` chars in each inner loop (done in the simpler solution below).
    // Runs in less than 50% compared to the original solution (338.696µs vs 125.458µs for part B).

    let bytes = input.as_bytes();
    let mut map: HashMap<u8, u8> = HashMap::new();
    for (i, b) in bytes.iter().enumerate() {
        if i >= len {
            let first = &bytes[i - len];
            let count = map.get(first).unwrap();
            if *count == 1 {
                map.remove(first);
            } else {
                map.entry(*first).and_modify(|x| {
                    *x -= 1;
                });
            }
        }

        map.entry(*b)
            .and_modify(|x| {
                *x += 1;
            })
            .or_insert(1);

        if map.len() == len {
            return Some(i + 1);
        }
    }

    // This is my original, simpler solution to solving the problem
    // use itertools::Itertools;
    // for i in 0..input.len() - len {
    //     if input[i..i + len].chars().all_unique() {
    //         return Some((i + len).into());
    //     }
    // }

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
