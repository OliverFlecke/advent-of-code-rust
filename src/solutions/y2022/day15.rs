use std::{collections::HashMap, ops::Add};

use crate::solutions::{answer::Answer, Solution};

pub struct Day15;

impl Solution for Day15 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        const ROW: isize = 2_000_000;
        let map = run(parse(input));

        Some(count_row(&map, ROW).into())
    }
    fn solve_b(&self, _input: &str) -> Option<Answer> {
        None
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

impl Add<(isize, isize)> for Point {
    type Output = Self;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

fn parse(input: &str) -> Vec<(Point, Point)> {
    input.trim_end().lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> (Point, Point) {
    match line.split(['=', ',', ':']).collect::<Vec<_>>()[..] {
        [_, x, _, y, _, a, _, b] => (
            (x.parse().unwrap(), y.parse().unwrap()).into(),
            (a.parse().unwrap(), b.parse().unwrap()).into(),
        ),
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Thing {
    Empty,
    Sensor,
    Beacon,
}

use Thing::*;

fn run(points: Vec<(Point, Point)>) -> HashMap<Point, Thing> {
    let mut map: HashMap<Point, Thing> = HashMap::new();
    for (sensor, beacon) in points.iter() {
        map.insert(*sensor, Sensor);
        map.insert(*beacon, Beacon);
    }

    for (sensor, beacon) in points.iter() {
        let dist = sensor.manhattan_distance(&beacon) as isize;

        for x in 0..=dist {
            for y in 0..=(dist - x) {
                vec![
                    *sensor + (x, y),
                    *sensor + (x, -y),
                    *sensor + (-x, y),
                    *sensor + (-x, -y),
                ]
                .iter()
                .for_each(|p| {
                    if !map.contains_key(p) {
                        map.insert(*p, Empty);
                    }
                });
            }
        }
    }

    map
}

fn count_row(map: &HashMap<Point, Thing>, row: isize) -> usize {
    map.iter()
        .filter(|(p, v)| p.y == row && **v == Empty)
        .count()
}

#[cfg(test)]
mod test {
    use crate::{utils::load_sample, Year};

    use super::*;

    #[test]
    fn check_marking() {
        // Just to test whether the points are being marked correctly.
        let points = vec![((8, 7).into(), (2, 10).into())];
        let map = run(points);

        for y in -3..17 {
            print!("{y:<3}");
            for x in -2..20 {
                if map.get(&Point { x, y }).is_some() {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    #[test]
    fn parse_input() {
        let input = load_sample(Year::Y2022, "15.txt").unwrap();
        assert_eq!(
            parse(input.as_str()),
            vec![
                ((2, 18).into(), (-2, 15).into()),
                ((9, 16).into(), (10, 16).into()),
                ((13, 2).into(), (15, 3).into()),
                ((12, 14).into(), (10, 16).into()),
                ((10, 20).into(), (10, 16).into()),
                ((14, 17).into(), (10, 16).into()),
                ((8, 7).into(), (2, 10).into()),
                ((2, 0).into(), (2, 10).into()),
                ((0, 11).into(), (2, 10).into()),
                ((20, 14).into(), (25, 17).into()),
                ((17, 20).into(), (21, 22).into()),
                ((16, 7).into(), (15, 3).into()),
                ((14, 3).into(), (15, 3).into()),
                ((20, 1).into(), (15, 3).into()),
            ]
        );
    }

    #[test]
    fn test_a() {
        let points = parse(load_sample(Year::Y2022, "15.txt").unwrap().as_str());
        let map = run(points);
        assert_eq!(count_row(&map, 10), 26)
    }
}
