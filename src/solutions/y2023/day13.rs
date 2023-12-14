use array2d::Array2D;

use crate::solutions::{answer::Answer, Solution};

pub struct Day13;

impl Solution for Day13 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let answer: usize = input
            .trim()
            .split("\n\n")
            .map(|image| {
                let grid: Vec<Vec<_>> = image.lines().map(|line| line.chars().collect()).collect();
                Array2D::from_rows(&grid).unwrap()
            })
            .map(|image| {
                vertical_split(&image, 0)
                    .map(|x| x + 1)
                    .or(horizontal_split(&image, 0).map(|x| (x + 1) * 100))
                    .unwrap()
            })
            .sum();

        Some(answer.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let answer: usize = input
            .trim()
            .split("\n\n")
            .map(|image| {
                let grid: Vec<Vec<_>> = image.lines().map(|line| line.chars().collect()).collect();
                Array2D::from_rows(&grid).unwrap()
            })
            .map(|image| {
                vertical_split(&image, 1)
                    .map(|x| x + 1)
                    .or(horizontal_split(&image, 1).map(|x| (x + 1) * 100))
                    .unwrap()
            })
            .sum();

        Some(answer.into())
    }
}

type Image = Array2D<char>;

fn vertical_split(image: &Image, expected: usize) -> Option<usize> {
    fn check_if_mirror(image: &Image, mid: usize, expected: usize) -> bool {
        let mut diff = 0;
        let mut left = mid;
        let mut right = mid + 1;
        while let Ok(l) = image.column_iter(left)
            && let Ok(r) = image.column_iter(right)
        {
            let sum = l.zip(r).filter(|(a, b)| a != b).count();
            diff += sum;
            if diff > expected {
                return false;
            }

            left = left.wrapping_sub(1);
            right += 1;
        }

        diff == expected
    }

    (0..image.num_columns() - 1).find(|mid| check_if_mirror(image, *mid, expected))
}

fn horizontal_split(image: &Image, expected: usize) -> Option<usize> {
    fn check_if_mirror(image: &Image, mid: usize, expected: usize) -> bool {
        let mut diff = 0;
        let mut left = mid;
        let mut right = mid + 1;
        while let Ok(l) = image.row_iter(left)
            && let Ok(r) = image.row_iter(right)
        {
            let sum = l.zip(r).filter(|(a, b)| a != b).count();
            diff += sum;
            if diff > expected {
                return false;
            }

            left = left.wrapping_sub(1);
            right += 1;
        }

        diff == expected
    }

    (0..image.num_rows() - 1).find(|mid| check_if_mirror(image, *mid, expected))
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

    // #[test]
    // fn solve_b() {
    //     let input = AocClient::default().get_input(PROBLEM).unwrap();
    //     assert_eq!(Day13 {}.solve_b(&input), Some(Answer::UInt(todo!())));
    // }
}
