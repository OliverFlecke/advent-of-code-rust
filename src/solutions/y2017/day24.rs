use std::{fmt::Display, str::FromStr};

use hashbag::HashBag;

use crate::solutions::{answer::Answer, Solution};

pub struct Day24;

impl Solution for Day24 {
    fn solve_a(&self, input: &str) -> Answer {
        let mut components = Port::to_bag(input);
        let mut bridges: Vec<Bridge> = Vec::new();

        Day24::build_bridges(&mut components, &mut Vec::new(), &mut bridges, 0);

        bridges.iter().map(|b| b.strength()).max().unwrap().into()
    }

    fn solve_b(&self, input: &str) -> Answer {
        let mut components = Port::to_bag(input);
        let mut bridges: Vec<Bridge> = Vec::new();

        Day24::build_bridges(&mut components, &mut Vec::new(), &mut bridges, 0);

        let longest = bridges.iter().max_by_key(|b| b.len()).unwrap().len();

        bridges
            .iter()
            .filter(|b| b.len() == longest)
            .map(|b| b.strength())
            .max()
            .unwrap()
            .into()
    }
}

impl Day24 {
    fn build_bridges(
        components: &mut HashBag<Port>,
        current: &mut Vec<Port>,
        answers: &mut Vec<Bridge>,
        from: usize,
    ) {
        answers.push(Bridge {
            ports: current.clone(),
        });

        for (port, _) in components.clone().into_iter() {
            if port.from == from {
                components.remove(&port);
                current.push(port);
                Self::build_bridges(components, current, answers, port.to);
                current.pop();
                components.insert(port);
            } else if port.to == from {
                components.remove(&port);
                current.push(port);
                Self::build_bridges(components, current, answers, port.from);
                current.pop();
                components.insert(port);
            }
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct Port {
    from: usize,
    to: usize,
}

impl FromStr for Port {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('/');
        Ok(Port {
            from: parts.next().unwrap().parse().unwrap(),
            to: parts.next().unwrap().parse().unwrap(),
        })
    }
}

impl Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.from, self.to)
    }
}

impl Port {
    fn to_bag(input: &str) -> HashBag<Port> {
        input
            .lines()
            .map(|l| l.parse().unwrap())
            .fold(HashBag::new(), |mut state, port| {
                state.insert(port);
                state
            })
    }
}

#[derive(Debug, PartialEq)]
struct Bridge {
    ports: Vec<Port>,
}

impl Bridge {
    fn strength(&self) -> usize {
        self.ports.iter().map(|p| p.from + p.to).sum()
    }

    fn len(&self) -> usize {
        self.ports.len()
    }
}

impl FromStr for Bridge {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Bridge {
            ports: s.split(" -- ").map(|p| p.parse().unwrap()).collect(),
        })
    }
}

impl From<Vec<Port>> for Bridge {
    fn from(ports: Vec<Port>) -> Self {
        Bridge {
            ports: ports.clone(),
        }
    }
}

impl Display for Bridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.ports
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join(" -- ")
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

    #[test]
    fn parse() {
        let mut parts = INPUT.lines().map(|l| l.parse::<Port>().unwrap());

        assert_eq!(parts.next().unwrap(), Port { from: 0, to: 2 });
        assert_eq!(parts.next().unwrap(), Port { from: 2, to: 2 });
        assert_eq!(parts.next().unwrap(), Port { from: 2, to: 3 });
        assert_eq!(parts.next().unwrap(), Port { from: 3, to: 4 });
        assert_eq!(parts.next().unwrap(), Port { from: 3, to: 5 });
        assert_eq!(parts.next().unwrap(), Port { from: 0, to: 1 });
        assert_eq!(parts.next().unwrap(), Port { from: 10, to: 1 });
        assert_eq!(parts.next().unwrap(), Port { from: 9, to: 10 });
        assert_eq!(parts.next(), None);
    }

    #[test]
    fn to_bag() {
        let bag = Port::to_bag(INPUT);
        assert_eq!(bag.len(), 8);

        assert_eq!(bag.contains(&Port { from: 0, to: 2 }), 1);
        assert_eq!(bag.contains(&Port { from: 2, to: 2 }), 1);
        assert_eq!(bag.contains(&Port { from: 2, to: 3 }), 1);
        assert_eq!(bag.contains(&Port { from: 3, to: 4 }), 1);
        assert_eq!(bag.contains(&Port { from: 3, to: 5 }), 1);
        assert_eq!(bag.contains(&Port { from: 0, to: 1 }), 1);
        assert_eq!(bag.contains(&Port { from: 10, to: 1 }), 1);
        assert_eq!(bag.contains(&Port { from: 9, to: 10 }), 1);
    }

    #[test]
    fn build_bridges() {
        let mut components = Port::to_bag(INPUT);
        let mut bridges: Vec<Bridge> = Vec::new();

        Day24::build_bridges(&mut components, &mut Vec::new(), &mut bridges, 0);

        let bridges_as_str = "0/2
0/2 -- 2/2
0/2 -- 2/2 -- 2/3
0/2 -- 2/2 -- 2/3 -- 3/4
0/2 -- 2/2 -- 2/3 -- 3/5
0/2 -- 2/3
0/2 -- 2/3 -- 3/4
0/2 -- 2/3 -- 3/5
0/1
0/1 -- 10/1
0/1 -- 10/1 -- 9/10";

        for line in bridges_as_str.lines() {
            assert!(bridges.contains(&line.parse().unwrap()));
        }
    }

    #[test]
    fn test_a() {
        assert_eq!(Day24 {}.solve_a(INPUT), Answer::UInt(31));
    }

    #[test]
    fn test_b() {
        assert_eq!(Day24 {}.solve_b(INPUT), Answer::UInt(19));
    }
}
