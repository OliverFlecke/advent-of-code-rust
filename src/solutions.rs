use advent_of_code::{Day, Year};

use self::answer::Answer;

pub mod answer;
pub mod y2017;
pub mod y2022;

pub trait Solution {
    fn solve_a(&self, input: &str) -> Answer;
    fn solve_b(&self, input: &str) -> Answer;
}

pub fn get_solver(year: Year, day: Day) -> Box<dyn Solution> {
    match year {
        Year::Y2017 => match day {
            1 => Box::new(y2017::day01::Day01 {}),
            2 => Box::new(y2017::day02::Day02 {}),
            3 => Box::new(y2017::day03::Day03 {}),
            4 => Box::new(y2017::day04::Day04 {}),
            5 => Box::new(y2017::day05::Day05 {}),
            6 => Box::new(y2017::day06::Day06 {}),
            7 => Box::new(y2017::day07::Day07 {}),
            8 => Box::new(y2017::day08::Day08 {}),
            9 => Box::new(y2017::day09::Day09 {}),
            10 => Box::new(y2017::day10::Day10 {}),
            11 => Box::new(y2017::day11::Day11 {}),
            12 => Box::new(y2017::day12::Day12 {}),
            13 => Box::new(y2017::day13::Day13 {}),
            14 => Box::new(y2017::day14::Day14 {}),
            15 => Box::new(y2017::day15::Day15 {}),
            16 => Box::new(y2017::day16::Day16 {}),
            17 => Box::new(y2017::day17::Day17 {}),
            18 => Box::new(y2017::day18::Day18 {}),
            19 => Box::new(y2017::day19::Day19 {}),
            20 => Box::new(y2017::day20::Day20 {}),
            21 => Box::new(y2017::day21::Day21 {}),
            22 => Box::new(y2017::day22::Day22 {}),
            23 => Box::new(y2017::day23::Day23 {}),
            24 => Box::new(y2017::day24::Day24 {}),
            25 => Box::new(y2017::day25::Day25 {}),
            _ => panic!("Solution for day {} not found", day),
        },
        Year::Y2022 => match day {
            1 => Box::new(y2022::day01::Day01 {}),
            _ => panic!("Solution for day {} not found", day),
        },
        _ => panic!("Solution for year {:?} was not found", year),
    }
}
