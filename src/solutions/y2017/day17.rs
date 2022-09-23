use crate::solutions::{answer::Answer, Solution};

pub struct Day17;

impl Solution for Day17 {
    fn solve_a(&self, input: &str) -> Answer {
        let times = 2017;
        let amount: usize = input.parse().unwrap();
        let mut buffer = vec![0];

        for size in 0..times {
            buffer.rotate_left(amount % (size + 1));
            buffer.push(size + 1);
        }

        buffer[0].into()
    }

    fn solve_b(&self, _input: &str) -> Answer {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(Day17 {}.solve_a("3"), Answer::UInt(638));
    }
}
