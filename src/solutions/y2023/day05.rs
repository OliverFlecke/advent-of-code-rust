use std::collections::HashMap;

use regex::Regex;

use crate::solutions::{answer::Answer, Solution};

pub struct Day05;

impl Solution for Day05 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let mut parts = input.split("\n\n");
        let seeds = parts
            .next()
            .and_then(|l| l.strip_prefix("seeds: "))
            .map(|numbers| {
                numbers
                    .split_whitespace()
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect::<Vec<_>>()
            })
            .unwrap();
        // println!("Parsed seeds => {seeds:?}");

        let re = Regex::new(r"(?<distination>\d+) (?<source>\d+) (?<length>\d+)").unwrap();

        let mapping: HashMap<_, _> = parts
            .map(|l| {
                let mut lines = l.lines();
                let (from, to) = lines
                    .next()
                    .and_then(|x| x.strip_suffix(" map:"))
                    .and_then(|l| l.split_once("-to-"))
                    .unwrap();

                let mapping: Vec<_> = lines
                    .map(|line| {
                        let m = re.captures(line).unwrap();
                        Mapping {
                            source: m["source"].parse().unwrap(),
                            distination: m["distination"].parse().unwrap(),
                            range: m["length"].parse().unwrap(),
                        }
                    })
                    .collect();

                (
                    from,
                    Converter {
                        to: to.to_string(),
                        mapping,
                    },
                )
            })
            .collect();

        let to_location = |seed: usize| -> usize {
            // println!("\nStarting from new seed {seed}");
            let mut current = "seed";
            let mut value = seed;
            while current != "location" {
                let converter = mapping.get(current).unwrap();
                value = converter.convert(value);
                current = converter.to.as_str();
                // println!("{current} => {value}");
            }

            value
        };

        let answer = seeds.into_iter().map(to_location).min().unwrap();

        Some(answer.into())
    }

    fn solve_b(&self, _input: &str) -> Option<Answer> {
        None
    }
}

struct Converter {
    to: String,
    mapping: Vec<Mapping>,
}
impl Converter {
    fn convert(&self, value: usize) -> usize {
        self.mapping
            .iter()
            .find(|m| m.source <= value && value < m.source + m.range)
            .map(|m| (value as isize + m.diff()) as usize)
            .unwrap_or(value)
    }
}

#[derive(Debug)]
struct Mapping {
    source: usize,
    distination: usize,
    range: usize,
}

impl Mapping {
    fn diff(&self) -> isize {
        self.distination as isize - self.source as isize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    #[test]
    fn test_a() {
        assert_eq!(Day05 {}.solve_a(INPUT), Some(Answer::UInt(35)));
    }
}
