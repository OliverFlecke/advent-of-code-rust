use itertools::Itertools;

use crate::solutions::Solution;

pub struct Day01 {}

impl Solution for Day01 {
    fn solve_a<S>(input: S) -> String
    where
        S: AsRef<str>,
    {
        fn compare(a: u32, b: u32) -> u32 {
            if a == b {
                a
            } else {
                0
            }
        }

        let (mut first, mut last) = (None::<u32>, None::<u32>);
        let sum = input
            .as_ref()
            .chars()
            .into_iter()
            .filter_map(|c| c.to_digit(10))
            .tuple_windows()
            .fold(0, |sum, (a, b)| {
                last = if first.is_some() { Some(b) } else { None };
                first = first.or(Some(a));

                sum + compare(a, b)
            })
            + (first
                .map(|a| last.map(|b| if a == b { a } else { 0 }).unwrap_or_default())
                .unwrap_or_default());

        sum.to_string()
    }

    fn solve_b<S>(_input: S) -> String
    where
        S: AsRef<str>,
    {
        todo!()
    }
}
