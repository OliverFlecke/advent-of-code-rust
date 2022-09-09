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
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
