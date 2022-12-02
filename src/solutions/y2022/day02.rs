use std::cmp::Ordering;

use crate::solutions::{answer::Answer, Solution};

pub struct Day02;

impl Solution for Day02 {
    fn solve_a(&self, input: &str) -> Answer {
        input
            .trim_end()
            .lines()
            .map(|l| {
                let mut it = l.split(' ').map(|x| Hand::from(x));
                (it.next().unwrap(), it.next().unwrap())
            })
            .map(|(other, me)| me.wins_over(&other).score() + me.score())
            .sum::<u64>()
            .into()
    }

    fn solve_b(&self, input: &str) -> Answer {
        input
            .trim_end()
            .lines()
            .map(|l| {
                let mut split = l.split(' ');
                (
                    Hand::from(split.next().unwrap()),
                    Outcome::from(split.next().unwrap()),
                )
            })
            .map(|(op, goal)| goal.score() + get_hand_to_play(goal, op).score())
            .sum::<u64>()
            .into()
    }
}

fn get_hand_to_play(goal: Outcome, opponent: Hand) -> Hand {
    match goal {
        Outcome::Draw => opponent,
        Outcome::Win => match opponent {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissor,
            Hand::Scissor => Hand::Rock,
        },
        Outcome::Loss => match opponent {
            Hand::Rock => Hand::Scissor,
            Hand::Paper => Hand::Rock,
            Hand::Scissor => Hand::Paper,
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Invalid output {s}"),
        }
    }
}

impl Outcome {
    fn score(&self) -> u64 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

impl From<Ordering> for Outcome {
    fn from(order: Ordering) -> Self {
        match order {
            Ordering::Less => Outcome::Loss,
            Ordering::Equal => Outcome::Draw,
            Ordering::Greater => Outcome::Win,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissor,
            _ => panic!("Unhandled character: '{}'", s),
        }
    }
}

impl Hand {
    fn score(&self) -> u64 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3,
        }
    }

    fn wins_over(&self, opponent: &Hand) -> Outcome {
        match self {
            Hand::Rock if *opponent == Hand::Scissor => Outcome::Win,
            Hand::Scissor if *opponent == Hand::Rock => Outcome::Loss,
            _ => self.cmp(opponent).into(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const SAMPLE_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_a() {
        assert_eq!(Day02 {}.solve_a(SAMPLE_INPUT), Answer::UInt(15));
    }

    #[test]
    fn test_b() {
        assert_eq!(Day02 {}.solve_b(SAMPLE_INPUT), Answer::UInt(12));
    }

    #[test]
    fn compare() {
        assert_eq!(Hand::Rock.wins_over(&Hand::Scissor), Outcome::Win);
        assert_eq!(Hand::Rock.wins_over(&Hand::Rock), Outcome::Draw);
        assert_eq!(Hand::Rock.wins_over(&Hand::Paper), Outcome::Loss);

        assert_eq!(Hand::Paper.wins_over(&Hand::Paper), Outcome::Draw);
        assert_eq!(Hand::Paper.wins_over(&Hand::Rock), Outcome::Win);
        assert_eq!(Hand::Paper.wins_over(&Hand::Scissor), Outcome::Loss);

        assert_eq!(Hand::Scissor.wins_over(&Hand::Scissor), Outcome::Draw);
        assert_eq!(Hand::Scissor.wins_over(&Hand::Rock), Outcome::Loss);
        assert_eq!(Hand::Scissor.wins_over(&Hand::Paper), Outcome::Win);
    }
}
