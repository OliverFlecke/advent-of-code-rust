use std::{
    collections::{hash_map::DefaultHasher, HashSet},
    hash::{Hash, Hasher},
};

use itertools::Itertools;

use crate::solutions::{answer::Answer, Solution};

pub struct Day06 {}

impl Solution for Day06 {
    fn solve_a(&self, input: &str) -> Answer {
        fn hash(blocks: &Vec<u32>) -> u64 {
            let mut hasher = DefaultHasher::new();
            blocks.hash(&mut hasher);
            hasher.finish()
        }

        fn find_max(blocks: &Vec<u32>) -> (usize, u32) {
            let mut max_index = 0;
            let mut max_value = 0;
            for i in 0..blocks.len() {
                if blocks[i] > max_value {
                    max_index = i;
                    max_value = blocks[i];
                }
            }

            return (max_index, max_value);
        }

        fn next(blocks: &Vec<u32>) -> Vec<u32> {
            let (max_index, max_value) = find_max(&blocks);
            let mut new_blocks: Vec<u32> = Vec::with_capacity(blocks.len());
            for i in 0..blocks.len() {
                new_blocks.push(blocks[i]);
            }
            new_blocks[max_index] = 0;

            let mut index = (max_index + 1) % blocks.len();
            let mut value = max_value;
            while value > 0 {
                new_blocks[index] += 1;
                value -= 1;
                index = (index + 1) % blocks.len();
            }

            return new_blocks;
        }

        let mut blocks = input
            .trim()
            .split('\t')
            .map(|n| n.parse::<u32>().unwrap())
            .collect_vec();

        let mut seen = HashSet::new();
        seen.insert(hash(&blocks));
        let mut iterations: u64 = 0;
        loop {
            iterations += 1;
            let new_blocks = next(&blocks);
            let h = hash(&new_blocks);
            if seen.contains(&h) {
                return iterations.into();
            }

            seen.insert(h);
            blocks = new_blocks;
        }
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
        let input = "0\t2\t7\t0";
        assert_eq!(Day06 {}.solve_a(&input), Answer::UInt(5));
    }
}