use crate::solutions::{answer::Answer, Solution};

pub struct Day10 {}

impl Solution for Day10 {
    fn solve_a(&self, input: &str) -> Answer {
        (solve_a(
            256,
            &input
                .trim()
                .split(',')
                .map(|number| number.parse::<usize>().unwrap())
                .collect(),
        ) as u64)
            .into()
    }

    fn solve_b(&self, _input: &str) -> Answer {
        todo!()
    }
}

fn solve_a(size: usize, lengths: &Vec<usize>) -> usize {
    let mut array: Vec<usize> = (0..size).collect();
    let get_index = |i: usize| i % size;

    let mut position = 0;
    let mut i = 0;
    while i < lengths.len() {
        let length = lengths[i];
        for x in (0..length / 2).map(|x| x as usize) {
            array.swap(
                get_index(position + x),
                get_index(position + length - x - 1),
            );
        }

        position = get_index(position + length + i);
        i += 1;
    }

    array[0] * array[1]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(solve_a(5, &vec![3, 4, 1, 5]), 12);
    }
}
