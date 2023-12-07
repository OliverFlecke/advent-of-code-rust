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
                hand: hand.to_string(),
                kind: hand.into(),
                bid: bid.parse::<usize>().unwrap(),
            })
            .sorted_by(Ord::cmp)
            .enumerate()
            .map(|(rank, card)| (rank + 1) * card.bid)
            .sum::<usize>();

        Some(answer.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let answer = input
            .trim()
            .lines()
            .filter_map(|line| line.split_once(' '))
            .map(|(hand, bid)| Card {
                hand: hand
                    .chars()
                    // Bit of a hack. To reuse the same `Ord` implementation below,
                    // we will just convert J into a different symbol.
                    .map(|c| if c == 'J' { 'X' } else { c })
                    .collect(),
                kind: from_str_with_joker(hand),
                bid: bid.parse::<usize>().unwrap(),
            })
            .sorted_by(Ord::cmp)
            .enumerate()
            .map(|(rank, card)| (rank + 1) * card.bid)
            .sum::<usize>();

        Some(answer.into())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Card {
    hand: String,
    kind: CardType,
    bid: usize,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
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
    Joker,
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
            'X' => Joker,
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

        let max = *map.values().max().unwrap_or(&0);

        use CardType::*;
        match max {
            5 => FiveOfAKind,
            4 => FourOfAKind,
            3 if map.values().any(|x| *x == 2) => FullHouse,
            3 => ThreeOfAKind,
            2 if map.values().filter(|x| **x == 2).count() == 2 => TwoPair,
            2 => OnePair,
            1 => HighCard,

            _ => unreachable!(),
        }
    }
}

fn from_str_with_joker(value: &str) -> CardType {
    let map = value.chars().fold(HashMap::new(), |mut map, c| {
        *map.entry(c).or_insert(0) += 1;
        map
    });

    let jokers = *map.get(&'J').unwrap_or(&0);
    let max = *map
        .iter()
        .filter(|(key, _)| **key != 'J')
        .map(|(_, value)| value)
        .max()
        .unwrap_or(&0);

    let two_pairs = map.values().filter(|x| **x == 2).count() == 2;

    use CardType::*;
    match (max, jokers) {
        (5, _) | (4, 1) | (3, 2) | (2, 3) | (1, 4) | (0, 5) => FiveOfAKind,
        (4, 0) | (3, 1) | (2, 2) | (1, 3) | (0, 4) => FourOfAKind,
        (3, 0) if map.values().any(|x| *x == 2) => FullHouse,
        (3, 0) => ThreeOfAKind,
        (2, 1) if two_pairs => FullHouse,
        (2, 1) => ThreeOfAKind,
        (2, 0) if two_pairs => TwoPair,
        (2, 0) => OnePair,
        (1, 2) => ThreeOfAKind,
        (1, 1) => OnePair,
        (1, 0) => HighCard,
        _ => unreachable!("Got {max}, {jokers}"),
    }
}

#[cfg(test)]
mod test {
    use crate::{client::get_input, Year};

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

    #[test]
    fn solve_a() {
        let input = get_input(Year::Y2023, 7).unwrap();
        assert_eq!(Day07 {}.solve_a(&input), Some(Answer::UInt(249748283)));
    }

    #[test]
    fn test_b() {
        assert_eq!(Day07 {}.solve_b(INPUT), Some(Answer::UInt(5905)));
    }

    #[test]
    fn solve_b() {
        let input = get_input(Year::Y2023, 7).unwrap();
        assert_eq!(Day07 {}.solve_b(&input), Some(Answer::UInt(248029057)));
    }
}
