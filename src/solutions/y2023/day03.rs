use array2d::Array2D;
use itertools::Itertools;

use crate::solutions::{answer::Answer, Solution};

pub struct Day03;

impl Solution for Day03 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let parsed_input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let schematic = Array2D::from_rows(&parsed_input).unwrap();

        let check_around = |x: usize, y: usize| -> bool {
            ((-1..=1).cartesian_product(-1..=1))
                .filter(|pair| *pair != (0, 0))
                .map(|(dx, dy)| ((x as isize + dx, y as isize + dy), (dx, dy)))
                .filter_map(|((x, y), d)| schematic.get(y as usize, x as usize).map(|v| (v, d)))
                .any(|(value, (_, dy))| (dy != 0 || !value.is_ascii_digit()) && *value != '.')
        };

        let print_around = |x: usize, y: usize| {
            (y - 1..=y + 1).for_each(|y| {
                (x - 4..x + 1)
                    .filter_map(|x| schematic.get(y, x))
                    .for_each(|value| {
                        print!("{}", value);
                    });
                println!();
            });
            println!();
        };

        let sum: u64 = schematic
            .rows_iter()
            .enumerate()
            .map(|(y, row)| {
                row.enumerate()
                    .fold(
                        (false, 0_u64, 0_u64),
                        |(should_include, number, sum), (x, c)| {
                            c.to_digit(10)
                                .map(|digit| {
                                    print!("{c}/{digit} ");
                                    (
                                        should_include || check_around(x, y),
                                        number * 10_u64 + digit as u64,
                                        sum,
                                    )
                                })
                                .unwrap_or_else(|| {
                                    (
                                        false,
                                        0,
                                        if should_include {
                                            println!(" - Including number {number}");
                                            print_around(x, y);
                                            sum + number
                                        } else {
                                            if number != 0 {
                                                println!(
                                                    "Skipping number at ({x}, {y}) => {number}"
                                                );
                                            }
                                            sum
                                        },
                                    )
                                })
                        },
                    )
                    .2
            })
            .sum();

        Some(sum.into())
    }

    fn solve_b(&self, _input: &str) -> Option<Answer> {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        assert_eq!(Day03 {}.solve_a(input), Some(Answer::UInt(4361)))
    }
}
