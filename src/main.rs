use advent_of_code::{client::*, Day, Level, Year};
use solutions::get_solver;
use std::error::Error;

mod solutions;

fn main() -> Result<(), Box<dyn Error>> {
    // TODO: Implement arg parser to get these values
    let year = Year::Y2017;
    let day: Day = 2;

    let solver = get_solver(year, day);
    let problem_input = match get_input(year, day) {
        Ok(input) => input,
        Err(_) => panic!("Unable to get input"),
    };

    submit(Year::Y2017, day, Level::A, &solver.solve_a(&problem_input));
    submit(Year::Y2017, day, Level::B, &solver.solve_b(&problem_input));

    Ok(())
}
