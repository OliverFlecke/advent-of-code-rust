use crate::solutions::{answer::Answer, Solution};

pub struct Day09;

impl Solution for Day09 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let answer: isize = input
            .trim()
            .lines()
            .map(parse_line)
            .map(|numbers| {
                let mut sum: isize = 0;
                solve(|ns| _ = ns.last().inspect(|x| sum += *x), numbers);

                sum
            })
            .sum();

        Some(answer.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let answer: isize = input
            .trim()
            .lines()
            .map(parse_line)
            .map(|numbers| {
                let mut diffs: Vec<isize> = Vec::new();
                solve(|ns| _ = ns.first().inspect(|x| diffs.push(**x)), numbers);

                diffs.into_iter().rev().reduce(|a, b| b - a).unwrap_or(0)
            })
            .sum();

        Some(answer.into())
    }
}

fn parse_line(line: &str) -> Vec<isize> {
    line.split_whitespace()
        .filter_map(|x| x.parse::<isize>().ok())
        .collect::<Vec<_>>()
}

fn solve<F: FnMut(&Vec<isize>)>(mut aggregate: F, mut numbers: Vec<isize>) {
    while numbers.iter().any(|x| *x != 0) {
        aggregate(&numbers);
        numbers = numbers.into_iter().map_windows(|[a, b]| b - a).collect();
    }
}

#[cfg(test)]
mod test {
    use advent_of_code_client::{AocClient, Problem, Year};

    use super::*;

    const PROBLEM: Problem = Problem::new(Year::Y2023, 9);
    const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    #[test]
    fn test_a() {
        assert_eq!(Day09 {}.solve_a(INPUT), Some(Answer::Int(114)));
    }

    #[test]
    fn solve_a() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day09 {}.solve_a(&input), Some(Answer::Int(1681758908)));
    }

    #[test]
    fn test_b() {
        assert_eq!(Day09 {}.solve_b(INPUT), Some(Answer::Int(2)));
    }

    #[test]
    fn solve_b() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day09 {}.solve_b(&input), Some(Answer::Int(803)));
    }
}
