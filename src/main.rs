use advent_of_code::{client::*, Level, Year};
use clap::Parser;
use solutions::get_solver;
use std::{error::Error, time::Instant};

mod solutions;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let problem_input = match get_input(args.year, args.day) {
        Ok(input) => input,
        Err(_) => panic!("Unable to get input"),
    };

    let solver = get_solver(args.year, args.day).expect("No solver is found for this day");

    let start_a = Instant::now();
    let answer_a = solver.solve_a(&problem_input);
    println!(
        "Part A - Answer: {:?} - Time: {:?}",
        answer_a,
        start_a.elapsed()
    );

    submit(args.year, args.day, Level::A, &answer_a.to_string());

    let start_b = Instant::now();
    let answer_b = solver.solve_b(&problem_input);
    println!(
        "Part B - Answer: {:?} - Time: {:?}",
        answer_b,
        start_b.elapsed()
    );
    submit(args.year, args.day, Level::B, &answer_b.to_string());

    Ok(())
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(value_enum)]
    year: Year,
    day: u8,
}
