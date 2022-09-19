use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::solutions::{answer::Answer, Solution};

pub struct Day12 {}

type Relations = HashMap<usize, HashSet<usize>>;

impl Day12 {
    fn parse_input(input: &str) -> Relations {
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
            .fold(HashMap::new(), |mut map, (from, to)| {
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
                map
            })
    }

    fn find_group(map: &Relations, from: usize) -> HashSet<usize> {
        let mut group: HashSet<usize> = HashSet::new();
        let mut queue: Vec<usize> = Vec::new();
        queue.push(from);

        while let Some(current) = queue.pop() {
            if group.contains(&current) {
                continue;
            }
            group.insert(current);

            map.get(&current)
                .iter()
                .for_each(|ns| queue.append(&mut ns.iter().map(|n| *n).collect::<Vec<usize>>()));
        }

        group
    }
}

impl Solution for Day12 {
    fn solve_a(&self, input: &str) -> Answer {
        let map = Self::parse_input(input);
        let group = Self::find_group(&map, 0);

        group.len().into()
    }

    fn solve_b(&self, input: &str) -> Answer {
        let map = Self::parse_input(input);
        let mut nodes: HashSet<usize> = map.keys().cloned().collect();

        let mut count: usize = 0;
        while let Some(start) = nodes.iter().find_or_first(|_| true) {
            count += 1;
            let group = Self::find_group(&map, *start);
            nodes = nodes.difference(&group).cloned().collect();
        }

        count.into()
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

    #[test]
    fn test_b() {
        let input = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
        assert_eq!(Day12 {}.solve_b(input), Answer::UInt(2));
    }
}
