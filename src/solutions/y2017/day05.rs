use itertools::Itertools;

use crate::solutions::{answer::Answer, Solution};

pub struct Day05 {}

fn parse(input: &str) -> Vec<i64> {
    input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i64>().unwrap())
        .collect_vec()
}

impl Solution for Day05 {
    fn solve_a(&self, input: &str) -> Answer {
        let mut jumps = parse(&input);

        let mut count: u64 = 0;
        let mut index: i64 = 0;
        loop {
            count += 1;
            let current = jumps[index as usize];
            let next = index + current;
            if next < 0 || jumps.len() as i64 <= next {
                return count.into();
            }

            jumps[index as usize] += 1;
            index = next;
        }
    }

    fn solve_b(&self, input: &str) -> Answer {
        let mut jumps = parse(&input);

        let mut count: u64 = 0;
        let mut index: i64 = 0;
        loop {
            count += 1;
            let current = jumps[index as usize];
            let next = index + current;
            if next < 0 || jumps.len() as i64 <= next {
                return count.into();
            }

            if current >= 3 {
                jumps[index as usize] -= 1;
            } else {
                jumps[index as usize] += 1;
            }
            index = next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = "0\n3\n0\n1\n-3";
        assert_eq!(Day05 {}.solve_a(input), Answer::UInt(5));
    }

    #[test]
    fn test_b() {
        let input = "0\n3\n0\n1\n-3";
        assert_eq!(Day05 {}.solve_b(input), Answer::UInt(10));
    }
}
