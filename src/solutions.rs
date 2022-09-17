use advent_of_code::{Day, Year};

use self::answer::Answer;

pub mod answer;
pub mod y2017;

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
            _ => panic!("Solution for day {} not found", day),
        },
        _ => panic!("Solution for year {:?} was not found", year),
    }
}
