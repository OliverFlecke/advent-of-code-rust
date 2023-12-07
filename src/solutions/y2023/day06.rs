use std::time::Instant;

use crate::solutions::{answer::Answer, Solution};

pub struct Day06;

impl Solution for Day06 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let races = parse(input);

        let answer = races.iter().map(|x| x.wins()).product::<usize>();

        Some(answer.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let (times, distances) = input.trim().split_once('\n').unwrap();
        let time = times
            .strip_prefix("Time: ")
            .unwrap()
            .replace(' ', "")
            .parse::<usize>()
            .unwrap();
        let distance = distances
            .strip_prefix("Distance: ")
            .unwrap()
            .replace(' ', "")
            .parse::<usize>()
            .unwrap();
        // let answer = Race { time, distance }.wins2();
        let answer = Race { time, distance }.wins_compute();

        Some(answer.into())
    }
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

#[allow(dead_code)]
impl Race {
    /// Brute force through all solutions.
    fn wins(&self) -> usize {
        (1..self.time)
            .map(|speed| (speed, speed * (self.time - speed)))
            .filter(|(_, d)| *d > self.distance)
            .count()
    }

    /// Attempt to optimize brute force by only looking at the first and last
    /// race that was won.
    fn wins2(&self) -> usize {
        let start = Instant::now();
        let min = (1..self.time)
            .map(|speed| (speed, speed * (self.time - speed)))
            .filter(|(_, d)| *d > self.distance)
            .map(|(speed, _)| speed)
            .next()
            .unwrap();
        println!("Elapsed min: {:?}", start.elapsed());

        let start = Instant::now();
        let max = (1..self.time)
            .rev()
            .map(|speed| (speed, speed * (self.time - speed)))
            .filter(|(_, d)| *d > self.distance)
            .map(|(speed, _)| speed)
            .next()
            .unwrap();
        println!("Elapsed max: {:?}", start.elapsed());

        println!("Min max: {min}/{max}");

        max - min + 1
    }

    /// Finds the solution by computing the quadratic equation, which roots are
    /// where the min and max of where we would win the race.
    fn wins_compute(&self) -> usize {
        let d = ((self.time.pow(2) as f64) - 4_f64 * self.distance as f64).sqrt();
        let max = ((self.time as f64 + d) / 2_f64).floor() as usize;
        let min = ((self.time as f64 - d) / 2_f64).floor() as usize;

        max - min
    }
}

fn parse(input: &str) -> Vec<Race> {
    let (times, distances) = input.trim().split_once('\n').unwrap();
    let times = times
        .strip_prefix("Time:")
        .map(|s| s.split_whitespace().filter_map(|x| x.parse::<usize>().ok()))
        .unwrap();
    let distances = distances
        .strip_prefix("Distance:")
        .map(|s| s.split_whitespace().filter_map(|x| x.parse::<usize>().ok()))
        .unwrap();

    times
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{
        client::{AocClient, Problem},
        Year,
    };

    use super::*;

    const PROBLEM: Problem = Problem::new(Year::Y2023, 6);
    const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_a() {
        assert_eq!(Day06 {}.solve_a(INPUT), Some(Answer::UInt(288)));
    }

    #[test]
    fn solve_a() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day06 {}.solve_a(&input), Some(Answer::UInt(220320)));
    }

    #[test]
    fn test_b() {
        assert_eq!(Day06 {}.solve_b(INPUT), Some(Answer::UInt(71503)));
    }

    #[test]
    fn solve_b() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day06 {}.solve_b(&input), Some(Answer::UInt(34454850)));
    }
}
