use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use regex::Regex;

use crate::solutions::{answer::Answer, Solution};

pub struct Day07 {}

impl Solution for Day07 {
    fn solve_a(&self, input: &str) -> Answer {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let re = Regex::new(r"(?P<name>\w+) \((?P<weight>\d+)\)( -> (?P<children>[\w ,]*))?")
            .expect("regex should always be valid");

        input.trim().split('\n').for_each(|line| {
            match re.captures(line) {
                Some(caps) => map.insert(
                    caps["name"].to_string(),
                    caps.name("children")
                        .map(|cs| {
                            cs.as_str()
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .collect()
                        })
                        .unwrap_or_default(),
                ),
                None => panic!("Line did not match regex: {:?}", line),
            };
        });

        let mut names: HashSet<String> = map.keys().cloned().collect();

        for (_, children) in map {
            for v in children {
                names.remove(&v);
            }
        }

        names
            .iter()
            .find(|_| true)
            .expect("there should be only one element remaining")
            .to_string()
            .into()
    }

    fn solve_b(&self, input: &str) -> Answer {
        fn parse_line(line: &str) -> (String, u64, Vec<String>) {
            let re: regex::Regex =
                Regex::new(r"(?P<name>\w+) \((?P<weight>\d+)\)( -> (?P<children>[\w ,]*))?")
                    .expect("regex should always be valid");
            match re.captures(line) {
                Some(caps) => {
                    let name = &caps["name"].to_string();
                    let weight = caps["weight"]
                        .parse::<u64>()
                        .expect("weight should be a number");
                    let children = caps
                        .name("children")
                        .map(|cs| cs.as_str().split(',').map(|s| s.trim().into()).collect())
                        .unwrap_or_default();

                    (name.into(), weight, children)
                }
                None => panic!("Line did not match regex: {:?}", line),
            }
        }

        fn build_tree<'a>(items: &[&str]) -> Rc<RefCell<Node>> {
            let nodes = items
                .iter()
                .map(|l| {
                    let item = parse_line(l);
                    (item.0.clone(), item)
                })
                .collect::<HashMap<_, _>>();

            let head = find_root(items);
            make_node(head, &nodes)
        }

        fn make_node(
            head: String,
            nodes: &HashMap<String, (String, u64, Vec<String>)>,
        ) -> Rc<RefCell<Node>> {
            let (name, weight, cs) = nodes.get(&head).unwrap();
            let children: Vec<_> = cs.iter().map(|c| make_node(c.clone(), nodes)).collect();
            let calculated_weight = children
                .iter()
                .fold(*weight, |sum, c| sum + c.borrow().calculated_weight);

            Rc::new(RefCell::new(Node {
                name: name.clone(),
                weight: *weight,
                children,
                calculated_weight,
            }))
        }

        fn find_root(items: &[&str]) -> String {
            let mut children = HashSet::new();
            let mut nodes = HashSet::new();

            items
                .iter()
                .map(|line| parse_line(line))
                .for_each(|(name, _, cs)| {
                    nodes.insert(name);
                    cs.iter().for_each(|c| {
                        children.insert(c.clone());
                    });
                });

            nodes.difference(&children).next().unwrap().to_owned()
        }

        fn find_imbalance(node: &Rc<RefCell<Node>>) -> Option<u64> {
            let mut weights_on_level = HashMap::new();
            for child in &node.borrow().children {
                if let Some(prev) = weights_on_level.insert(child.borrow().calculated_weight, 1) {
                    weights_on_level.insert(child.borrow().calculated_weight, prev + 1);
                }
            }

            if weights_on_level.len() > 1 {
                let uneven_child = node
                    .borrow()
                    .children
                    .iter()
                    .find(|c| weights_on_level.get(&c.borrow().calculated_weight) == Some(&1))
                    .map(|c| c.clone())
                    .unwrap();

                let uneven_weight = uneven_child.borrow().calculated_weight;

                // Check if any of the children is the problem
                for child in &uneven_child.borrow().children {
                    if let Some(result) = find_imbalance(&child.clone()) {
                        return Some(result);
                    }
                }

                // Otherwise - this node must be the problem
                let sibling_weight = *weights_on_level
                    .keys()
                    .find(|w| **w != uneven_weight)
                    .unwrap();

                if uneven_weight > sibling_weight {
                    Some(uneven_child.borrow().weight - (uneven_weight - sibling_weight))
                } else {
                    Some(uneven_child.borrow().weight + (sibling_weight - uneven_weight))
                }
            } else {
                None
            }
        }

        let tree = build_tree(input.trim().split('\n').collect::<Vec<_>>().as_slice());
        find_imbalance(&tree).expect("no imbalance found").into()
    }
}

struct Node {
    #[allow(dead_code)]
    name: String,
    weight: u64,
    children: Vec<Rc<RefCell<Node>>>,
    calculated_weight: u64,
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "pbga (66)
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

    #[test]
    fn test_a() {
        assert_eq!(Day07 {}.solve_a(INPUT), "tknk".into());
    }

    #[test]
    fn test_b() {
        assert_eq!(Day07 {}.solve_b(INPUT), Answer::UInt(60));
    }
}
