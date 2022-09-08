use advent_of_code::{client::*, Year};

mod solutions;

use crate::solutions::y2017::day01::Day01;

fn main() {
    let problem_input = match get_input(Year::Y2017, 1) {
        Ok(input) => input,
        Err(_) => panic!("Unable to get input"),
    };
    println!("input: {}", problem_input);

    Day01::solve()
}
