use crate::solutions::{answer::Answer, Solution};

pub struct Day12;

impl Solution for Day12 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let answer: usize = input.trim().lines().map(find_arrangements).sum();
        Some(answer.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        None
    }
}

fn find_arrangements(line: &str) -> usize {
    let (gears, pattern) = line.split_once(' ').unwrap();
    let unknown_indexes: Vec<usize> = gears
        .char_indices()
        .filter(|(_, c)| *c == '?')
        .map(|(i, _)| i)
        .collect();
    let pattern: Vec<usize> = pattern
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    // println!("Starting on {gears} with pattern: {pattern:?}");
    let mut gears = gears.to_string();
    unsafe {
        let pointer = gears.as_mut_ptr();
        helper(&pattern, &gears, pointer, &unknown_indexes)
    }
}

unsafe fn helper(pattern: &[usize], gears: &str, pointer: *mut u8, indexes: &[usize]) -> usize {
    if let Some(i) = indexes.first() {
        let mut sum = 0;
        for c in [b'.', b'#'] {
            let current = pointer.add(*i);
            let original = *current;
            *current = c;

            sum += helper(pattern, gears, pointer, &indexes[1..]);

            *current = original;
        }

        sum
    } else if is_valid(gears, pattern) {
        // println!("Found valid pattern: {}", gears);
        1
    } else {
        0
    }
}

fn is_valid(s: &str, pattern: &[usize]) -> bool {
    let groups: Vec<_> = s.split('.').map(|x| x.len()).filter(|x| *x != 0).collect();

    groups.len() == pattern.len() && groups.iter().zip(pattern.iter()).all(|(a, b)| a == b)
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
    fn find_arrangements_from_line() {
        assert_eq!(find_arrangements("???.### 1,1,3"), 1);
        assert_eq!(find_arrangements(".??..??...?##. 1,1,3"), 4);
        assert_eq!(find_arrangements("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(find_arrangements("????.#...#... 4,1,1"), 1);
        assert_eq!(find_arrangements("????.######..#####. 1,6,5"), 4);
        assert_eq!(find_arrangements("?###???????? 3,2,1"), 10);
    }

    #[test]
    fn solve_a() {
        let input = AocClient::default().get_input(PROBLEM).unwrap();
        assert_eq!(Day12 {}.solve_a(&input), Some(Answer::UInt(7599)));
    }

    // #[test]
    // fn test_b() {
    //     assert_eq!(Day12 {}.solve_b(INPUT), Some(Answer::UInt(todo!())));
    // }

    // #[test]
    // fn solve_b() {
    //     let input = AocClient::default().get_input(PROBLEM).unwrap();
    //     assert_eq!(Day12 {}.solve_b(&input), Some(Answer::UInt(todo!())));
    // }
}
