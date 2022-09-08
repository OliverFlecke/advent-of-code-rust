use advent_of_code::{client::*, Level, Year};
use std::error::Error;

mod solutions;

use crate::solutions::{y2017::day01::Day01, Solution};

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Implement arg parser to get these values
    let year = Year::Y2017;
    let day = 1;

    let problem_input = match get_input(year, day) {
        Ok(input) => input,
        Err(_) => panic!("Unable to get input"),
    };

    submit(Year::Y2017, 1, Level::A, &Day01::solve_a(&problem_input));
    submit(Year::Y2017, 1, Level::B, &Day01::solve_b(&problem_input));

    Ok(())
}
