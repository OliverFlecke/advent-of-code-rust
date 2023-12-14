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
                vertical_split(&image)
                    .map(|x| x + 1)
                    .or(horizontal_split(&image).map(|x| (x + 1) * 100))
                    .unwrap()
            })
            .sum();

        Some(answer.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        None
    }
}

fn vertical_split(image: &Array2D<char>) -> Option<usize> {
    fn check_if_mirror(image: &Array2D<char>, mid: usize) -> bool {
        let mut left = mid;
        let mut right = mid + 1;
        while let Ok(l) = image.column_iter(left)
            && let Ok(r) = image.column_iter(right)
        {
            println!("Checking {left} vs {right}");
            if !l.eq(r) {
                println!("{left} <> {right}");
                return false;
            }

            left = left.wrapping_sub(1);
            right += 1;
        }

        true
    }

    (0..image.num_columns() - 1).find(|mid| check_if_mirror(image, *mid))
}

fn horizontal_split(image: &Array2D<char>) -> Option<usize> {
    fn check_if_mirror(image: &Array2D<char>, mid: usize) -> bool {
        let mut left = mid;
        let mut right = mid + 1;
        while let Ok(l) = image.row_iter(left)
            && let Ok(r) = image.row_iter(right)
        {
            println!("Checking {left} vs {right}");
            if !l.eq(r) {
                println!("{left} <> {right}");
                return false;
            }

            left = left.wrapping_sub(1);
            right += 1;
        }

        true
    }

    (0..image.num_rows() - 1).find(|mid| check_if_mirror(image, *mid))
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

    // #[test]
    // fn test_b() {
    //     assert_eq!(Day13 {}.solve_b(INPUT), Some(Answer::UInt(todo!())));
    // }

    // #[test]
    // fn solve_b() {
    //     let input = AocClient::default().get_input(PROBLEM).unwrap();
    //     assert_eq!(Day13 {}.solve_b(&input), Some(Answer::UInt(todo!())));
    // }
}
