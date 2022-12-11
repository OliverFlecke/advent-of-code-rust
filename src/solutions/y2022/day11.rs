use regex::Regex;

use crate::solutions::{answer::Answer, Solution};

pub struct Day11;

impl Solution for Day11 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let (mut items, monkies) = parse(input);
        let mut counts = vec![0; monkies.len()];

        // print_items(&items);

        for round in 0..20 {
            for (i, monkey) in monkies.iter().enumerate() {
                // let clone = items.iter().cloned().collect::<Vec<u32>>();
                let (a, b) =
                    items[i]
                        .iter()
                        .fold((Vec::new(), Vec::new()), |(mut a, mut b), item| {
                            let worry = monkey.operation.eval(&monkey.parameters, *item) / 3;
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

            // println!("\nRound: {round}");
            // print_items(&items);
        }

        // counts.iter().enumerate().for_each(|(i, count)| {
        //     println!("Monkey {i} inspected items {count} times");
        // });
        counts.sort();

        let monkey_business = counts[counts.len() - 1] * counts[counts.len() - 2];

        Some(monkey_business.into())
    }

    fn solve_b(&self, _input: &str) -> Option<Answer> {
        None
    }
}

fn parse(input: &str) -> (Vec<Vec<u32>>, Vec<Monkey>) {
    input
        .trim_end()
        .split("\n\n")
        .map(|x| {
            let (first, second) = x.split_once("Operation").unwrap();
            let mut lines = first.lines().skip(1);
            let items = parse_items(lines.next().unwrap());
            let monkey = Monkey::try_from(second).unwrap();

            (items, monkey)
        })
        .unzip()
}

fn parse_items(input: &str) -> Vec<u32> {
    input
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|x| x.trim().parse::<u32>().expect("to be number"))
        .collect::<Vec<u32>>()
}

fn print_items(monkies: &Vec<Vec<u32>>) {
    monkies.iter().enumerate().for_each(|(i, m)| {
        println!("Monkey {i}: {:?}", m);
    });
}

#[derive(Debug, Clone, PartialEq)]
struct Monkey {
    operation: Operation,
    parameters: Parameters,
    divisor: u32,
    true_index: u32,
    false_index: u32,
}

impl TryFrom<&str> for Monkey {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // println!("Parsing: {value}");

        let mut lines = value.lines();
        let op_pattern = Regex::new(r"(?P<first>\w+) (?P<op>\+|\*) (?P<second>\w+)").unwrap();
        let (operation, parameters) = lines
            .next()
            .map(|x| {
                let text = x.split('=').skip(1).next().unwrap();

                let caps = op_pattern.captures(text).unwrap();
                let op = if caps["op"] == *"+" {
                    Operation::Add
                } else {
                    Operation::Mul
                };

                let params = match (
                    caps["first"].to_string().as_str(),
                    caps["second"].to_string().as_str(),
                ) {
                    ("old", "old") => Parameters::ZeroConst,
                    ("old", x) | (x, "old") => Parameters::OneConst(x.parse().unwrap()),
                    _ => panic!("unable to parse parameters"),
                };

                (op, params)
            })
            .unwrap();

        let divisor = Regex::new(r"(?P<divisor>\d+)$")
            .unwrap()
            .captures(lines.next().unwrap())
            .unwrap()["divisor"]
            .parse::<u32>()
            .unwrap();

        let true_index = lines
            .next()
            .unwrap()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap();
        let false_index = lines
            .next()
            .unwrap()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap();

        Ok(Self {
            operation,
            parameters,
            divisor,
            true_index,
            false_index,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn eval(&self, param: &Parameters, old: u32) -> u32 {
        match self {
            Operation::Add => match param {
                Parameters::OneConst(v) => old + v,
                Parameters::ZeroConst => old + old,
            },
            Operation::Mul => match param {
                Parameters::OneConst(v) => old * v,
                Parameters::ZeroConst => old * old,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Parameters {
    OneConst(u32),
    ZeroConst,
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
                operation: Operation::Mul,
                parameters: Parameters::OneConst(19),
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
}
