use array2d::Array2D;
use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::solutions::{answer::Answer, Solution};

const GALAXY: char = '#';

pub struct Day11;

impl Solution for Day11 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let answer = solve(input, 2);
        Some(answer.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let answer = solve(input, 1_000_000);
        Some(answer.into())
    }
}

fn solve(input: &str, expand_factor: usize) -> usize {
    let cells: Vec<Vec<_>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let map = Array2D::from_rows(&cells).unwrap();

    let empty_rows: FxHashSet<_> = {
        let mut set = FxHashSet::default();
        for (index, mut row) in map.rows_iter().enumerate() {
            if row.all(|c| *c != GALAXY) {
                set.insert(index);
            }
        }
        set
    };
    let empty_cols: FxHashSet<usize> = {
        let mut set = FxHashSet::default();
        for (index, mut column) in map.columns_iter().enumerate() {
            if column.all(|c| *c != GALAXY) {
                set.insert(index);
            }
        }
        set
    };
    // println!("Cols: {:?}", empty_cols);
    // println!("Rows: {:?}", empty_rows);

    let galaxies: FxHashSet<_> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == GALAXY)
                .map(|(col, _)| (row, col))
                .collect::<Vec<_>>()
        })
        .collect();
    let pairs = galaxies.iter().combinations(2).collect::<Vec<_>>();

    pairs
        .into_iter()
        .map(|x| {
            let a = x[0];
            let b = x[1];
            let doubles = doubles(&empty_rows, &empty_cols, a, b);
            manhattan_distance(*a, *b) + (expand_factor - 1) * doubles
        })
        .sum()
}

type Position = (usize, usize);

fn doubles(
    empty_rows: &FxHashSet<usize>,
    empty_cols: &FxHashSet<usize>,
    a: &Position,
    b: &Position,
) -> usize {
    let row_range = a.0.min(b.0)..a.0.max(b.0);
    let col_range = a.1.min(b.1)..a.1.max(b.1);

    row_range.filter(|r| empty_rows.contains(r)).count()
        + col_range.filter(|c| empty_cols.contains(c)).count()
}

fn manhattan_distance(a: Position, b: Position) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

#[cfg(test)]
mod test {
    use advent_of_code_client::{AocClient, Problem, Year};

    use super::*;

    const PROBLEM: Problem = Problem::new(Year::Y2023, 11);
    const INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;

    #[test]
    fn test_a() {
        assert_eq!(Day11 {}.solve_a(INPUT), Some(Answer::UInt(374)));
    }

    #[test]
    fn solve_a() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day11 {}.solve_a(&input), Some(Answer::UInt(9639160)));
    }

    #[test]
    fn test_b() {
        assert_eq!(solve(INPUT, 10), 1030);
        assert_eq!(solve(INPUT, 100), 8410);
    }

    #[test]
    fn solve_b() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day11 {}.solve_b(&input), Some(Answer::UInt(752936133304)));
    }
}
