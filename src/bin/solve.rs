use advent_of_code::{client::*, solutions::get_solver, Level, Year};
use clap::Parser;
use colored::Colorize;
use std::time::Instant;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(value_enum)]
    year: Year,
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    #[arg(short = 'a', long)]
    submit_a: bool,
    #[arg(short = 'b', long)]
    submit_b: bool,
}

impl Args {
    pub fn problem(&self) -> Problem {
        (self.year, self.day).into()
    }
}

fn main() {
    let args = Args::parse();
    let problem = args.problem();
    let client = AocClient::default();

    let solver = get_solver(problem).unwrap_or_else(|| panic!("no solver found for {problem}"));

    let problem_input = client
        .get_input(problem)
        .unwrap_or_else(|_| panic!("no input for {problem} was found"));

    let start_a = Instant::now();
    if let Some(answer_a) = solver.solve_a(&problem_input) {
        println!(
            "Part A: {:>16} \tTime: {:>16?}",
            answer_a.to_string().cyan(),
            start_a.elapsed()
        );
        if args.submit_a {
            client.submit(problem, Level::A, &answer_a.to_string());
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
            client.submit(problem, Level::B, &answer_b.to_string());
        }
    }
}
