use std::collections::HashMap;

use crate::solutions::{answer::Answer, Solution};

pub struct Day13 {}

enum ScannerDirection {
    Up,
    Down,
}
struct SecurityScanner {
    #[allow(dead_code)]
    depth: usize,
    range: usize,
    position: usize,
    direction: ScannerDirection,
}

impl SecurityScanner {
    fn new(depth: usize, range: usize) -> Self {
        SecurityScanner {
            depth,
            range,
            position: 0,
            direction: ScannerDirection::Up,
        }
    }
    fn step(&mut self) {
        match self.direction {
            ScannerDirection::Up if self.position == self.range - 1 => {
                self.direction = ScannerDirection::Down;
                self.position -= 1;
            }
            ScannerDirection::Down if self.position == 0 => {
                self.direction = ScannerDirection::Up;
                self.position += 1;
            }
            ScannerDirection::Up => self.position += 1,
            ScannerDirection::Down => self.position -= 1,
        };
    }
}

impl Solution for Day13 {
    fn solve_a(&self, input: &str) -> Answer {
        let mut firewall: HashMap<usize, SecurityScanner> = input
            .lines()
            .map(|x| {
                let mut parts = x.split(':');
                let depth = parts.next().unwrap().parse().unwrap();
                (
                    depth,
                    SecurityScanner::new(depth, parts.next().unwrap().trim().parse().unwrap()),
                )
            })
            .collect();

        let max_depth = *firewall.keys().max().unwrap();
        let mut position: usize = 0;
        let mut severity: usize = 0;

        while position <= max_depth {
            // Increment severity if caught
            if let Some(scanner) = firewall.get(&position).filter(|s| s.position == 0) {
                severity += scanner.range * position;
            }

            firewall.values_mut().for_each(|x| {
                x.step();
            });
            position += 1;
        }

        severity.into()
    }

    fn solve_b(&self, input: &str) -> Answer {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "0: 3
1: 2
4: 4
6: 4";

    #[test]
    fn test_a() {
        assert_eq!(Day13 {}.solve_a(INPUT), Answer::UInt(24));
    }
}
