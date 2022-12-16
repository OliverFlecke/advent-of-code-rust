use lazy_static::lazy_static;
use regex::Regex;

use crate::solutions::{answer::Answer, Solution};
const PART_B_MAX: isize = 4_000_000;

pub struct Day15;

impl Solution for Day15 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        const ROW: isize = 2_000_000;
        let sensors = parse(input);

        Some(part_a(&sensors, ROW).into())
    }
    fn solve_b(&self, input: &str) -> Option<Answer> {
        part_b(&parse(input), PART_B_MAX).map(Answer::from)
    }
}

fn part_a(sensors: &[Sensor], row: isize) -> usize {
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

fn part_b(sensors: &[Sensor], max: isize) -> Option<isize> {
    sensors.iter().find_map(|s| {
        (0.max(s.position.x - s.distance - 1)..=s.position.x.min(max))
            .zip(s.position.y..=max)
            .map(Point::from)
            .find_map(|p| {
                sensors
                    .iter()
                    .all(|s| !s.is_inside_range(p))
                    .then_some(p.x * PART_B_MAX + p.y)
            })
    })
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
        assert_eq!(part_a(&points, 10), 26)
    }

    #[test]
    fn test_b() {
        let sensors = parse(load_sample(Year::Y2022, "15.txt").unwrap().as_str());
        assert_eq!(part_b(&sensors, 20), Some(56000011));
    }
}
