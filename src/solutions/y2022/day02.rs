use std::cmp::Ordering;

use crate::solutions::{answer::Answer, Solution};

pub struct Day02;

impl Solution for Day02 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        Some(
            input
                .trim_end()
                .lines()
                .map(|l| {
                    let mut it = l
                        .split(' ')
                        .map(|x| Hand::try_from(x).expect("valid character"));
                    (it.next().unwrap(), it.next().unwrap())
                })
                .map(|(other, me)| me.wins_over(&other).score() + me.score())
                .sum::<u64>()
                .into(),
        )
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        Some(
            input
                .trim_end()
                .lines()
                .map(|l| {
                    let mut split = l.split(' ');
                    (
                        Hand::try_from(split.next().unwrap()).expect("valid hand character"),
                        Outcome::try_from(split.next().unwrap()).expect("to be valid outcome"),
                    )
                })
                .map(|(op, goal)| goal.score() + get_hand_to_play(goal, op).score())
                .sum::<u64>()
                .into(),
        )
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

impl TryFrom<&str> for Outcome {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(value.to_string()),
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

impl TryFrom<&str> for Hand {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissor),
            _ => Err(value.to_string()),
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
        assert_eq!(Day02 {}.solve_a(SAMPLE_INPUT), Some(Answer::UInt(15)));
    }

    #[test]
    fn test_b() {
        assert_eq!(Day02 {}.solve_b(SAMPLE_INPUT), Some(Answer::UInt(12)));
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
