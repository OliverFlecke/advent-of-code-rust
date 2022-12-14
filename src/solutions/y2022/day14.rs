use array2d::Array2D;
use itertools::{Itertools, MinMaxResult};

use crate::solutions::{answer::Answer, Solution};

pub struct Day14;

impl Solution for Day14 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let lines = parse(input);
        let (x_min, x_max) = match lines.iter().flat_map(|l| l).map(|e| e.0).minmax() {
            MinMaxResult::NoElements | MinMaxResult::OneElement(_) => unreachable!(),
            MinMaxResult::MinMax(min, max) => (min, max),
        };
        let (_, y_max) = match lines.iter().flat_map(|l| l).map(|e| e.1).minmax() {
            MinMaxResult::NoElements | MinMaxResult::OneElement(_) => unreachable!(),
            MinMaxResult::MinMax(min, max) => (min, max),
        };
        let y_min = 0; // Always want this to be zero
        // println!("Bounds: [{x_min}-{x_max}] [{y_min}-{y_max}]");

        let cols = x_max.abs_diff(x_min) + 1;
        let rows = y_max + 1;
        let mut map = Array2D::filled_with(false, rows, cols);
        lines.iter().for_each(|line| {
            line.iter().tuple_windows().for_each(|(a, b)| {
                for col in (a.0.min(b.0) - x_min)..=(a.0.max(b.0) - x_min) {
                    for row in a.1.min(b.1)..=a.1.max(b.1) {
                        map.set(row, col, true).unwrap();
                    }
                }
            })
        });

        // print_map(&map);
        Some(simulate(&mut map, (x_min, x_max), (y_min, y_max)).into())
    }

    fn solve_b(&self, _input: &str) -> Option<Answer> {
        None
    }
}

fn simulate(map: &mut Array2D<bool>, (x_min, x_max): Point, (_, y_max): Point) -> usize {
    let mut sand = 0;

    loop {
        let (mut col, mut row) = (500 - x_min, 0);
        loop {
            if row >= y_max || col >= x_max {
                return sand;
            }

            match map.get(row + 1, col) {
                Some(false) => {
                    row += 1;
                    continue;
                }
                Some(true) => {}
                None => return sand,
            };

            if col == 0 {
                return sand;
            }
            match map.get(row + 1, col - 1) {
                Some(false) => {
                    row += 1;
                    col -= 1;
                    continue;
                }
                Some(true) => {}
                None => return sand,
            };
            match map.get(row + 1, col + 1) {
                Some(false) => {
                    row += 1;
                    col += 1;
                    continue;
                }
                Some(true) => break,
                None => return sand,
            };
        }
        map.set(row, col, true).unwrap();
        sand += 1;
    }
}

#[allow(dead_code)]
fn print_map(map: &Array2D<bool>) {
    map.rows_iter().for_each(|r| {
        r.for_each(|x| if *x { print!("#") } else { print!(".") });
        println!();
    });
}

type Point = (usize, usize);

fn parse(input: &str) -> Vec<Vec<Point>> {
    input.trim_end().lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Vec<Point> {
    line.split(" -> ")
        .map(|x| {
            let mut split = x.split(',');
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn parse_input() {
        assert_eq!(
            parse(SAMPLE_INPUT),
            vec![
                vec![(498, 4), (498, 6), (496, 6)],
                vec![(503, 4), (502, 4), (502, 9), (494, 9)]
            ]
        );
    }

    #[test]
    fn test_a() {
        assert_eq!(Day14.solve_a(SAMPLE_INPUT), Some(Answer::UInt(24)))
    }
}
