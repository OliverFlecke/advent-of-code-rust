use advent_of_code::{client::*, Day, Level, Year};
use solutions::get_solver;
use std::{env, error::Error, time::Instant};

mod solutions;

fn main() -> Result<(), Box<dyn Error>> {
    let (year, day) = parse_args();

    let problem_input = match get_input(year, day) {
        Ok(input) => input,
        Err(_) => panic!("Unable to get input"),
    };

    let solver = get_solver(year, day);
    let start_a = Instant::now();
    let answer_a = solver.solve_a(&problem_input);
    println!(
        "Part A - Answer: {:?} - Time: {:?}",
        answer_a,
        start_a.elapsed()
    );

    submit(year, day, Level::A, &answer_a.to_string());

    let start_b = Instant::now();
    let answer_b = solver.solve_b(&problem_input);
    println!(
        "Part B - Answer: {:?} - Time: {:?}",
        answer_b,
        start_b.elapsed()
    );
    submit(year, day, Level::B, &answer_b.to_string());

    Ok(())
}

fn parse_args() -> (Year, Day) {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please pass the year and day to run");
    }
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
