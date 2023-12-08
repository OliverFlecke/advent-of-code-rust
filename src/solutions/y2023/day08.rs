use crate::{
    solutions::{answer::Answer, Solution},
    utils::math,
};
use regex::Regex;
use rustc_hash::FxHashMap;

pub struct Day08;

impl Solution for Day08 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let (path, network) = parse(input);
        let steps = find_distances_to_end(path, &network, |s| s == "ZZZ", &"AAA".to_owned());

        Some(steps.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let (path, network) = parse(input);
        let answer: usize = network
            .keys()
            .filter(|k| k.chars().nth(2) == Some('A'))
            .map(|start| {
                find_distances_to_end(path, &network, |s| s.chars().nth(2) == Some('Z'), start)
            })
            .reduce(math::lcm)
            .expect("no starting positions found");

        Some(answer.into())
    }
}

fn find_distances_to_end<F: Fn(&str) -> bool>(
    path: &str,
    network: &FxHashMap<String, Node>,
    is_end: F,
    start: &String,
) -> usize {
    let mut current = start;
    let mut steps: usize = 0;
    for choice in path.chars().cycle() {
        steps += 1;
        let node = network.get(current).unwrap();
        current = if choice == 'R' {
            &node.right
        } else {
            &node.left
        };

        if is_end(current) {
            break;
        }
    }

    steps
}

fn parse(input: &str) -> (&str, FxHashMap<String, Node>) {
    let re = Regex::new(r#"(?<from>[\dA-Z]{3}) = \((?<left>[\dA-Z]{3}), (?<right>[\dA-Z]{3})\)"#)
        .unwrap();
    let (path, network) = input.split_once("\n\n").unwrap();
    let network = network
        .trim()
        .lines()
        .map(|line| {
            re.captures(line)
                .map(|caps| {
                    (
                        caps["from"].to_string(),
                        Node {
                            left: caps["left"].to_string(),
                            right: caps["right"].to_string(),
                        },
                    )
                })
                .unwrap()
        })
        .collect();

    (path, network)
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        client::{AocClient, Problem},
        Year,
    };

    const PROBLEM: Problem = Problem::new(Year::Y2023, 8);
    const INPUT: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

    #[test]
    fn test_a() {
        assert_eq!(Day08 {}.solve_a(INPUT), Some(Answer::UInt(6)));
    }

    #[test]
    fn solve_a() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day08 {}.solve_a(&input), Some(Answer::UInt(19637)));
    }

    #[test]
    fn test_b() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;
        assert_eq!(Day08 {}.solve_b(input), Some(Answer::UInt(6)));
    }

    #[test]
    fn solve_b() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day08 {}.solve_b(&input), Some(Answer::UInt(8811050362409)));
    }
}
