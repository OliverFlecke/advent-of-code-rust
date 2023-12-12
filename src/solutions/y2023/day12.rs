use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::solutions::{answer::Answer, Solution};

pub struct Day12;

impl Solution for Day12 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let answer: usize = input
            .trim()
            .lines()
            .map(|line| {
                let (gears, pattern) = line.split_once(' ').unwrap();
                let pattern: Vec<usize> = pattern
                    .split(',')
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect();
                let mut cache = Cache::default();

                possible_ways(&mut cache, gears.as_bytes(), None, &pattern)
            })
            .sum();
        Some(answer.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let answer: usize = input
            .trim()
            .lines()
            .map(|line| {
                let (gears, pattern) = line.split_once(' ').unwrap();
                let pattern: Vec<usize> = pattern
                    .split(',')
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect();
                let gears = (0..5).map(|_| gears).join("?");
                let pattern: Vec<usize> = (0..5).flat_map(|_| &pattern).copied().collect();
                let mut cache = Cache::default();

                possible_ways(&mut cache, gears.as_bytes(), None, &pattern)
            })
            .sum();
        Some(answer.into())
    }
}

type Cache = FxHashMap<(usize, usize, usize), usize>;

fn possible_ways(cache: &mut Cache, s: &[u8], within: Option<usize>, rest: &[usize]) -> usize {
    if s.is_empty() {
        return match (within, rest.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == rest[0] => 1,
            _ => 0,
        };
    }

    if within.is_some() && rest.is_empty() {
        return 0;
    }

    let key = (s.len(), within.unwrap_or(0), rest.len());
    if let Some(&ways) = cache.get(&key) {
        return ways;
    }

    let ways = match (s[0], within) {
        (b'.', Some(x)) if x != rest[0] => 0,
        (b'.', Some(_)) => possible_ways(cache, &s[1..], None, &rest[1..]),
        (b'.', None) => possible_ways(cache, &s[1..], None, rest),
        (b'#', _) => possible_ways(cache, &s[1..], within.or(Some(0)).map(|x| x + 1), rest),
        (b'?', Some(x)) => {
            let mut sum = possible_ways(cache, &s[1..], within.map(|x| x + 1), rest);
            if x == rest[0] {
                sum += possible_ways(cache, &s[1..], None, &rest[1..]);
            }
            sum
        }
        (b'?', None) => {
            possible_ways(cache, &s[1..], Some(1), rest) + possible_ways(cache, &s[1..], None, rest)
        }
        _ => unreachable!(),
    };

    _ = cache.insert(key, ways);
    ways
}

#[cfg(test)]
mod test {
    use advent_of_code_client::{AocClient, Problem, Year};

    use super::*;

    const PROBLEM: Problem = Problem::new(Year::Y2023, 12);
    const INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn test_a() {
        assert_eq!(Day12 {}.solve_a(INPUT), Some(Answer::UInt(21)));
    }

    #[test]
    fn solve_a() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day12 {}.solve_a(&input), Some(Answer::UInt(7599)));
    }

    #[test]
    fn test_b() {
        assert_eq!(Day12 {}.solve_b(INPUT), Some(Answer::UInt(525152)));
    }

    #[test]
    fn solve_b() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day12 {}.solve_b(&input), Some(Answer::UInt(15454556629917)));
    }
}
