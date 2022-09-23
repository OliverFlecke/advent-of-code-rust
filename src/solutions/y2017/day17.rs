use crate::solutions::{answer::Answer, Solution};

pub struct Day17;

impl Solution for Day17 {
    fn solve_a(&self, input: &str) -> Answer {
        let times = 2017;
        let amount: usize = input.parse().unwrap();
        let mut buffer = vec![0];
        let mut index = 0;

        for size in 0..times {
            index = (index + amount) % (size + 1) + 1;
            buffer.insert(index, size + 1);
        }

        buffer[index + 1 % buffer.len()].into()
    }

    fn solve_b(&self, input: &str) -> Answer {
        let times = 50_000_000;
        let amount: usize = input.parse().unwrap();
        let mut next = 0;
        let mut answer = 0;
        for i in 1..times + 1 {
            next = (next + amount) % i;
            if next == 0 {
                answer = i;
            }
            next += 1;
        }

        answer.into()
    }
}

#[cfg(test)]
mod test {
    use advent_of_code::client::get_input;

    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(Day17 {}.solve_a("3"), Answer::UInt(638));
    }

    #[test]
    fn test_b() {
        assert_eq!(
            Day17 {}.solve_b(get_input(advent_of_code::Year::Y2017, 17).unwrap().as_str()),
            Answer::UInt(41797835)
        );
    }
}
