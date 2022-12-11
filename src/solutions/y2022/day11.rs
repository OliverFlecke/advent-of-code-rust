use regex::Regex;

use crate::solutions::{answer::Answer, Solution};

pub struct Day11;

impl Solution for Day11 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        Some(simulate(parse(input), 20, 3).into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        Some(simulate(parse(input), 10_000, 1).into())
    }
}

fn simulate((monkeys, mut items): MonkeysWithItems, rounds: usize, divisor: u64) -> usize {
    let mut counts = vec![0; monkeys.len()];
    let common: u64 = monkeys.iter().map(|m| m.divisor).product();

    for _ in 0..rounds {
        for (i, monkey) in monkeys.iter().enumerate() {
            let (a, b) = items[i]
                .iter()
                .fold((Vec::new(), Vec::new()), |(mut a, mut b), item| {
                    let worry = monkey.operation.eval(*item);
                    let worry = worry / divisor;
                    let worry = worry % common;

                    if worry % monkey.divisor == 0 {
                        a.push(worry);
                    } else {
                        b.push(worry);
                    }

                    (a, b)
                });

            items[monkey.true_index as usize].extend(a);
            items[monkey.false_index as usize].extend(b);
            counts[i] += items[i].len();
            items[i].clear();
        }
    }

    counts.sort();

    counts.iter().rev().take(2).product()
}

type MonkeysWithItems = (Vec<Monkey>, Vec<Vec<u64>>);

fn parse(input: &str) -> MonkeysWithItems {
    input
        .trim_end()
        .split("\n\n")
        .map(|x| {
            let (first, second) = x.split_once("Operation").unwrap();
            let mut lines = first.lines().skip(1);
            let items = parse_items(lines.next().unwrap());
            let monkey = Monkey::try_from(second).unwrap();

            (monkey, items)
        })
        .unzip()
}

fn parse_items(input: &str) -> Vec<u64> {
    input
        .split([':', ','])
        .skip(1)
        .map(|x| x.trim().parse::<u64>().expect("to be number"))
        .collect::<Vec<u64>>()
}

#[allow(dead_code)]
fn print_items(monkeys: &Vec<Vec<u64>>) {
    monkeys.iter().enumerate().for_each(|(i, m)| {
        println!("Monkey {i}: {:?}", m);
    });
}

#[derive(Debug, Clone, PartialEq)]
struct Monkey {
    operation: Operation,
    divisor: u64,
    true_index: u64,
    false_index: u64,
}

impl TryFrom<&str> for Monkey {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();
        let operation = lines
            .next()
            .map(|x| {
                let parts: Vec<&str> = x.split_whitespace().collect();

                match parts[..] {
                    [.., "*", "old"] => Operation::Square,
                    [.., "+", v] => Operation::Add(v.parse().unwrap()),
                    [.., "*", v] => Operation::Mul(v.parse().unwrap()),
                    _ => panic!("Unknown operation"),
                }
            })
            .unwrap();

        let divisor = Regex::new(r"(?P<divisor>\d+)$")
            .unwrap()
            .captures(lines.next().unwrap())
            .unwrap()["divisor"]
            .parse::<u64>()
            .unwrap();

        let true_index = lines
            .next()
            .unwrap()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as u64;
        let false_index = lines
            .next()
            .unwrap()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as u64;

        Ok(Self {
            operation,
            divisor,
            true_index,
            false_index,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Add(u64),
    Mul(u64),
    Square,
}

impl Operation {
    fn eval(&self, old: u64) -> u64 {
        match self {
            Operation::Add(v) => old + v,
            Operation::Mul(v) => old * v,
            Operation::Square => old * old,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{utils::load_sample, Year};

    use super::*;

    #[test]
    fn parse_monkey() {
        let sample = "Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

        let monkey = Monkey::try_from(sample);
        assert_eq!(
            monkey,
            Ok(Monkey {
                operation: Operation::Mul(19),
                // parameters: Parameters::OneConst(19),
                divisor: 23,
                true_index: 2,
                false_index: 3
            })
        )
    }

    #[test]
    fn test_a() {
        assert_eq!(
            Day11.solve_a(load_sample(Year::Y2022, "11.txt").unwrap().as_str()),
            Some(Answer::UInt(10605))
        )
    }
    #[test]
    fn test_b() {
        assert_eq!(
            Day11.solve_b(load_sample(Year::Y2022, "11.txt").unwrap().as_str()),
            Some(Answer::UInt(2713310158))
        )
    }
}
