use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

use crate::solutions::{answer::Answer, Solution};

pub struct Day07 {}

// struct Node {
//     pub name: String,
//     pub weight: u64,
//     pub children: Vec<Node>,
// }

// impl Node {
//     fn new(name: String, weight: u64) -> Node {
//         Node {
//             name,
//             weight,
//             children: Vec::new(),
//         }
//     }
// }

impl Solution for Day07 {
    fn solve_a(&self, input: &str) -> Answer {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        input.trim().split('\n').for_each(|line| {
            let re = Regex::new(r"(?P<name>\w+) \((?P<weight>\d+)\)( -> (?P<children>[\w ,]*))?")
                .expect("regex should always be valid");
            let caps = match re.captures(line) {
                Some(x) => x,
                None => panic!("Line did not match regex: {:?}", line),
            };

            let name = &caps["name"];
            // let weight = caps["weight"]
            //     .parse::<u64>()
            //     .expect("weight should be a number");

            let mut list: Vec<String> = Vec::new();
            if let Some(children) = &caps.name("children") {
                children
                    .as_str()
                    .split(',')
                    .map(|s| s.trim())
                    .for_each(|c| {
                        list.push(c.to_string());
                    });
            }
            map.insert(name.to_string(), list);

            // Node::new(name.to_string(), weight)
        });

        let mut names: HashSet<String> = map.keys().cloned().collect();

        for (_, children) in map {
            for v in children {
                names.remove(&v);
            }
        }

        names
            .iter()
            .find_or_first(|_| true)
            .expect("there should be only one element remaining")
            .to_string()
            .into()
    }

    fn solve_b(&self, _input: &str) -> Answer {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        assert_eq!(Day07 {}.solve_a(input), "tknk".into());
    }
}
