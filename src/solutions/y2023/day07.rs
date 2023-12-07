use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

use crate::solutions::{answer::Answer, Solution};

pub struct Day07;

impl Solution for Day07 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let answer = input
            .trim()
            .lines()
            .filter_map(|line| line.split_once(' '))
            .map(|(hand, bid)| Card {
                hand,
                kind: hand.into(),
                bid: bid.parse::<usize>().unwrap(),
            })
            .sorted_by(Ord::cmp)
            .enumerate()
            .map(|(rank, card)| (rank + 1) * card.bid)
            .sum::<usize>();

        Some(answer.into())
    }

    fn solve_b(&self, _input: &str) -> Option<Answer> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Card<'a> {
    hand: &'a str,
    kind: CardType,
    bid: usize,
}

impl<'a> Ord for Card<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.kind
            .partial_cmp(&other.kind)
            .filter(|c| *c != Ordering::Equal)
            .or_else(|| {
                self.hand
                    .chars()
                    .zip(other.hand.chars())
                    .filter(|(a, b)| a != b)
                    .map(|(a, b)| (a.into(), b.into()))
                    .map(|(a, b): (Value, Value)| a.cmp(&b))
                    .next()
            })
            .unwrap_or(Ordering::Equal)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        use Value::*;

        match value {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Jack,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CardType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<&str> for CardType {
    fn from(value: &str) -> Self {
        let map = value.chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });

        let max = map.values().max().unwrap();

        match max {
            5 => CardType::FiveOfAKind,
            4 => CardType::FourOfAKind,
            3 => {
                if map.values().any(|x| *x == 2) {
                    CardType::FullHouse
                } else {
                    CardType::ThreeOfAKind
                }
            }
            2 => {
                if map.values().filter(|x| **x == 2).count() == 2 {
                    CardType::TwoPair
                } else {
                    CardType::OnePair
                }
            }

            1 => CardType::HighCard,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    // use crate::{client::get_input, Year};

    use super::*;

    const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    #[test]
    fn test_a() {
        assert_eq!(Day07 {}.solve_a(INPUT), Some(Answer::UInt(6440)));
    }

    // #[test]
    // fn solve_a() {
    //     let input = get_input(Year::Y2023, 7).unwrap();
    //     assert_eq!(Day07 {}.solve_a(&input), Some(Answer::UInt(todo!())));
    // }

    // #[test]
    // fn test_b() {
    //     assert_eq!(Day07 {}.solve_b(INPUT), Some(Answer::UInt(todo!())));
    // }

    // #[test]
    // fn solve_b() {
    //     let input = get_input(Year::Y2023, 7).unwrap();
    //     assert_eq!(Day07 {}.solve_b(&input), Some(Answer::UInt(todo!())));
    // }
}
