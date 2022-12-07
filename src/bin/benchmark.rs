use std::{
    error::Error,
    time::{Duration, Instant},
};

use advent_of_code::{
    client::get_input,
    solutions::{answer::Answer, get_solver},
    Year,
};
use clap::Parser;
use colored::Colorize;
use serde::Serialize;

#[derive(Debug, Parser)]
struct Args {
    #[arg(value_enum)]
    year: Year,
    #[arg(
        short = 'i',
        long,
        default_value_t = 1_000,
        help = "Number of iteration to run each solution for"
    )]
    iterations: u32,
    #[arg(short, long, help = "Output csv file to write the benchmarks to")]
    output: Option<String>,
}

const ANSWER_WIDTH: usize = 32;

/// Benchmark a year. This will run and time all available solutions for the given year.
/// It assumes that the solutions are created from the start and to the end, and will break
/// if on the first day that is missing.
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

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

    let mut writer = args
        .output
        .clone()
        .map(|out| csv::WriterBuilder::new().from_path(out).unwrap());

    for day in 1..=25 {
        let solver = match get_solver(year, day) {
            Some(solver) => solver,
            None => break,
        };
        let problem_input = get_input(year, day)
            .unwrap_or_else(|_| panic!("no input for {year:?}/{day} was found"));

        let (answer_a, elapsed_a) =
            benchmark(args.iterations, &problem_input, |s| solver.solve_a(s));
        let (answer_b, elapsed_b) =
            benchmark(args.iterations, &problem_input, |s| solver.solve_b(s));

        println!(
            "Day {day: >2} \t| {:>ANSWER_WIDTH$} | {:>ANSWER_WIDTH$} | {elapsed_a:>16?} | {elapsed_b:>16?} ",
            answer_a.clone().map(|x| x.to_string()).unwrap_or_default(),
            answer_b.clone().map(|x| x.to_string()).unwrap_or_default(),
        );

        total_a += elapsed_a;
        total_b += elapsed_b;

        if args.output.is_some() {
            let bench = Benchmark {
                day,
                answer_a,
                answer_b,
                elapsed_a: elapsed_a.as_nanos(),
                elapsed_b: elapsed_b.as_nanos(),
            };

            writer.as_mut().unwrap().serialize(bench)?;
        }
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

    Ok(())
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Benchmark {
    day: u8,
    answer_a: Option<Answer>,
    answer_b: Option<Answer>,
    elapsed_a: u128,
    elapsed_b: u128,
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
