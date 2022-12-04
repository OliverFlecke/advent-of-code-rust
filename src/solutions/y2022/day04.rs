use crate::solutions::{answer::Answer, Solution};

pub struct Day04;

impl Solution for Day04 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        Some(
            input
                .trim_end()
                .lines()
                .map(parse)
                .filter(|(a, b)| a.fully_contains(b) || b.fully_contains(a))
                .count()
                .into(),
        )
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        // This was just an experiment to see the time difference between doing both at once.
        // From this, I couldn't see any runtime difference between this and doing only part B,
        // which makes sense, as the only difference is a few extra comprisons and one addition.
        // But I plan to stick with two different methods, mainly so I can time and submit them independently.

        // let pairs = input.trim_end().lines().map(parse);
        // let mut contains = 0;
        // let mut overlaps = 0;
        // for (a, b) in pairs {
        //     if a.fully_contains(&b) || b.fully_contains(&a) {
        //         contains += 1;
        //     }
        //     if a.overlap(&b) {
        //         overlaps += 1
        //     }
        // }

        // Some(overlaps.into())

        Some(
            input
                .trim_end()
                .lines()
                .map(parse)
                .filter(|(a, b)| a.overlap(b))
                .count()
                .into(),
        )
    }
}

fn parse(line: &str) -> (Pair, Pair) {
    let mut split = line.split(',');

    (
        Pair::try_from(split.next().expect("first pair to be in line")).unwrap(),
        Pair::try_from(split.next().expect("second pair to be in line")).unwrap(),
    )
}

#[derive(Debug, Clone, Copy)]
struct Pair {
    start: u64,
    end: u64,
}

impl Pair {
    fn fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn overlap(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

impl TryFrom<&str> for Pair {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split = value.split('-');
        Ok(Self {
            start: split.next().unwrap().parse::<u64>().unwrap(),
            end: split.next().unwrap().parse::<u64>().unwrap(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_a() {
        assert_eq!(Day04 {}.solve_a(SAMPLE_INPUT), Some(Answer::UInt(2)))
    }

    #[test]
    fn test_b() {
        assert_eq!(Day04 {}.solve_b(SAMPLE_INPUT), Some(Answer::UInt(4)))
    }
}
