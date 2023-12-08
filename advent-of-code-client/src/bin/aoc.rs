use advent_of_code_client::{AocClient, Level, Problem, Year};
use anyhow::anyhow;
use clap::Parser;
use colored::Colorize;

/// Arguments for the CLI.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = Some(r#"Client to interact with Advent of Code. Used to submit answer for the daily puzzles.

To retrive your personal session token (varies by browser):
- Go to [adventofcode.com](https://adventofcode.com) and login
- Open the developer settings in your browser (F12)
- Go to `application` -> `Cookies`.
- You should see a session variable - this is the token we need."#
))]
struct Args {
    /// Year of AoC to submit puzzle solution for.
    #[arg(value_parser = clap::value_parser!(u16).range(2015..=Year::max() as i64))]
    year: u16,
    /// Day of AoC to submit puzzle solution for.
    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    /// Answer for part A.
    #[arg(short = 'a', long)]
    answer_a: Option<String>,

    /// Answer for part B.
    #[arg(short = 'b', long)]
    answer_b: Option<String>,

    /// Token to use for authenticating against Advent of Code.
    ///
    /// If not provided it will default to look for `AOC_TOKEN`
    /// in our current environment.
    ///
    /// Program panics if neither --token or `AOC_TOKEN` is provided.
    #[arg(short = 't', long)]
    token: Option<String>,
}

impl Args {
    fn problem(&self) -> Result<Problem, String> {
        (self.year, self.day).try_into()
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let problem = args.problem().expect("Clap parser handles verification");
    if args.answer_a.is_none() && args.answer_b.is_none() {
        return Err(anyhow!(
            "No answer provided for either part A or part B. Please provide at least one answer"
                .red()
        ));
    }

    let client = args.token.map(AocClient::from_token).unwrap_or_default();

    if let Some(answer) = args.answer_a {
        client.submit(problem, Level::A, &answer);
    }

    if let Some(answer) = args.answer_b {
        client.submit(problem, Level::B, &answer);
    }

    Ok(())
}
