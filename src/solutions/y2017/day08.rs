use std::collections::HashMap;

use crate::solutions::{answer::Answer, Solution};

pub struct Day08 {}

enum Command {
    Inc(i64),
    Dec(i64),
}

enum Operator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqualTo,
}

impl Operator {
    fn compare(&self, left: i64, right: i64) -> bool {
        match self {
            Operator::Equal => left == right,
            Operator::NotEqualTo => left != right,
            Operator::LessThan => left < right,
            Operator::LessThanOrEqual => left <= right,
            Operator::GreaterThan => left > right,
            Operator::GreaterThanOrEqual => left >= right,
        }
    }
}

struct Condition {
    register: String,
    value: i64,
    operator: Operator,
}

impl Condition {
    fn check(&self, registers: &HashMap<String, i64>) -> bool {
        self.operator
            .compare(*registers.get(&self.register).unwrap_or(&0), self.value)
    }
}

impl From<&str> for Operator {
    fn from(s: &str) -> Self {
        match s {
            ">" => Operator::GreaterThan,
            ">=" => Operator::GreaterThanOrEqual,
            "<" => Operator::LessThan,
            "<=" => Operator::LessThanOrEqual,
            "==" => Operator::Equal,
            "!=" => Operator::NotEqualTo,
            _ => panic!("Cannot convert operator from {}", s),
        }
    }
}

struct Statement {
    register: String,
    command: Command,
    condition: Condition,
}

impl Statement {
    fn execute(&self, registers: &mut HashMap<String, i64>) {
        if self.condition.check(registers) {
            let current = *registers.get(&self.register).unwrap_or(&0);
            match self.command {
                Command::Inc(value) => registers.insert(self.register.to_owned(), current + value),
                Command::Dec(value) => registers.insert(self.register.to_owned(), current - value),
            };
        }
    }
}

impl Day08 {
    fn parse_line(line: &str) -> Statement {
        let parts: Vec<&str> = line.split(' ').collect();
        Statement {
            register: parts[0].to_string(),
            command: match parts[1] {
                "inc" => Command::Inc(parts[2].parse::<i64>().unwrap()),
                "dec" => Command::Dec(parts[2].parse::<i64>().unwrap()),
                _ => panic!("Command not understood: {}", parts[1]),
            },
            condition: Condition {
                register: parts[4].to_string(),
                operator: parts[5].into(),
                value: match parts[6].parse() {
                    Ok(x) => x,
                    Err(_) => panic!("Value: {}", parts[6]),
                },
            },
        }
    }
}

impl Solution for Day08 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let mut registers: HashMap<String, i64> = HashMap::new();

        input
            .trim()
            .split('\n')
            .map(Day08::parse_line)
            .for_each(|statement| statement.execute(&mut registers));

        Some((*registers.values().max().unwrap()).into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let mut registers: HashMap<String, i64> = HashMap::new();
        let mut max = 0;

        input
            .trim()
            .split('\n')
            .map(Day08::parse_line)
            .for_each(|statement| {
                statement.execute(&mut registers);
                let current_value = *registers.get(&statement.register).unwrap_or(&0);
                if current_value > max {
                    max = current_value;
                }
            });

        Some(max.into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
        "b inc 5 if a > 1\na inc 1 if b < 5\nc dec -10 if a >= 1\nc inc -20 if c == 10\n";

    #[test]
    fn test_a() {
        assert_eq!(Day08 {}.solve_a(INPUT), Some(Answer::Int(1)))
    }

    #[test]
    fn test_b() {
        assert_eq!(Day08 {}.solve_b(INPUT), Some(Answer::Int(10)))
    }
}
