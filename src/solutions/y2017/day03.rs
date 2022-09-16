use std::collections::HashMap;

use itertools::Itertools;

use crate::solutions::{Answer, Solution};

pub struct Day03 {}

impl Solution for Day03 {
    fn solve_a(&self, input: &str) -> Answer {
        let n: i64 = input.parse().unwrap();
        if n == 1 {
            return Answer::UInt(0);
        }

        let level = (1..)
            .step_by(2)
            .map(|x| x)
            .take_while(|x| *x * *x < n)
            .map(|x| x + 2)
            .last()
            .unwrap_or(1);

        (0..4)
            .map(|k| level * level - k * (level - 1))
            .map(|p| p.abs_diff(n) as i64)
            .filter(|dist| *dist <= ((level - 1) / 2))
            .map(|dist| level - 1 - dist)
            .map(|x| x as u64)
            .find_or_first(|_| true)
            .unwrap()
            .into()
    }

    fn solve_b(&self, input: &str) -> Answer {
        fn sum_of_neighbors(a: i32, b: i32, grid: &HashMap<(i32, i32), u32>) -> u32 {
            (-1..=1)
                .map(|x| (-1..=1).map(move |y| (x, y)))
                .flatten()
                .filter(|p| *p != (0, 0))
                .map(|(x, y)| grid.get(&(a + x, b + y)).unwrap_or(&0))
                .sum::<u32>()
        }

        let target: u32 = input.parse().unwrap();

        let mut grid: HashMap<(i32, i32), u32> = HashMap::new();
        grid.insert((0, 0), 1);

        let mut k = 1;
        let (mut x, mut y) = (0, 0);
        loop {
            for _ in 0..k {
                y += 1;
                let value = sum_of_neighbors(x, y, &grid);
                if value > target {
                    return value.into();
                }
                grid.insert((x, y), value);
            }
            for _ in 0..k {
                x -= 1;
                let value = sum_of_neighbors(x, y, &grid);
                if value > target {
                    return value.into();
                }
                grid.insert((x, y), value);
            }
            k += 1;
            for _ in 0..k {
                y -= 1;
                let value = sum_of_neighbors(x, y, &grid);
                if value > target {
                    return value.into();
                }
                grid.insert((x, y), value);
            }
            for _ in 0..k {
                x += 1;
                let value = sum_of_neighbors(x, y, &grid);
                if value > target {
                    return value.into();
                }
                grid.insert((x, y), value);
            }
            k += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::solutions::{Answer, Solution};

    use super::Day03;

    #[test]
    fn test_a() {
        assert_eq!(Day03 {}.solve_a("1"), Answer::UInt(0));
        assert_eq!(Day03 {}.solve_a("9"), Answer::UInt(2));
        assert_eq!(Day03 {}.solve_a("23"), Answer::UInt(2));
        assert_eq!(Day03 {}.solve_a("12"), Answer::UInt(3));
        assert_eq!(Day03 {}.solve_a("1024"), Answer::UInt(31));
    }

    #[test]
    fn test_b() {
        assert_eq!(Day03 {}.solve_b("121"), Answer::UInt(122))
    }
}
