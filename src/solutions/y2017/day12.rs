use std::collections::{HashMap, HashSet};

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
                for n in to {
                    map.entry(from).or_default().insert(n);
                    map.entry(n).or_default().insert(from);
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
                .for_each(|ns| queue.append(&mut ns.iter().copied().collect::<Vec<usize>>()));
        }

        group
    }
}

impl Solution for Day12 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let map = Self::parse_input(input);
        let group = Self::find_group(&map, 0);

        Some(group.len().into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let map = Self::parse_input(input);
        let mut nodes: HashSet<usize> = map.keys().cloned().collect();

        let mut count: usize = 0;
        while let Some(start) = nodes.iter().find(|_| true) {
            count += 1;
            let group = Self::find_group(&map, *start);
            nodes = nodes.difference(&group).cloned().collect();
        }

        Some(count.into())
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

        assert_eq!(Day12 {}.solve_a(input), Some(Answer::UInt(6)));
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
        assert_eq!(Day12 {}.solve_b(input), Some(Answer::UInt(2)));
    }
}
