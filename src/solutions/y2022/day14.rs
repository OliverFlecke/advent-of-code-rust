use array2d::Array2D;
use itertools::{Itertools, MinMaxResult};

use crate::{
    solutions::{answer::Answer, Solution},
    Level,
};

pub struct Day14;

impl Solution for Day14 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let (mut map, x_bounds, y_bounds) = create_map(parse(input));

        Some(simulate(&mut map, x_bounds, y_bounds, Level::A).into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let (mut map, x_bounds, y_bounds) = create_map(parse(input));

        Some(simulate(&mut map, x_bounds, y_bounds, Level::B).into())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Block {
    Air,
    Rock,
    Sand,
}
use Block::*;

impl From<Block> for char {
    fn from(b: Block) -> Self {
        match b {
            Air => '.',
            Rock => '#',
            Sand => 'o',
        }
    }
}

fn simulate(
    map: &mut Array2D<Block>,
    (x_min, _x_max): Point,
    (_y_min, y_max): Point,
    level: Level,
) -> usize {
    let is_part_b = level == Level::B;
    let mut sand = 0;

    loop {
        let (mut col, mut row) = (500 - x_min, 0);

        if is_part_b && *map.get(row, col).unwrap() != Air {
            return sand;
        }

        loop {
            if is_part_b && row == y_max + 1 {
                break;
            }

            match map.get(row + 1, col) {
                Some(Air) => {
                    row += 1;
                    continue;
                }
                Some(_) => {}
                None => return sand,
            };

            // Manually check for underflow.
            // Doesn't actually matter in release mode, as the subtraction will be allowed and `map.get` will just return `None`
            if col == 0 {
                return sand;
            }
            match map.get(row + 1, col - 1) {
                Some(Air) => {
                    row += 1;
                    col -= 1;
                    continue;
                }
                Some(_) => {}
                None => return sand,
            };
            match map.get(row + 1, col + 1) {
                Some(Air) => {
                    row += 1;
                    col += 1;
                    continue;
                }
                Some(_) => break,
                None => return sand,
            };
        }

        map.set(row, col, Sand).unwrap();
        sand += 1;
    }
}

fn create_map(lines: Vec<Vec<Point>>) -> (Array2D<Block>, Point, Point) {
    let (x_min, x_max) = match lines.iter().flat_map(|l| l).map(|e| e.0).minmax() {
        MinMaxResult::NoElements | MinMaxResult::OneElement(_) => unreachable!(),
        MinMaxResult::MinMax(min, max) => (min, max),
    };

    // Choosing buffer based on what works for my input. To save space
    const BUFFER: usize = 150;
    let (x_min, x_max) = (x_min - BUFFER, x_max + BUFFER);
    let (_, y_max) = match lines.iter().flat_map(|l| l).map(|e| e.1).minmax() {
        MinMaxResult::NoElements | MinMaxResult::OneElement(_) => unreachable!(),
        MinMaxResult::MinMax(min, max) => (min, max),
    };

    // Always want this to be zero
    let y_min = 0;
    // println!("Bounds: [{x_min}-{x_max}] [{y_min}-{y_max}]");

    let cols = x_max.abs_diff(x_min) + 1;
    let rows = y_max + 2;

    let mut map = Array2D::filled_with(Air, rows, cols);
    lines.iter().for_each(|line| {
        line.iter().tuple_windows().for_each(|(a, b)| {
            for col in (a.0.min(b.0) - x_min)..=(a.0.max(b.0) - x_min) {
                for row in a.1.min(b.1)..=a.1.max(b.1) {
                    map.set(row, col, Rock).unwrap();
                }
            }
        })
    });

    (map, (x_min, x_max), (y_min, y_max))
}

#[allow(dead_code)]
fn print_map(map: &Array2D<Block>) {
    map.rows_iter().for_each(|r| {
        r.for_each(|block| print!("{}", char::from(*block)));
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

    #[test]
    fn test_b() {
        assert_eq!(Day14.solve_b(SAMPLE_INPUT), Some(Answer::UInt(93)))
    }
}
