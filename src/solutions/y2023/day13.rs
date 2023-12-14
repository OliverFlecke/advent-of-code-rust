use array2d::Array2D;
use duplicate::duplicate_item;

use crate::solutions::{answer::Answer, Solution};

pub struct Day13;

impl Solution for Day13 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        Some(solve(input, 0).into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        Some(solve(input, 1).into())
    }
}

fn solve(input: &str, expected: usize) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|image| {
            let grid: Vec<Vec<_>> = image.lines().map(|line| line.chars().collect()).collect();
            Array2D::from_rows(&grid).unwrap()
        })
        .map(|image| {
            vertical_split(&image, expected)
                .map(|x| x + 1)
                .or(horizontal_split(&image, expected).map(|x| (x + 1) * 100))
                .unwrap()
        })
        .sum()
}

type Image = Array2D<char>;

#[duplicate_item(
    direction          len_fun        iter_fun;
    [vertical_split]   [num_columns]  [column_iter];
    [horizontal_split] [num_rows]     [row_iter];
)]
fn direction(image: &Image, expected: usize) -> Option<usize> {
    fn check_if_mirror(image: &Image, mid: usize, expected: usize) -> bool {
        let mut diff = 0;
        let (mut left, mut right) = (mid, mid + 1);
        while let Ok(l) = image.iter_fun(left)
            && let Ok(r) = image.iter_fun(right)
        {
            diff += l.zip(r).filter(|(a, b)| a != b).count();
            if diff > expected {
                return false;
            }

            left = left.wrapping_sub(1);
            right += 1;
        }

        diff == expected
    }

    (0..image.len_fun() - 1).find(|mid| check_if_mirror(image, *mid, expected))
}

#[cfg(test)]
mod test {
    use advent_of_code_client::{AocClient, Problem, Year};

    use super::*;

    const PROBLEM: Problem = Problem::new(Year::Y2023, 13);
    const INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;

    #[test]
    fn test_a() {
        assert_eq!(Day13 {}.solve_a(INPUT), Some(Answer::UInt(405)));
    }

    #[test]
    fn solve_a() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day13 {}.solve_a(&input), Some(Answer::UInt(37113)));
    }

    #[test]
    fn test_b() {
        assert_eq!(Day13 {}.solve_b(INPUT), Some(Answer::UInt(400)));
    }

    #[test]
    fn solve_b() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day13 {}.solve_b(&input), Some(Answer::UInt(30449)));
    }
}
