use crate::solutions::{answer::Answer, Solution};
use std::collections::HashSet;

pub struct Day04;

impl Solution for Day04 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let answer = input
            .lines()
            .map(play_round)
            .map(|count| 2_usize.pow((count - 1) as u32))
            .sum::<usize>();

        Some(answer.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let lines: Vec<_> = input.lines().collect();
        let mut counter = vec![1; lines.len()];

        lines
            .iter()
            .map(|l| play_round(l))
            .enumerate()
            .for_each(|(i, wins)| {
                let total_cards_of_this_type = *counter.get(i).unwrap();

                (i + 1..=i + wins)
                    .filter(|j| *j < lines.len())
                    .for_each(|j| {
                        counter[j] += total_cards_of_this_type;
                    });
            });

        let answer: usize = counter.iter().sum();

        Some(answer.into())
    }
}

fn play_round(line: &str) -> usize {
    line.split_once(':')
        .map(|(_, numbers)| numbers)
        .and_then(|numbers| numbers.split_once('|'))
        .map(|(winners, mine)| (to_set(winners), to_set(mine)))
        .map(|(w, m)| w.intersection(&m).collect::<HashSet<_>>().len())
        .filter(|x| *x != 0)
        .unwrap_or_default()
}

fn to_set(s: &str) -> HashSet<usize> {
    s.split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn part_a() {
        assert_eq!(Day04 {}.solve_a(INPUT), Some(Answer::UInt(13)));
    }

    #[test]
    fn part_b() {
        assert_eq!(Day04 {}.solve_b(INPUT), Some(Answer::UInt(30)))
    }
}
