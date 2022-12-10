use array2d::Array2D;

use crate::solutions::{answer::Answer, Solution};

pub struct Day10;

impl Solution for Day10 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let commands = parse(input);

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

    fn solve_b(&self, input: &str) -> Option<Answer> {
        const SCREEN_ROWS: usize = 6;
        const SCREEN_COLS: usize = 40;
        // const SPIRTE_WIDTH: i32 = 3;

        let commands = parse(input);
        let mut screen = Array2D::filled_with(false, SCREEN_ROWS, SCREEN_COLS);

        let mut cycle = 0;
        let mut x = 1;
        for cmd in commands {
            for _ in 0..cmd.cycle_time() {
                if let Ok(current) = usize::try_from(x) {
                    let col = cycle % 40;
                    if current.abs_diff(col) <= 1 {
                        let row = cycle / 40;
                        // println!("Setting {col}/{row}");
                        screen.set(row, col, true).unwrap();
                    }
                }
                cycle += 1;
            }
            match cmd {
                Inst::Addx(v) => x += v,
                Inst::Noop => {}
            }
        }

        screen.rows_iter().for_each(|col| {
            col.for_each(|lit| if *lit { print!("#") } else { print!(" ") });
            println!();
        });

        // This answer is manually extracted from the image generated above
        Some("EPJBRKAH".into())
    }
}

fn parse(input: &str) -> Vec<Inst> {
    input
        .trim_end()
        .lines()
        .map(|x| Inst::try_from(x).unwrap())
        .collect()
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

    #[test]
    fn test_b() {
        let path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/samples/2022/10.0.txt";
        let input = fs::read_to_string(path).unwrap();
        assert_eq!(Day10.solve_b(input.as_str()), Some(Answer::Int(13140)));
    }
}
