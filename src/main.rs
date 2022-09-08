use advent_of_code::{client::*, Level, Year};
use std::error::Error;

mod solutions;

use crate::solutions::{y2017::day01::Day01, Solution};

fn main() -> Result<(), Box<dyn Error>> {
    let problem_input = match get_input(Year::Y2017, 1) {
        Ok(input) => input,
        Err(_) => panic!("Unable to get input"),
    };

    assert_eq!(Day01::solve_a("1122"), "3");
    assert_eq!(Day01::solve_a("1111"), "4");
    assert_eq!(Day01::solve_a("1234"), "0");
    assert_eq!(Day01::solve_a("91212129"), "9");
    submit(Year::Y2017, 1, Level::A, &Day01::solve_a(&problem_input));

    Ok(())
}
