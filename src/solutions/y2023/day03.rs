use array2d::Array2D;
use itertools::Itertools;

use crate::solutions::{answer::Answer, Solution};

pub struct Day03;

impl Solution for Day03 {
    fn solve_a(&self, input: &str) -> Option<Answer> {}

    fn solve_b(&self, _input: &str) -> Option<Answer> {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::{client::get_input, Year};

    use super::*;

    #[test]
    fn test_a() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        assert_eq!(Day03 {}.solve_a(input), Some(Answer::UInt(4361)))
    }

    #[test]
    fn answer_a() {
        let input = get_input(Year::Y2023, 3).unwrap();
        assert_eq!(Day03 {}.solve_a(input.as_str()), Some(Answer::UInt(509115)))
    }
}
