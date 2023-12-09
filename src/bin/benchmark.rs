use std::{
    error::Error,
    time::{Duration, Instant},
};

use advent_of_code::solutions::{answer::Answer, get_solver};
use advent_of_code_client::{AocClient, Year};
#[cfg(feature = "memory-profile")]
use byte_unit::Byte;
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
        default_value_t = 1_0,
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
///
/// To run with memory profiling enabled, use:
/// ```sh
/// cargo run -r --bin benchmark --features memory-profile -- y2023 -i 1
/// ```
///
/// It is generally advised to use `-i 1` to only run one iteration, as
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let year = args.year;
    println!("Running benchmarks for {year:?}");

    // Write header
    print!(
        "{}",
        format!(
            "        | {:^ANSWER_WIDTH$} | {:^ANSWER_WIDTH$} ",
            "Part A", "Part B",
        )
        .cyan()
    );

    #[cfg(not(feature = "memory-profile"))]
    {
        print!(
            "{}",
            format!("| {:^16} | {:^16}", "Elapsed A", "Elapsed B").cyan()
        );
    }

    #[cfg(feature = "memory-profile")]
    {
        print!(
            "{}",
            format!(
                "| {:^16} | {:^16} | {:^16} | {:^16} ",
                "A bytes used", "A peak bytes", "B bytes used", "B peak bytes",
            )
            .cyan()
        );
    }

    println!();

    #[cfg(not(feature = "memory-profile"))]
    let mut total_a = Duration::ZERO;
    #[cfg(not(feature = "memory-profile"))]
    let mut total_b = Duration::ZERO;

    #[cfg(feature = "memory-profile")]
    let mut memory_total_a: u64 = 0;
    #[cfg(feature = "memory-profile")]
    let mut memory_total_b: u64 = 0;
    #[cfg(feature = "memory-profile")]
    let mut memory_peak_a: usize = 0;
    #[cfg(feature = "memory-profile")]
    let mut memory_peak_b: usize = 0;

    let mut writer = args
        .output
        .clone()
        .map(|out| csv::WriterBuilder::new().from_path(out).unwrap());

    let client = AocClient::default();

    for day in 1..=25 {
        let solver = match get_solver((year, day).into()) {
            Some(solver) => solver,
            None => continue,
        };
        let problem_input = client
            .get_input((year, day).into())
            .unwrap_or_else(|_| panic!("no input for {year:?}/{day} was found"));

        let (answer_a, stats_a) = benchmark(args.iterations, &problem_input, |s| solver.solve_a(s));
        let (answer_b, stats_b) = benchmark(args.iterations, &problem_input, |s| solver.solve_b(s));

        print!(
            "Day {day: >2} \t| {:>ANSWER_WIDTH$} | {:>ANSWER_WIDTH$} ",
            answer_a.clone().map(|x| x.to_string()).unwrap_or_default(),
            answer_b.clone().map(|x| x.to_string()).unwrap_or_default(),
        );

        #[cfg(not(feature = "memory-profile"))]
        {
            print!(
                "| {elapsed_a:>16?} | {elapsed_b:>16?} ",
                elapsed_a = stats_a.duration,
                elapsed_b = stats_b.duration
            );

            total_a += stats_a.duration;
            total_b += stats_b.duration;
        }

        #[cfg(feature = "memory-profile")]
        {
            print!("| {:>#16.6} ", Byte::from(stats_a.memory_stats.total_bytes));
            print!("| {:>#16.6} ", Byte::from(stats_a.memory_stats.max_bytes));
            print!("| {:>#16.6} ", Byte::from(stats_b.memory_stats.total_bytes));
            print!("| {:>#16.6} ", Byte::from(stats_b.memory_stats.max_bytes));

            memory_total_a += stats_a.memory_stats.total_bytes;
            memory_peak_a = memory_peak_a.max(stats_a.memory_stats.max_bytes);
            memory_total_b += stats_b.memory_stats.total_bytes;
            memory_peak_b = memory_peak_b.max(stats_b.memory_stats.max_bytes);
        }

        println!();

        if args.output.is_some() {
            let bench = Benchmark {
                day,
                answer_a,
                answer_b,
                elapsed_a: stats_a.duration.as_nanos(),
                elapsed_b: stats_b.duration.as_nanos(),
            };

            writer.as_mut().unwrap().serialize(bench)?;
        }
    }

    print!(
        "{}",
        format!("Total   | {:^ANSWER_WIDTH$} | {:^ANSWER_WIDTH$} ", "", "").green()
    );

    #[cfg(not(feature = "memory-profile"))]
    {
        println!("{}", format!("| {total_a:>16?} | {total_b:>16?} ").green());
        println!(
            "Total time for both parts: {}",
            format!("{:?}", total_a + total_b).green()
        );
    }

    #[cfg(feature = "memory-profile")]
    {
        print!("| {:>#16.6} ", Byte::from(memory_total_a));
        print!("| {:>#16.6} ", Byte::from(memory_peak_a));
        print!("| {:>#16.6} ", Byte::from(memory_total_b));
        print!("| {:>#16.6} ", Byte::from(memory_peak_b));
        println!();

        println!(
            "Total memory for both parts: {}. Peak for both parts: {}",
            format!("{:>#16.6}", Byte::from(memory_total_a + memory_total_b)).green(),
            format!("{:>#16.6}", Byte::from(memory_peak_a.max(memory_peak_b))).green()
        );
    }

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

#[derive(Debug)]
struct BenchmarkStatistics {
    duration: Duration,
    #[cfg(feature = "memory-profile")]
    memory_stats: dhat::HeapStats,
}

/// Please note, that when running with memory benchmarks, the solutions will
/// run a lot slower, as the program has to keep track of all the allocations.
#[cfg(feature = "memory-profile")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn benchmark<F>(
    iterations: u32,
    problem_input: &str,
    solver: F,
) -> (Option<Answer>, BenchmarkStatistics)
where
    F: Fn(&str) -> Option<Answer>,
{
    let mut total = Duration::ZERO;
    let mut answer: Option<Answer> = None;
    #[cfg(feature = "memory-profile")]
    let mut memory_stats: Option<dhat::HeapStats> = None;

    for _ in 0..iterations {
        #[cfg(feature = "memory-profile")]
        let _profiler = dhat::Profiler::builder().testing().build();

        let start = Instant::now();
        let a = solver(problem_input);
        total += start.elapsed();

        #[cfg(feature = "memory-profile")]
        {
            memory_stats = Some(dhat::HeapStats::get());
        }

        if answer.is_none() {
            answer = a;
        }
    }

    (
        answer,
        BenchmarkStatistics {
            duration: total / iterations,
            #[cfg(feature = "memory-profile")]
            memory_stats: memory_stats.unwrap(),
        },
    )
}
