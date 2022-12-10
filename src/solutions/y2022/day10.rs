use crate::solutions::{answer::Answer, Solution};

pub struct Day10;

impl Solution for Day10 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let commands = input.trim_end().lines().map(|x| Inst::try_from(x).unwrap());

        let mut sum = 0;
        let mut cycle = 0;
        let mut x = 1;
        for cmd in commands {
            for _ in 0..cmd.cycle_time() {
                cycle += 1;
                if cycle % 40 == 20 {
                    sum += x * cycle;
                }
            }
            match cmd {
                Inst::Addx(v) => x += v,
                Inst::Noop => {}
            }
        }

        Some(sum.into())
    }

    fn solve_b(&self, _input: &str) -> Option<Answer> {
        None
    }
}

#[derive(Debug, Clone, Copy)]
enum Inst {
    Addx(i32),
    Noop,
}

impl Inst {
    fn cycle_time(&self) -> usize {
        match self {
            Inst::Addx(_) => 2,
            Inst::Noop => 1,
        }
    }
}

impl TryFrom<&str> for Inst {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split(' ');

        match split.next() {
            Some("noop") => Ok(Inst::Noop),
            Some("addx") => Ok(Inst::Addx(
                split
                    .next()
                    .and_then(|x| x.parse::<i32>().ok())
                    .expect("there to be a number on the line"),
            )),
            Some(_) | None => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use std::{env, fs};

    use super::*;

    //     const SAMPLE_INPUT: &str = "noop
    // addx 3
    // addx -5";

    #[test]
    fn test_a() {
        let path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/samples/2022/10.0.txt";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(Day10.solve_a(input.as_str()), Some(Answer::Int(13140)));
    }
}
