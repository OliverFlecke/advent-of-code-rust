use itertools::{Itertools, MinMaxResult};

use crate::solutions::Solution;

pub struct Day02 {}

impl Solution for Day02 {
    fn solve_a(&self, input: &str) -> String {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                match line
                    .split('\t')
                    .filter(|n| !n.trim().is_empty())
                    .map(|n| n.parse::<u64>().unwrap())
                    .minmax()
                {
                    MinMaxResult::MinMax(min, max) => max - min,
                    x => panic!("No min/max found: {:?}", x),
                }
            })
            .sum::<u64>()
            .to_string()
    }

    fn solve_b(&self, input: &str) -> String {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                let numbers = line
                    .split('\t')
                    .filter(|n| !n.trim().is_empty())
                    .map(|n| n.parse::<u64>().unwrap())
                    .sorted()
                    .collect_vec();

                for i in 0..numbers.len() {
                    for j in i + 1..numbers.len() {
                        let (a, b) = (numbers[j], numbers[i]);
                        if a % b == 0 {
                            return a / b;
                        }
                    }
                }
                unreachable!()
            })
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = "5\t1\t9\t5\n7\t5\t3\n2\t4\t6\t8";

        assert_eq!(Day02 {}.solve_a(input), "18");
    }

    #[test]
    fn test_b() {
        let input = "5\t9\t2\t8\n9\t4\t7\t3\n3\t8\t6\t5";
        assert_eq!(Day02 {}.solve_b(input), "9");
    }
}
