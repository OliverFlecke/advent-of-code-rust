use advent_of_code::{client::*, Level, Year};
use clap::Parser;
use colored::Colorize;
use solutions::{answer::Answer, get_solver};
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
        benchmark_year(&args);
    }

    Ok(())
}

/// Solve a given day, and submit to the AoC server (if desired).
fn solve_day(args: &Args, year: Year, day: u8) {
    let solver =
        get_solver(year, day).unwrap_or_else(|| panic!("no solver found for {year:?}/{day}"));

    let problem_input =
        get_input(year, day).unwrap_or_else(|_| panic!("no input for {year:?}/{day} was found"));

    let start_a = Instant::now();
    if let Some(answer_a) = solver.solve_a(&problem_input) {
        println!(
            "Part A: {:>16} \tTime: {:>16?}",
            answer_a.to_string().cyan(),
            start_a.elapsed()
        );
        if args.submit_a {
            submit(year, day, Level::A, &answer_a.to_string());
        }
    }

    let start_b = Instant::now();
    if let Some(answer_b) = solver.solve_b(&problem_input) {
        println!(
            "Part B: {:>16} \tTime: {:>16?}",
            answer_b.to_string().cyan(),
            start_b.elapsed()
        );
        if args.submit_b {
            submit(year, day, Level::B, &answer_b.to_string());
        }
    }
}

/// Benchmark a year. This will run and time all available solutions for the given year.
/// It assumes that the solutions are created from the start and to the end, and will break
/// if on the first day that is missing.
fn benchmark_year(args: &Args) {
    const ANSWER_WIDTH: usize = 32;
    let year = args.year;
    println!("Running benchmarks for {year:?}");
    println!(
        "{}",
        format!(
            "        | {:^ANSWER_WIDTH$} | {:^ANSWER_WIDTH$} | {:^16} | {:^16} ",
            "Part A", "Part B", "Elapsed A", "Elapsed B"
        )
        .cyan()
    );

    let mut total_a = Duration::ZERO;
    let mut total_b = Duration::ZERO;

    for day in 1..=25 {
        let solver = match get_solver(year, day) {
            Some(solver) => solver,
            None => break,
        };
        let problem_input = get_input(year, day)
            .unwrap_or_else(|_| panic!("no input for {year:?}/{day} was found"));

        let iterations = args.benchmark_iterations.unwrap_or(1_000);
        let (answer_a, elapsed_a) = benchmark(iterations, &problem_input, |s| solver.solve_a(s));
        let (answer_b, elapsed_b) = benchmark(iterations, &problem_input, |s| solver.solve_b(s));

        println!(
            "Day {day: >2} \t| {:>ANSWER_WIDTH$} | {:>ANSWER_WIDTH$} | {elapsed_a:>16?} | {elapsed_b:>16?} ",
            answer_a.map(|x| x.to_string()).unwrap_or_default(),
            answer_b.map(|x| x.to_string()).unwrap_or_default(),
        );

        total_a += elapsed_a;
        total_b += elapsed_b;
    }

    println!(
        "{}",
        format!(
            "Total   | {:^ANSWER_WIDTH$} | {:^ANSWER_WIDTH$} | {total_a:>16?} | {total_b:>16?} ",
            "", ""
        )
        .green()
    );
    println!(
        "Total time for both parts: {}",
        format!("{:?}", total_a + total_b).green()
    );
}

fn benchmark<F>(iterations: u32, problem_input: &str, solver: F) -> (Option<Answer>, Duration)
where
    F: Fn(&str) -> Option<Answer>,
{
    let mut total = Duration::ZERO;
    let mut answer: Option<Answer> = None;

    for _ in 0..iterations {
        let start = Instant::now();
        let a = solver(problem_input);
        total += start.elapsed();

        if answer.is_none() {
            answer = a;
        }
    }

    (answer, total / iterations)
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
    #[arg(short = 'i', long)]
    benchmark_iterations: Option<u32>,
}
