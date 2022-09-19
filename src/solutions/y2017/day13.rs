use std::collections::HashMap;

use crate::solutions::{answer::Answer, Solution};

pub struct Day13 {}

impl Day13 {
    fn parse_firewall(input: &str) -> HashMap<usize, SecurityScanner> {
        input
            .lines()
            .map(|x| {
                let mut parts = x.split(':');
                let depth = parts.next().unwrap().parse().unwrap();
                (
                    depth,
                    SecurityScanner::new(depth, parts.next().unwrap().trim().parse().unwrap()),
                )
            })
            .collect()
    }

    fn calculate_severity(
        firewall: &HashMap<usize, SecurityScanner>,
        delay: usize,
    ) -> (bool, usize) {
        let max_depth = firewall.values().max_by_key(|x| x.depth).unwrap().depth;
        let mut position = 0;
        let mut severity: usize = 0;
        let mut caught = false;

        while position <= max_depth {
            // Increment severity if caught
            if let Some(scanner) = firewall.get(&position).filter(|s| s.caught(delay)) {
                severity += scanner.range * position;
                caught = true;
            }

            position += 1;
        }

        (caught, severity)
    }
}

#[derive(Clone)]
struct SecurityScanner {
    depth: usize,
    range: usize,
}

impl SecurityScanner {
    fn new(depth: usize, range: usize) -> Self {
        SecurityScanner { depth, range }
    }

    fn caught(&self, delay: usize) -> bool {
        (delay + self.depth) % (2 * (self.range - 1)) == 0
    }
}

impl Solution for Day13 {
    fn solve_a(&self, input: &str) -> Answer {
        let firewall = Self::parse_firewall(input);

        Self::calculate_severity(&firewall, 0).1.into()
    }

    // Bruteforcing, quite slow...
    fn solve_b(&self, input: &str) -> Answer {
        let firewall = Self::parse_firewall(input);
        let mut delay: usize = 1;

        loop {
            if firewall.values().find(|s| s.caught(delay)).is_none() {
                return delay.into();
            }

            delay += 1;
        }
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

    #[test]
    fn test_b() {
        assert_eq!(Day13 {}.solve_b(INPUT), Answer::UInt(10));
    }
}
