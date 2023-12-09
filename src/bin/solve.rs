use std::time::Instant;

use advent_of_code::solutions::{answer::Answer, get_solver};
use advent_of_code_client::{AocClient, Level, Problem, Year};
use clap::Parser;
use colored::Colorize;

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

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let problem = args.problem();
    let client = AocClient::default();

    let solver = get_solver(problem).unwrap_or_else(|| panic!("no solver found for {problem}"));

    let problem_input = client
        .get_input(problem)
        .unwrap_or_else(|_| panic!("no input for {problem} was found"));
    let print_and_submit = |answer: Option<Answer>, level: Level| -> anyhow::Result<()> {
        if let Some(answer) = answer {
            println!("Part {level:?}:  {:>20}", answer.to_string().cyan());
            let should_submit = match level {
                Level::A => args.submit_a,
                Level::B => args.submit_b,
            };

            if should_submit {
                let result = client.submit(problem, level, &answer.to_string())?;
                println!("{result}");
            }
        }

        Ok(())
    };

    let answer_a = run_solver(|x| solver.solve_a(x), &problem_input);
    print_and_submit(answer_a, Level::A)?;

    let answer_b = run_solver(|x| solver.solve_b(x), &problem_input);
    print_and_submit(answer_b, Level::B)?;

    Ok(())
}

fn run_solver<F: Fn(&str) -> Option<Answer>>(solve: F, input: &str) -> Option<Answer> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::builder().testing().build();

    let start_a = Instant::now();
    let answer = solve(input);
    let elapsed = start_a.elapsed();

    #[cfg(feature = "dhat-heap")]
    {
        use byte_unit::Byte;

        let mem_stats = dhat::HeapStats::get();
        println!(
            "Total bytes: {:>#16.6} \tPeak  {:>#16.6}",
            Byte::from_u64(mem_stats.total_bytes),
            Byte::from_u64(mem_stats.max_bytes as u64),
        );
        println!(
            "Total alloc: {:>16} \tPeak  {:>16}",
            mem_stats.total_blocks, mem_stats.max_blocks,
        );
    }
    println!("Time:    {elapsed:>20?}");

    answer
}
