use std::{
    collections::{HashMap, VecDeque},
    iter::Map,
    str::Lines,
};

use crate::solutions::{answer::Answer, Solution};

pub struct Day18;

impl Solution for Day18 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        Some(Inst::run_a(parse(input).collect()).unwrap().into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        Some(Inst::run_b(parse(input).collect()).into())
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
                1 => RegOrVal::Register(value.chars().next().unwrap()),
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
    fn execute(&self, registers: &mut Registers, index: &mut usize) -> ThreadState {
        match self {
            Inst::Set(x, y) => {
                registers.insert(x.get_register(), y.get_value(registers));
            }
            Inst::Add(x, y) => {
                registers.insert(
                    x.get_register(),
                    x.get_value(registers) + y.get_value(registers),
                );
            }
            Inst::Mul(x, y) => {
                registers.insert(
                    x.get_register(),
                    x.get_value(registers) * y.get_value(registers),
                );
            }
            Inst::Mod(x, y) => {
                registers.insert(
                    x.get_register(),
                    x.get_value(registers) % y.get_value(registers),
                );
            }
            Inst::Jgz(x, y) => {
                if x.get_value(registers) > 0 {
                    let dist = y.get_value(registers);
                    let current = *index as i64 + dist;
                    if current < 0 {
                        return ThreadState::Finished;
                    }

                    *index = current as usize;
                    return ThreadState::Ready;
                }
            }
            _ => unimplemented!(
                "execute not supported and must be implemented yourself: {:?}",
                self
            ),
        };

        *index += 1;
        ThreadState::Ready
    }

    fn run_a(instructions: Vec<Inst>) -> Option<i64> {
        let mut registers = Registers::new();
        let mut sounds = Sounds::new();
        let mut index = 0;

        while index < instructions.len() {
            match &instructions[index] {
                Inst::Snd(x) => {
                    sounds.push(x.get_value(&registers));
                    index += 1;
                }
                Inst::Rcv(x) => {
                    if x.get_value(&registers) != 0 {
                        return sounds.pop();
                    }
                    index += 1;
                }
                inst => match inst.execute(&mut registers, &mut index) {
                    ThreadState::Ready => continue,
                    _ => break,
                },
            };
        }

        unreachable!();
    }

    fn run_b(instructions: Vec<Inst>) -> u64 {
        let mut a_context = ThreadContext::new(0);
        let mut b_context = ThreadContext::new(1);

        fn run(
            context: &mut ThreadContext,
            other: &mut ThreadContext,
            instructions: &Vec<Inst>,
        ) -> ThreadState {
            while context.index < instructions.len() {
                match &instructions[context.index] {
                    Inst::Snd(x) => {
                        other.queue.push_back(x.get_value(&context.registers));
                        context.send_count += 1;
                        context.index += 1;
                    }
                    Inst::Rcv(x) => {
                        if let Some(value) = context.queue.pop_front() {
                            context.registers.insert(x.get_register(), value);
                            context.index += 1;
                        } else {
                            return ThreadState::Waiting;
                        }
                    }
                    inst => {
                        match inst.execute(&mut context.registers, &mut context.index) {
                            ThreadState::Ready => continue,
                            state => return state,
                        };
                    }
                };
            }

            ThreadState::Finished
        }

        while a_context.can_run() || b_context.can_run() {
            // println!("Switching to A {:?}", a_context);
            a_context.state = run(&mut a_context, &mut b_context, &instructions);
            // println!("Switching to B {:?}", b_context);
            b_context.state = run(&mut b_context, &mut a_context, &instructions);
        }

        b_context.send_count
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

#[derive(PartialEq, Debug)]
enum ThreadState {
    Ready,
    Waiting,
    Finished,
}

#[derive(Debug)]
struct ThreadContext {
    state: ThreadState,
    registers: Registers,
    queue: VecDeque<i64>,
    index: usize,
    send_count: u64,
}

impl ThreadContext {
    fn new(thread_id: i64) -> Self {
        let mut registers = Registers::new();
        registers.insert('p', thread_id);

        ThreadContext {
            state: ThreadState::Ready,
            registers,
            queue: VecDeque::new(),
            index: 0,
            send_count: 0,
        }
    }

    fn can_run(&self) -> bool {
        self.state == ThreadState::Ready
            || (self.state == ThreadState::Waiting && !self.queue.is_empty())
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
        assert_eq!(Day18 {}.solve_a(INPUT), Some(Answer::Int(4)))
    }

    #[test]
    fn test_b() {
        let input = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d";
        assert_eq!(Day18 {}.solve_b(input), Some(Answer::UInt(3)))
    }
}
