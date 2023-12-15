use crate::solutions::{answer::Answer, Solution};

pub struct Day15;

impl Solution for Day15 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        Some(input.trim().split(',').map(hash).sum::<usize>().into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let mut hashmap: Vec<Vec<Entry>> = vec![vec![]; 256];
        for line in input.trim().split(',') {
            if let Some((label, focal_length)) = line.split_once('=') {
                println!("focal length: {focal_length}");
                let focal_length = focal_length
                    .parse::<usize>()
                    .expect("Failed to parse focal length digit");
                let Some(entry) = hashmap.get_mut(hash(label)) else {
                    continue;
                };
                if let Some(position) = entry.iter().position(|x| x.label == label) {
                    entry.get_mut(position).unwrap().focal_length = focal_length;
                } else {
                    entry.push(Entry {
                        label,
                        focal_length,
                    });
                }
            } else if let Some((label, _)) = line.split_once('-') {
                let Some(entry) = hashmap.get_mut(hash(label)) else {
                    continue;
                };
                if let Some(position) = entry.iter().position(|x| x.label == label) {
                    entry.remove(position);
                }
            }

            println!("After {}", line);
            for (i, value) in hashmap.iter().enumerate().filter(|(_, x)| !x.is_empty()) {
                println!("Box {i}: {:?}", value);
            }
            println!();
        }

        let answer: usize = hashmap
            .iter()
            .enumerate()
            .map(|(b, values)| {
                values
                    .iter()
                    .enumerate()
                    .map(|(slot, value)| (b + 1) * (slot + 1) * value.focal_length)
                    .sum::<usize>()
            })
            .sum();
        Some(answer.into())
    }
}

#[derive(Debug, Clone)]
struct Entry<'a> {
    label: &'a str,
    focal_length: usize,
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

    // #[test]
    // fn solve_b() {
    //     let input = AocClient::default().get_input(PROBLEM).unwrap();
    //     assert_eq!(Day15 {}.solve_b(&input), Some(Answer::UInt(todo!())));
    // }
}
