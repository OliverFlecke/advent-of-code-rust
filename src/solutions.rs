use crate::{Day, Year};

use self::answer::Answer;

pub mod answer;
pub mod y2017;
pub mod y2022;

pub trait Solution {
    fn solve_a(&self, input: &str) -> Option<Answer>;
    fn solve_b(&self, input: &str) -> Option<Answer>;
}

pub fn get_solver(year: Year, day: Day) -> Option<Box<dyn Solution>> {
    match year {
        Year::Y2017 => match day {
            1 => Some(Box::new(y2017::day01::Day01 {})),
            2 => Some(Box::new(y2017::day02::Day02 {})),
            3 => Some(Box::new(y2017::day03::Day03 {})),
            4 => Some(Box::new(y2017::day04::Day04 {})),
            5 => Some(Box::new(y2017::day05::Day05 {})),
            6 => Some(Box::new(y2017::day06::Day06 {})),
            7 => Some(Box::new(y2017::day07::Day07 {})),
            8 => Some(Box::new(y2017::day08::Day08 {})),
            9 => Some(Box::new(y2017::day09::Day09 {})),
            10 => Some(Box::new(y2017::day10::Day10 {})),
            11 => Some(Box::new(y2017::day11::Day11 {})),
            12 => Some(Box::new(y2017::day12::Day12 {})),
            13 => Some(Box::new(y2017::day13::Day13 {})),
            14 => Some(Box::new(y2017::day14::Day14 {})),
            15 => Some(Box::new(y2017::day15::Day15 {})),
            16 => Some(Box::new(y2017::day16::Day16 {})),
            17 => Some(Box::new(y2017::day17::Day17 {})),
            18 => Some(Box::new(y2017::day18::Day18 {})),
            19 => Some(Box::new(y2017::day19::Day19 {})),
            20 => Some(Box::new(y2017::day20::Day20 {})),
            21 => Some(Box::new(y2017::day21::Day21 {})),
            22 => Some(Box::new(y2017::day22::Day22 {})),
            23 => Some(Box::new(y2017::day23::Day23 {})),
            24 => Some(Box::new(y2017::day24::Day24 {})),
            25 => Some(Box::new(y2017::day25::Day25 {})),
            _ => None,
        },
        Year::Y2022 => match day {
            1 => Some(Box::new(y2022::day01::Day01 {})),
            2 => Some(Box::new(y2022::day02::Day02 {})),
            3 => Some(Box::new(y2022::day03::Day03 {})),
            4 => Some(Box::new(y2022::day04::Day04 {})),
            5 => Some(Box::new(y2022::day05::Day05 {})),
            6 => Some(Box::new(y2022::day06::Day06 {})),
            7 => Some(Box::new(y2022::day07::Day07 {})),
            8 => Some(Box::new(y2022::day08::Day08 {})),
            9 => Some(Box::new(y2022::day09::Day09 {})),
            10 => Some(Box::new(y2022::day10::Day10 {})),
            _ => None,
        },
        _ => None,
    }
}
