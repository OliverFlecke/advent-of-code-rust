use std::{collections::HashMap, iter::Map, str::Lines};

use crate::solutions::{answer::Answer, Solution};

pub struct Day18;

impl Solution for Day18 {
    fn solve_a(&self, input: &str) -> Answer {
        let instrutions: Vec<Inst> = parse(input).collect();
        Inst::execute(instrutions).unwrap().into()
    }

    fn solve_b(&self, _input: &str) -> Answer {
        todo!()
    }
}

fn parse(input: &str) -> Map<Lines<'_>, fn(&str) -> Inst> {
    input.lines().map(|l| l.into())
}

#[derive(PartialEq, Debug)]
enum RegOrVal {
    Register(char),
    Value(i64),
}

impl RegOrVal {
    fn get_value(&self, registers: &Registers) -> i64 {
        match self {
            RegOrVal::Register(r) => *registers.get(r).unwrap_or(&0),
            RegOrVal::Value(n) => *n,
        }
    }

    fn get_register(&self) -> char {
        match self {
            RegOrVal::Register(r) => *r,
            RegOrVal::Value(_) => panic!("Cannot get register from value"),
        }
    }
}

impl From<&str> for RegOrVal {
    fn from(value: &str) -> Self {
        match value.parse::<i64>() {
            Ok(n) => RegOrVal::Value(n),
            Err(_) => match value.len() {
                1 => RegOrVal::Register(value.chars().nth(0).unwrap()),
                _ => panic!("str is wrong length"),
            },
        }
    }
}

type Registers = HashMap<char, i64>;
type Sounds = Vec<i64>;

#[derive(PartialEq, Debug)]
enum Inst {
    Snd(RegOrVal),
    Set(RegOrVal, RegOrVal),
    Add(RegOrVal, RegOrVal),
    Mul(RegOrVal, RegOrVal),
    Mod(RegOrVal, RegOrVal),
    Rcv(RegOrVal),
    Jgz(RegOrVal, RegOrVal),
}

impl Inst {
    fn execute(instructions: Vec<Inst>) -> Option<i64> {
        let mut registers = Registers::new();
        let mut sounds = Sounds::new();
        let mut index = 0;

        while index < instructions.len() {
            match &instructions[index] {
                Inst::Snd(x) => sounds.push(x.get_value(&registers)),
                Inst::Set(x, y) => {
                    registers.insert(x.get_register(), y.get_value(&registers));
                }
                Inst::Add(x, y) => {
                    registers.insert(
                        x.get_register(),
                        x.get_value(&registers) + y.get_value(&registers),
                    );
                }
                Inst::Mul(x, y) => {
                    registers.insert(
                        x.get_register(),
                        x.get_value(&registers) * y.get_value(&registers),
                    );
                }
                Inst::Mod(x, y) => {
                    registers.insert(
                        x.get_register(),
                        x.get_value(&registers) % y.get_value(&registers),
                    );
                }
                Inst::Rcv(x) => {
                    if x.get_value(&registers) != 0 {
                        return sounds.pop();
                    }
                }
                Inst::Jgz(x, y) => {
                    if x.get_value(&registers) > 0 {
                        let dist = y.get_value(&registers);
                        let current = index as i64 + dist;
                        if current < 0 {
                            panic!("Program pointer out of range!");
                        }

                        index = current as usize;
                        continue;
                    }
                }
            };

            index += 1;
        }

        unreachable!();
    }
}

impl From<&str> for Inst {
    fn from(value: &str) -> Self {
        let mut args = value.get(4..).unwrap().split(' ');
        match value.get(0..=2) {
            Some("snd") => Inst::Snd(args.next().unwrap().into()),
            Some("set") => Inst::Set(args.next().unwrap().into(), args.next().unwrap().into()),
            Some("add") => Inst::Add(args.next().unwrap().into(), args.next().unwrap().into()),
            Some("mul") => Inst::Mul(args.next().unwrap().into(), args.next().unwrap().into()),
            Some("mod") => Inst::Mod(args.next().unwrap().into(), args.next().unwrap().into()),
            Some("rcv") => Inst::Rcv(args.next().unwrap().into()),
            Some("jgz") => Inst::Jgz(args.next().unwrap().into(), args.next().unwrap().into()),
            Some(x) => panic!("instrution '{}' not supported", x),
            None => todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";

    #[test]
    fn parse_test() {
        let mut it = parse(INPUT);

        assert_eq!(
            it.next().unwrap(),
            Inst::Set(RegOrVal::Register('a'), RegOrVal::Value(1))
        );
        assert_eq!(
            it.next().unwrap(),
            Inst::Add(RegOrVal::Register('a'), RegOrVal::Value(2))
        );
        assert_eq!(
            it.next().unwrap(),
            Inst::Mul(RegOrVal::Register('a'), RegOrVal::Register('a'))
        );
        assert_eq!(
            it.next().unwrap(),
            Inst::Mod(RegOrVal::Register('a'), RegOrVal::Value(5))
        );
        assert_eq!(it.next().unwrap(), Inst::Snd(RegOrVal::Register('a')));
        assert_eq!(
            it.next().unwrap(),
            Inst::Set(RegOrVal::Register('a'), RegOrVal::Value(0))
        );
        assert_eq!(it.next().unwrap(), Inst::Rcv(RegOrVal::Register('a')));
        assert_eq!(
            it.next().unwrap(),
            Inst::Jgz(RegOrVal::Register('a'), RegOrVal::Value(-1))
        );
        assert_eq!(
            it.next().unwrap(),
            Inst::Set(RegOrVal::Register('a'), RegOrVal::Value(1))
        );
        assert_eq!(
            it.next().unwrap(),
            Inst::Jgz(RegOrVal::Register('a'), RegOrVal::Value(-2))
        );
    }

    #[test]
    fn test_a() {
        assert_eq!(Day18 {}.solve_a(INPUT), Answer::Int(4));
    }
}
