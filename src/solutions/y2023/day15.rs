use crate::solutions::{answer::Answer, Solution};

pub struct Day15;

impl Solution for Day15 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        Some(input.trim().split(',').map(hash).sum::<usize>().into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let mut hashmap = Hashmap::default();

        for instruction in input.trim().split(',') {
            let (label, action) = parse_label_and_action(instruction);

            match action {
                Action::Insert(focal_length) => {
                    hashmap.insert(label, focal_length);
                }
                Action::Remove => {
                    hashmap.remove(label);
                }
            };
        }

        let answer = hashmap.focusing_power();
        Some(answer.into())
    }
}

fn hash(s: &str) -> usize {
    let mut value: usize = 0;
    for c in s.chars() {
        value += c as usize;
        value *= 17;
        value %= 256;
    }

    value
}

struct Hashmap<'a>(Vec<Vec<Entry<'a>>>);

impl<'a> Default for Hashmap<'a> {
    fn default() -> Self {
        Self(vec![vec![]; 256])
    }
}

impl<'a> std::fmt::Display for Hashmap<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, value) in self.0.iter().enumerate().filter(|(_, x)| !x.is_empty()) {
            writeln!(f, "Box {i}: {:?}", value)?;
        }
        write!(f, "")
    }
}

impl<'a> Hashmap<'a> {
    fn insert(&mut self, label: &'a str, focal_length: usize) {
        let Some(entry) = self.0.get_mut(hash(label)) else {
            return;
        };

        if let Some(position) = entry.iter_mut().find(|x| x.label == label) {
            position.focal_length = focal_length;
        } else {
            entry.push(Entry {
                label,
                focal_length,
            });
        }
    }

    fn remove(&mut self, label: &'a str) {
        if let Some(entry) = self.0.get_mut(hash(label))
            && let Some(position) = entry.iter().position(|x| x.label == label)
        {
            entry.remove(position);
        }
    }

    fn focusing_power(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(b, values)| {
                values
                    .iter()
                    .enumerate()
                    .map(|(slot, value)| (b + 1) * (slot + 1) * value.focal_length)
                    .sum::<usize>()
            })
            .sum()
    }
}

enum Action {
    Insert(usize),
    Remove,
}

fn parse_label_and_action(instruction: &str) -> (&str, Action) {
    instruction
        .split_once('=')
        .map(|(label, focal)| (label, Action::Insert(focal.parse::<usize>().unwrap())))
        .or_else(|| {
            instruction
                .split_once('-')
                .map(|(label, _)| (label, Action::Remove))
        })
        .expect("Not a valid action")
}

#[derive(Debug, Clone)]
struct Entry<'a> {
    label: &'a str,
    focal_length: usize,
}

#[cfg(test)]
mod test {
    use advent_of_code_client::{AocClient, Problem, Year};
    use rstest::rstest;

    use super::*;

    const PROBLEM: Problem = Problem::new(Year::Y2023, 15);
    const INPUT: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[rstest]
    #[case("HASH", 52)]
    fn hash_str(#[case] s: &str, #[case] expected_hash_value: usize) {
        assert_eq!(hash(s), expected_hash_value);
    }

    #[test]
    fn test_a() {
        assert_eq!(Day15 {}.solve_a(INPUT), Some(Answer::UInt(1320)));
    }

    #[test]
    fn solve_a() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day15 {}.solve_a(&input), Some(Answer::UInt(513643)));
    }

    #[test]
    fn test_b() {
        assert_eq!(Day15 {}.solve_b(INPUT), Some(Answer::UInt(145)));
    }

    #[test]
    fn solve_b() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day15 {}.solve_b(&input), Some(Answer::UInt(265345)));
    }
}
