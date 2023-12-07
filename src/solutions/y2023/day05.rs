use rangemap::RangeMap;
use regex::Regex;
use std::{collections::HashMap, ops::Range};

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
        let mapping = create_mapping(parts);

        let answer = seeds
            .iter()
            .map(|s| to_location(&mapping, *s))
            .min()
            .unwrap();

        Some(answer.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let mut parts = input.split("\n\n");
        let seeds = parts
            .next()
            .and_then(|l| l.strip_prefix("seeds: "))
            .map(|numbers| {
                numbers
                    .split_whitespace()
                    .filter_map(|x| x.parse::<isize>().ok())
                    .collect::<Vec<_>>()
                    .chunks(2)
                    .map(|v| (v[0]..v[0] + v[1]))
                    .collect::<Vec<_>>()
            })
            .unwrap();

        let answer = parse(parts)
            .iter()
            .fold(seeds, map_range)
            .iter()
            .map(|range| range.start)
            .min()
            .unwrap();

        Some(answer.into())
    }
}

fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<RangeMap<isize, isize>> {
    let re = Regex::new(r"(?<distination>\d+) (?<source>\d+) (?<length>\d+)").unwrap();

    lines
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| {
                    let m = re.captures(line).unwrap();
                    let source = m["source"].parse::<isize>().unwrap();
                    let destination = m["distination"].parse::<isize>().unwrap();
                    let length = m["length"].parse::<isize>().unwrap();

                    (source..source + length, destination - source)
                })
                .collect::<RangeMap<_, _>>()
        })
        .collect()
}

fn map_range(mut inputs: Vec<Range<isize>>, map: &RangeMap<isize, isize>) -> Vec<Range<isize>> {
    let mut output = Vec::new();
    while let Some(input) = inputs.pop() {
        if map.overlaps(&input) {
            for (range, offset) in map.overlapping(&input) {
                let start = input.start.max(range.start);
                let end = input.end.min(range.end);

                output.push(start + offset..end + offset);
                if input.start < start {
                    inputs.push(input.start..start);
                }
                if end < input.end {
                    inputs.push(end..input.end);
                }
            }
        } else {
            output.push(input);
        }
    }
    output
}

// Naive implementation for part A.
type Map<'a> = HashMap<&'a str, Converter>;

fn to_location(mapping: &Map, seed: usize) -> usize {
    let mut current = "seed";
    let mut value = seed;
    while current != "location" {
        let converter = mapping.get(current).unwrap();
        value = converter.convert(value);
        current = converter.to.as_str();
    }

    value
}

fn create_mapping<'a>(parts: impl Iterator<Item = &'a str>) -> Map<'a> {
    let re = Regex::new(r"(?<distination>\d+) (?<source>\d+) (?<length>\d+)").unwrap();

    parts
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
        .collect()
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
    use crate::{client::AocClient, Year};

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

    #[test]
    fn test_b() {
        assert_eq!(Day05 {}.solve_b(INPUT), Some(Answer::Int(46)));
    }

    #[test]
    fn answer_b() {
        let input = AocClient::default().get_input(Year::Y2023, 5).unwrap();
        assert_eq!(
            Day05 {}.solve_b(input.as_str()),
            Some(Answer::Int(41222968))
        );
    }
}
