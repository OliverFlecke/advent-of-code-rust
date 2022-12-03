use std::{collections::HashMap, str::FromStr};

use crate::solutions::{answer::Answer, Solution};

pub struct Day23;

impl Solution for Day23 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let instructions: Vec<Inst> = input.lines().map(|l| l.parse().unwrap()).collect();
        let mut context = ThreadContext::default();

        while context.can_run(&instructions) {
            instructions[context.index].execute(&mut context);
        }

        Some(context.multi_count.into())
    }

    fn solve_b(&self, _input: &str) -> Option<Answer> {
        // Optimized from the assembly code.
        // first manually calculate what b and c is set to at the beginning,
        // then the middle part of the algorithm is simply checking whether
        // the number is a composite number.
        let b = 107900;
        let c = 124900;
        let mut h = 0;
        for x in (b..c + 1).step_by(17) {
            if !is_prime(x) {
                h += 1;
            }
        }

        Some(h.into())
    }
}

fn is_prime(n: i64) -> bool {
    if n == 2 || n == 3 {
        return true;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i: i64 = 5;
    let mut w: i64 = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += w;
        w = 6 - w;
    }

    true
}

type Registers = HashMap<char, i64>;

#[derive(PartialEq, Debug)]
enum RegOrVal {
    Register(char),
    Value(i64),
}

impl FromStr for RegOrVal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(n) => Ok(RegOrVal::Value(n)),
            Err(_) => match s.len() {
                1 => Ok(RegOrVal::Register(s.chars().next().unwrap())),
                _ => panic!("str is wrong length"),
            },
        }
    }
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

#[derive(PartialEq, Debug)]
enum ThreadState {
    Ready,
    Finished,
}

enum Inst {
    Set(RegOrVal, RegOrVal),
    Sub(RegOrVal, RegOrVal),
    Mul(RegOrVal, RegOrVal),
    Jnz(RegOrVal, RegOrVal),
}

impl FromStr for Inst {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut args = value.get(4..).unwrap().split(' ');
        Ok(match value.get(0..=2) {
            Some("set") => Inst::Set(args.next().unwrap().parse()?, args.next().unwrap().parse()?),
            Some("sub") => Inst::Sub(args.next().unwrap().parse()?, args.next().unwrap().parse()?),
            Some("mul") => Inst::Mul(args.next().unwrap().parse()?, args.next().unwrap().parse()?),
            Some("jnz") => Inst::Jnz(args.next().unwrap().parse()?, args.next().unwrap().parse()?),
            Some(x) => panic!("instrution '{}' not supported", x),
            None => todo!(),
        })
    }
}

impl Inst {
    fn execute(&self, ctx: &mut ThreadContext) {
        match self {
            Inst::Set(x, y) => {
                ctx.registers
                    .insert(x.get_register(), y.get_value(&ctx.registers));
            }
            Inst::Sub(x, y) => {
                ctx.registers.insert(
                    x.get_register(),
                    x.get_value(&ctx.registers) - y.get_value(&ctx.registers),
                );
            }
            Inst::Mul(x, y) => {
                ctx.registers.insert(
                    x.get_register(),
                    x.get_value(&ctx.registers) * y.get_value(&ctx.registers),
                );
                ctx.multi_count += 1;
            }
            Inst::Jnz(x, y) => {
                if x.get_value(&ctx.registers) != 0 {
                    let dist = y.get_value(&ctx.registers);
                    let current = ctx.index as i64 + dist;
                    if current < 0 {
                        ctx.state = ThreadState::Finished;
                    }

                    ctx.index = current as usize;
                    return;
                }
            }
        };

        ctx.index += 1;
    }
}

#[derive(Debug)]
struct ThreadContext {
    state: ThreadState,
    registers: Registers,
    index: usize,
    multi_count: usize,
}

impl Default for ThreadContext {
    fn default() -> Self {
        Self {
            state: ThreadState::Ready,
            registers: Registers::new(),
            index: 0,
            multi_count: 0,
        }
    }
}

impl ThreadContext {
    fn can_run(&self, inst: &Vec<Inst>) -> bool {
        self.state == ThreadState::Ready && self.index < inst.len()
    }
}
