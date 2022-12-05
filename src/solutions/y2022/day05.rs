use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::solutions::{answer::Answer, Solution};

pub struct Day05;

impl Solution for Day05 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let (mut stacks, commands) = parse(input);

        for command in commands {
            let from = stacks.get_mut(&command.from).unwrap();
            let mut items = from.split_off(from.len() - command.amount);
            items.reverse();

            stacks.get_mut(&command.to).unwrap().extend(items);
        }

        Some(get_top_items(stacks).into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let (mut stacks, commands) = parse(input);
        for command in commands {
            let from = stacks.get_mut(&command.from).unwrap();
            let items = from.split_off(from.len() - command.amount);

            stacks.get_mut(&command.to).unwrap().extend(items);
        }

        Some(get_top_items(stacks).into())
    }
}

fn get_top_items(stacks: HashMap<usize, Vec<char>>) -> String {
    stacks
        .iter()
        .sorted_by_key(|x| x.0)
        .map(|(_, x)| x.last().unwrap())
        .collect::<String>()
}

// Stack can just be a Vec
type Stacks = HashMap<usize, Vec<char>>;

fn parse(input: &str) -> (Stacks, Vec<Command>) {
    let mut splits = input.trim_end().split("\n\n");
    let stacks = splits.next().unwrap();
    let commands = splits.next().unwrap();

    let mut mapping: Stacks = HashMap::new();
    stacks
        .lines()
        .map(|l| l.chars().skip(1).step_by(4))
        .rev()
        .for_each(|l| {
            l.enumerate()
                .filter(|(_, c)| 'A' <= *c && *c <= 'Z')
                .for_each(|(i, c)| {
                    mapping
                        .entry(i + 1)
                        .and_modify(|v| v.push(c))
                        .or_insert(vec![c]);
                })
        });

    let re = Regex::new(r"move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)")
        .expect("regex to be valid");
    let commands = commands
        .lines()
        .map(|l| {
            let captures = re.captures(l).unwrap();
            Command {
                amount: captures["amount"].parse::<usize>().unwrap(),
                from: captures["from"].parse::<usize>().unwrap(),
                to: captures["to"].parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<Command>>();

    (mapping, commands)
}

struct Command {
    amount: usize,
    from: usize,
    to: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_a() {
        assert_eq!(
            Day05 {}.solve_a(SAMPLE_INPUT),
            Some(Answer::String("CMZ".to_string()))
        )
    }

    #[test]
    fn test_b() {
        assert_eq!(
            Day05 {}.solve_b(SAMPLE_INPUT),
            Some(Answer::String("MCD".to_string()))
        )
    }
}
