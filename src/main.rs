use std::error::Error;
use advent_of_code::{client::*, Year};

mod solutions;

use crate::solutions::{Solution, y2017::day01::Day01};

fn main() -> Result<(), Box<dyn Error>> {
    let problem_input = match get_input(Year::Y2017, 1) {
        Ok(input) => input,
        Err(_) => panic!("Unable to get input"),
    };

    Day01::solve_a(&problem_input);

    Ok(())
}
