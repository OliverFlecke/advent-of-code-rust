use std::ops::AddAssign;

use array2d::Array2D;

use crate::{
    solutions::{answer::Answer, Solution},
    utils::take_until_inclusive::TakeUntilInclusiveExt,
};

pub struct Day08;

impl Solution for Day08 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let trees = parse(input);
        let mut visible: Array2D<bool> =
            Array2D::filled_with(false, trees.num_rows(), trees.column_len());

        for x in 0..trees.column_len() {
            let mut tallest = 0;
            for y in 0..trees.row_len() {
                let current = trees.get(y, x).unwrap();

                check(&trees, y, x, current, &mut tallest, &mut visible);
                if *current == 9 {
                    break;
                }
            }
        }

        for x in 0..trees.column_len() {
            let mut tallest = 0;
            for y in (0..trees.row_len()).rev() {
                let current = trees.get(y, x).unwrap();

                check(&trees, y, x, current, &mut tallest, &mut visible);
                if *current == 9 {
                    break;
                }
            }
        }

        for y in 0..trees.row_len() {
            let mut tallest = 0;
            for x in (0..trees.column_len()).rev() {
                let current = trees.get(y, x).unwrap();

                check(&trees, y, x, current, &mut tallest, &mut visible);
                if *current == 9 {
                    break;
                }
            }
        }

        for y in (0..trees.row_len()).rev() {
            let mut tallest = 0;
            for x in 0..trees.column_len() {
                let current = trees.get(y, x).unwrap();

                check(&trees, y, x, current, &mut tallest, &mut visible);
                if *current == 9 {
                    break;
                }
            }
        }

        let mut sum: usize = 0;
        visible
            .elements_row_major_iter()
            .filter(|x| **x)
            .for_each(|_| sum.add_assign(1));

        Some(sum.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let forest = parse(input);

        Some(
            forest
                .rows_iter()
                .enumerate()
                .map(|(row, x)| {
                    x.enumerate()
                        .map(|(col, height)| scenic_score(&forest, height, col, row))
                        // .map(|(col, height)| scenic_score_loop(&forest, height, col, row))
                        .max()
                        .unwrap()
                })
                .max()
                .unwrap()
                .into(),
        )
    }
}

#[allow(dead_code)]
fn scenic_score_loop(forest: &Array2D<u8>, height: &u8, col: usize, row: usize) -> usize {
    let (mut above, mut below, mut left, mut right) = (0, 0, 0, 0);

    // Count left
    for x in (0..col).rev() {
        left += 1;
        if forest.get(row, x).unwrap() >= height {
            break;
        }
    }
    // Count right
    for x in (col + 1)..forest.column_len() {
        right += 1;
        if forest.get(row, x).unwrap() >= height {
            break;
        }
    }
    // Count above
    for y in (0..row).rev() {
        above += 1;
        if forest.get(y, col).unwrap() >= height {
            break;
        }
    }
    // Count below
    for y in (row + 1)..forest.row_len() {
        below += 1;
        if forest.get(y, col).unwrap() >= height {
            break;
        }
    }

    above * below * left * right
}

#[allow(dead_code)]
/// Compute the scenic score with iterators. Not sure if I actually think this is cleaner,
/// and it takes roughly 10µs longer (230µs with this solution vs 220µs with loops).
fn scenic_score(forest: &Array2D<u8>, height: &u8, col: usize, row: usize) -> usize {
    fn check_direction<F, I>(it: I, mapper: F, height: &u8) -> usize
    where
        F: Fn(usize) -> u8,
        I: Iterator<Item = usize>,
    {
        it.map(mapper).take_until_inclusive(|x| x >= height).count()
    }

    let left = check_direction((0..col).rev(), |col| *forest.get(row, col).unwrap(), height);
    let right = check_direction(
        (col + 1)..forest.column_len(),
        |col| *forest.get(row, col).unwrap(),
        height,
    );
    let above = check_direction((0..row).rev(), |row| *forest.get(row, col).unwrap(), height);
    let below = check_direction(
        (row + 1)..forest.row_len(),
        |row| *forest.get(row, col).unwrap(),
        height,
    );

    left * right * above * below
}

fn check(
    trees: &Array2D<u8>,
    y: usize,
    x: usize,
    current: &u8,
    tallest: &mut u8,
    visible: &mut Array2D<bool>,
) {
    if x == 0
        || y == 0
        || y == trees.row_len() - 1
        || x == trees.column_len() - 1
        || *current > *tallest
    {
        visible.set(y, x, true).unwrap();
    }

    if current > tallest {
        *tallest = *current;
    }
}

type Forest = Vec<Vec<u8>>;

fn parse(input: &str) -> Array2D<u8> {
    Array2D::from_rows(
        &input
            .trim_end()
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect::<Forest>(),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_a() {
        assert_eq!(Day08.solve_a(SAMPLE_INPUT), Some(Answer::UInt(21)))
    }

    #[test]
    fn test_b() {
        assert_eq!(Day08.solve_b(SAMPLE_INPUT), Some(Answer::UInt(8)))
    }
}
