use advent_of_code_ocr::parse_string_to_letters;
use array2d::Array2D;

use crate::{
    solutions::{answer::Answer, Solution},
    utils::ocr::screen_to_string,
};

pub struct Day10;

impl Solution for Day10 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let commands = parse(input);

        let mut sum = 0;
        simulate(commands, CycleOrder::Before, |cycle, x| {
            if cycle % 40 == 20 {
                sum += x * cycle;
            }
        });

        Some(sum.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let mut screen = Array2D::filled_with(false, 6, 40);
        let commands = parse(input);

        simulate(commands, CycleOrder::After, |cycle, x| {
            let col = cycle % 40;
            if x.abs_diff(col) <= 1 {
                let row = cycle / 40;
                screen.set(row as usize, col as usize, true).unwrap();
            }
        });

        Some(parse_string_to_letters(screen_to_string(&screen).as_str()).into())
    }
}

#[derive(PartialEq)]
enum CycleOrder {
    Before,
    After,
}

fn simulate<F>(commands: Vec<Inst>, order: CycleOrder, mut fun: F)
where
    F: FnMut(i32, i32),
{
    let mut cycle = 0;
    let mut x = 1;
    for cmd in commands {
        for _ in 0..cmd.cycle_time() {
            if order == CycleOrder::Before {
                cycle += 1;
            }
            fun(cycle, x);
            if order == CycleOrder::After {
                cycle += 1;
            }
        }
        match cmd {
            Inst::Addx(v) => x += v,
            Inst::Noop => {}
        }
    }
}

fn parse(input: &str) -> Vec<Inst> {
    input
        .trim_end()
        .lines()
        .map(|x| Inst::try_from(x).unwrap())
        .collect()
}

#[allow(dead_code)]
fn print_screen(screen: &Array2D<bool>) {
    screen.rows_iter().for_each(|col| {
        col.for_each(|lit| if *lit { print!("#") } else { print!(" ") });
        println!();
    });
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
    use crate::{utils::load_sample, Year};

    use super::*;

    #[test]
    fn test_a() {
        let sample = load_sample(Year::Y2022, "10.txt").unwrap();
        assert_eq!(Day10.solve_a(sample.as_str()), Some(Answer::Int(13140)));
    }

    // Used to validate that the test input draws the screen as expected.
    // It doesn't really make sense to test the answer.
    // #[test]
    // fn test_b() {
    //     let path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/samples/2022/10.txt";
    //     let input = fs::read_to_string(path).unwrap();
    //     assert_eq!(Day10.solve_b(input.as_str()), Some(Answer::Int(13140)));
    // }
}
