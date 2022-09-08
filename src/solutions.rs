use advent_of_code::{Day, Year};

pub mod y2017;

pub trait Solution {
    fn solve_a(&self, input: &str) -> String;
    fn solve_b(&self, input: &str) -> String;
}

pub fn get_solver(year: Year, day: Day) -> Box<dyn Solution> {
    match year {
        Year::Y2017 => match day {
            1 => Box::new(y2017::day01::Day01 {}),
            2 => Box::new(y2017::day02::Day02 {}),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
