use std::collections::{HashMap, HashSet};

use crate::solutions::{answer::Answer, Solution};

pub struct Day12 {}

impl Solution for Day12 {
    fn solve_a(&self, input: &str) -> Answer {
        let mut map = HashMap::new();
        input
            .lines()
            .map(|line| {
                let mut parts = line.split(" <-> ");
                let from: usize = parts.next().unwrap().parse().unwrap();
                let to = parts
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|x| x.trim().parse::<usize>().unwrap());
                (from, to)
            })
            .for_each(|(from, to)| {
                if !map.contains_key(&from) {
                    map.insert(from, HashSet::new());
                }

                for n in to {
                    map.get_mut(&from).unwrap().insert(n);
                    if !map.contains_key(&n) {
                        map.insert(n, HashSet::new());
                    }
                    map.get_mut(&n).unwrap().insert(from);
                }
            });

        let mut visited: HashSet<usize> = HashSet::new();
        let mut queue: Vec<usize> = Vec::new();
        queue.push(0);

        while let Some(current) = queue.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            map
                .get(&current)
                .iter()
                .for_each(|ns| queue.append(&mut ns.iter().map(|n| *n).collect::<Vec<usize>>()));
        }

        visited.len().into()
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
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        assert_eq!(Day12 {}.solve_a(input), Answer::UInt(6));
    }
}
