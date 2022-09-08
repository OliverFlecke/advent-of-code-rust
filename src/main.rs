use advent_of_code::*;

mod solutions;
use solutions::y2017::day01;

fn main() {
    let problem_input = get_input(Year::Y2017, 1);
    println!("input: {}", problem_input);

    day01::solve();
}
