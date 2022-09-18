use crate::solutions::{answer::Answer, Solution};

const BLOCK_SIZE: usize = 16;
const SIZE: usize = 256;

pub struct Day10 {}

impl Solution for Day10 {
    fn solve_a(&self, input: &str) -> Answer {
        let mut array: Vec<usize> = (0..SIZE).collect();
        let mut position = 0;
        let mut skip_size = 0;

        (calc_round(
            &mut position,
            &mut skip_size,
            &mut array,
            &input
                .split(',')
                .map(|number| number.parse::<usize>().unwrap())
                .collect(),
        ) as u64)
            .into()
    }

    fn solve_b(&self, input: &str) -> Answer {
        knot_hash(input).into()
    }
}

fn calc_round(
    position: &mut usize,
    skip_size: &mut usize,
    array: &mut Vec<usize>,
    lengths: &Vec<usize>,
) -> usize {
    let size = array.len();
    let get_index = |i: usize| i % size;

    let mut i: usize = 0;
    while i < lengths.len() {
        let length = lengths[i];
        for x in (0..length / 2).map(|x| x as usize) {
            array.swap(
                get_index(*position + x),
                get_index(*position + length - x - 1),
            );
        }

        *position = get_index(*position + length + *skip_size);
        *skip_size += 1;
        i += 1;
    }

    array[0] * array[1]
}

fn knot_hash(input: &str) -> String {
    let mut lengths: Vec<usize> = input.chars().map(|c| c as usize).collect();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);

    let mut array: Vec<usize> = (0..SIZE).collect();
    let mut position = 0;
    let mut i = 0;

    for _ in 0..64 {
        calc_round(&mut position, &mut i, &mut array, &lengths);
    }

    let mut dense_hash: Vec<u8> = Vec::with_capacity(BLOCK_SIZE);

    for block in 0..BLOCK_SIZE {
        let mut value: u8 = 0;
        for i in 0..BLOCK_SIZE {
            value ^= array[block * BLOCK_SIZE + i] as u8;
        }
        dense_hash.push(value);
    }

    let mut hash = String::new();
    for value in dense_hash {
        hash.push_str(&format!("{:02x}", value));
    }

    hash
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        let mut array = (0..5).collect();
        let mut position = 0;
        let mut i = 0;
        assert_eq!(
            calc_round(&mut position, &mut i, &mut array, &vec![3, 4, 1, 5]),
            12
        );
    }

    #[test]
    fn test_b() {
        assert_eq!(knot_hash(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(knot_hash("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(knot_hash("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(knot_hash("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}
