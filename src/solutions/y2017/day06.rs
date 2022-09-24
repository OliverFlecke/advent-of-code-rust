use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::{Hash, Hasher},
};

use crate::solutions::{answer::Answer, Solution};

pub struct Day06 {}

fn hash(blocks: &Vec<u32>) -> u64 {
    let mut hasher = DefaultHasher::new();
    blocks.hash(&mut hasher);
    hasher.finish()
}

fn find_max(blocks: &[u32]) -> (usize, u32) {
    let mut max_index = 0;
    let mut max_value = 0;
    for (i, v) in blocks.iter().enumerate() {
        if blocks[i] > max_value {
            max_index = i;
            max_value = *v;
        }
    }

    (max_index, max_value)
}

fn next(blocks: &Vec<u32>) -> Vec<u32> {
    let (max_index, max_value) = find_max(blocks);
    let mut new_blocks = blocks.clone();
    new_blocks[max_index] = 0;

    let mut index = (max_index + 1) % blocks.len();
    let mut value = max_value;
    while value > 0 {
        new_blocks[index] += 1;
        value -= 1;
        index = (index + 1) % blocks.len();
    }

    new_blocks
}

impl Solution for Day06 {
    fn solve_a(&self, input: &str) -> Answer {
        let mut blocks = input
            .trim()
            .split('\t')
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

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

    fn solve_b(&self, input: &str) -> Answer {
        let mut blocks = input
            .trim()
            .split('\t')
            .map(|n| n.parse::<u32>().unwrap())
            .collect();

        let mut seen = HashMap::new();
        seen.insert(hash(&blocks), 0);
        let mut iterations: u64 = 0;
        loop {
            iterations += 1;
            let new_blocks = next(&blocks);
            let h = hash(&new_blocks);
            if seen.contains_key(&h) {
                return (iterations - seen.get(&h).unwrap()).into();
            }

            seen.insert(h, iterations);
            blocks = new_blocks;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        let input = "0\t2\t7\t0";
        assert_eq!(Day06 {}.solve_a(input), Answer::UInt(5));
    }

    #[test]
    fn test_b() {
        let input = "0\t2\t7\t0";
        assert_eq!(Day06 {}.solve_b(input), Answer::UInt(4));
    }
}
