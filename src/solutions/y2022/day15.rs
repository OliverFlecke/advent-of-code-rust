use lazy_static::lazy_static;
use regex::Regex;

use crate::solutions::{answer::Answer, Solution};

pub struct Day15;

impl Solution for Day15 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        const ROW: isize = 2_000_000;
        let sensors = parse(input);

        Some(run(&sensors, ROW).into())
    }
    fn solve_b(&self, _input: &str) -> Option<Answer> {
        None
    }
}

fn run(sensors: &[Sensor], row: isize) -> usize {
    let l = sensors
        .iter()
        .map(|s| s.position.x - s.distance)
        .min()
        .unwrap();
    let h = sensors
        .iter()
        .map(|s| s.position.x + s.distance)
        .max()
        .unwrap();

    (l..=h)
        .filter(|&col| sensors.iter().any(|s| s.is_inside_range((col, row).into())))
        .count()
}

fn parse(input: &str) -> Vec<Sensor> {
    input.trim_end().lines().map(Sensor::from).collect()
}

lazy_static! {
    static ref PATTERN: Regex =
        Regex::new(r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)")
            .unwrap();
}

#[derive(Debug, Clone, Copy)]
struct Sensor {
    position: Point,
    beacon: Point,
    distance: isize,
}

impl Sensor {
    fn is_inside_range(&self, point: Point) -> bool {
        if self.beacon == point {
            return false;
        }
        self.distance >= self.position.manhattan_distance(&point) as isize
    }
}

impl From<&str> for Sensor {
    fn from(value: &str) -> Self {
        let caps = PATTERN.captures(value).unwrap();
        let s: Point = (caps["sx"].parse().unwrap(), caps["sy"].parse().unwrap()).into();
        let b: Point = (caps["bx"].parse().unwrap(), caps["by"].parse().unwrap()).into();

        Self {
            position: s,
            beacon: b,
            distance: s.manhattan_distance(&b) as isize,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn manhattan_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl From<(isize, isize)> for Point {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod test {
    use crate::{utils::load_sample, Year};

    use super::*;

    #[test]
    fn test_a() {
        let points = parse(load_sample(Year::Y2022, "15.txt").unwrap().as_str());
        assert_eq!(run(&points, 10), 26)
    }
}
