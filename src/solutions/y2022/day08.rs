use std::ops::AddAssign;

use array2d::Array2D;

use crate::solutions::{answer::Answer, Solution};

pub struct Day08;

impl Solution for Day08 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let trees = parse(input);
        let mut visible: Array2D<bool> =
            Array2D::filled_with(false, trees.num_rows(), trees.column_len());

        // println!("Down");
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

        // println!("Up");
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

        // println!("Left");
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

        // println!("Right");
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

        // visible.rows_iter().for_each(|r| {
        //     r.for_each(|x| print!("{}", if *x { 'T' } else { 'F' }));
        //     print!("\n");
        // });

        let mut sum: usize = 0;
        visible
            .elements_row_major_iter()
            .filter(|x| **x)
            .for_each(|_| sum.add_assign(1));

        Some(sum.into())
    }

    fn solve_b(&self, _input: &str) -> Option<Answer> {
        None
    }
}

fn check(
    trees: &Array2D<u8>,
    y: usize,
    x: usize,
    current: &u8,
    tallest: &mut u8,
    visible: &mut Array2D<bool>,
) {
    let is_visible = x == 0
        || y == 0
        || y == trees.row_len() - 1
        || x == trees.column_len() - 1
        || *current > *tallest;
    // println!(
    //     "Looking at {:?} -> {}. Visible: {}",
    //     (x, y),
    //     *current,
    //     is_visible
    // );
    if is_visible {
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
}
