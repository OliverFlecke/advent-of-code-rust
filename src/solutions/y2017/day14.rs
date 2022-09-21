use crate::solutions::{answer::Answer, Solution};

use super::day10::KnotHash;

pub struct Day14 {}

impl Solution for Day14 {
    fn solve_a(&self, input: &str) -> Answer {
        let mut count: u32 = 0;
        for row in 0..128 {
            let row_input = format!("{}-{}", input, row);
            KnotHash::compute_hash(row_input)
                .into_iter()
                .for_each(|x| count += x.count_ones());
        }

        count.into()
    }

    fn solve_b(&self, _input: &str) -> Answer {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "flqrgnkx";

    #[test]
    fn test_a() {
        assert_eq!(Day14 {}.solve_a(TEST_INPUT), Answer::UInt(8108));
    }
}
