use advent_of_code::{client::*, Level, Year};
use clap::Parser;
use colored::Colorize;
use solutions::get_solver;
use std::{
    error::Error,
    time::{Duration, Instant},
};

mod solutions;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if let Some(day) = args.day {
        solve_day(&args, args.year, day);
    } else {
        benchmark_year(args.year);
    }

    Ok(())
}

/// Solve a given day, and submit to the AoC server (if desired).
fn solve_day(args: &Args, year: Year, day: u8) {
    let problem_input =
        get_input(year, day).expect(format!("no input for {year:?}/{day} was found").as_str());

    let solver =
        get_solver(year, day).expect(format!("no solver found for {year:?}/{day}").as_str());

    let start_a = Instant::now();
    let answer_a = solver.solve_a(&problem_input);
    println!(
        "Part A - Answer: {:?} - Time: {:?}",
        answer_a,
        start_a.elapsed()
    );
    if args.submit_a {
        submit(year, day, Level::A, &answer_a.to_string());
    }

    let start_b = Instant::now();
    let answer_b = solver.solve_b(&problem_input);
    println!(
        "Part B - Answer: {:?} - Time: {:?}",
        answer_b,
        start_b.elapsed()
    );
    if args.submit_b {
        submit(year, day, Level::B, &answer_b.to_string());
    }
}

/// Benchmark a year. This will run and time all available solutions for the given year.
/// It assumes that the solutions are created from the start and to the end, and will break
/// if on the first day that is missing.
fn benchmark_year(year: Year) {
    println!("Running benchmarks for {year:?}");
    println!(
        "{}",
        format!(
            "        | {:^16} | {:^16} | {:^16} | {:^16} ",
            "Part A", "Part B", "Elapsed A", "Elapsed B"
        )
        .cyan()
    );

    let mut total_a = Duration::ZERO;
    let mut total_b = Duration::ZERO;

    for day in 1..=25 {
        let problem_input =
            get_input(year, day).expect(format!("no input for {year:?}/{day} was found").as_str());
        let solver = match get_solver(year, day) {
            Some(solver) => solver,
            None => break,
        };

        let start_a = Instant::now();
        let answer_a = solver.solve_a(&problem_input);
        let elapsed_a = start_a.elapsed();

        let start_b = Instant::now();
        let answer_b = solver.solve_b(&problem_input);
        let elapsed_b = start_b.elapsed();

        println!(
            "Day {day: >2} \t| {:>16} | {:>16} | {elapsed_a:>16?} | {elapsed_b:>16?} ",
            answer_a.to_string(),
            answer_b.to_string(),
        );

        total_a += elapsed_a;
        total_b += elapsed_b;
    }

    println!(
        "{}",
        format!(
            "Total   | {:^16} | {:^16} | {total_a:>16?} | {total_b:>16?} ",
            "", ""
        )
        .green()
    );
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(value_enum)]
    year: Year,
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: Option<u8>,

    #[arg(short = 'a', long)]
    submit_a: bool,
    #[arg(short = 'b', long)]
    submit_b: bool,
}
