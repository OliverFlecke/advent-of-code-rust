use advent_of_code::{client::*, Day, Level, Year};
use solutions::get_solver;
use std::{env, error::Error};

mod solutions;

fn main() -> Result<(), Box<dyn Error>> {
    let (year, day) = parse_args();

    let solver = get_solver(year, day);
    let problem_input = match get_input(year, day) {
        Ok(input) => input,
        Err(_) => panic!("Unable to get input"),
    };

    submit(year, day, Level::A, &solver.solve_a(&problem_input));
    submit(year, day, Level::B, &solver.solve_b(&problem_input));

    Ok(())
}

fn parse_args() -> (Year, Day) {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            let year: Year = match args[1].parse::<u32>() {
                Ok(n) => Year::from(n),
                Err(_) => panic!("Cannot parse parameter as a valid year"),
            };
            let day: Day = match args[2].parse::<u8>() {
                Ok(n) => match n {
                    1..=25 => n,
                    _ => panic!("Day must be a number from 1 to 25 (inclusive)"),
                },
                Err(_) => panic!("Connect parse 2nd parameter as a valid day"),
            };

            (year, day)
        }
        _ => panic!("Please pass the year and day to run"),
    }
}
