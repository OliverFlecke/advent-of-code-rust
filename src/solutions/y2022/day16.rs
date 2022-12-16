use std::{
    collections::{hash_map::Entry, HashMap},
    hash::Hash,
};

use crate::{
    solutions::{answer::Answer, Solution},
    utils::bit_set::BitSet,
};
use pathfinding::prelude::{astar, dijkstra_all};

use lazy_static::lazy_static;
use regex::Regex;

pub struct Day16;

impl Solution for Day16 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let (valves, start) = helper(input);

        let mut sorted_by_flow: Vec<(usize, i64)> =
            valves.values().map(|v| (v.name, v.flow)).collect();
        sorted_by_flow.sort_by_key(|(_, f)| *f);
        sorted_by_flow.reverse();

        Some((-one_actor_cost(&State::new(&valves, start), &valves, &sorted_by_flow)).into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let (valves, start) = helper(input);

        let mut sorted_by_flow: Vec<(usize, i64)> =
            valves.values().map(|v| (v.name, v.flow)).collect();
        sorted_by_flow.sort_by_key(|(_, f)| *f);
        sorted_by_flow.reverse();

        let (_, cost) = astar(
            &StateWithElephant::new(&valves, start),
            |state| state.moves(&valves),
            |state| state.heuristic(&sorted_by_flow, &valves),
            |state| state.finished(&valves),
        )
        .unwrap();

        Some((-cost).into())
    }
}

const START: &str = "AA";

lazy_static! {
    static ref PATTERN: Regex = Regex::new(
        r"Valve (?P<name>[A-Z]{2}).*=(?P<rate>\d+); tunnels? leads? to valves? (?P<exit>.*)$"
    )
    .unwrap();
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow: i64,
    exit: Vec<String>,
}

impl From<&str> for Valve {
    fn from(value: &str) -> Self {
        let caps = PATTERN.captures(value).unwrap();

        Self {
            name: caps["name"].to_string(),
            flow: caps["rate"].parse().unwrap(),
            exit: caps["exit"]
                .split(", ")
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
        }
    }
}

fn parse(input: &str) -> Vec<Valve> {
    input.trim_end().lines().map(Valve::from).collect()
}

fn helper(input: &str) -> (HashMap<usize, IdValve>, usize) {
    identifiers(&compress(&parse(input)))
}

#[derive(Debug)]
struct CompressedValve<'a> {
    name: &'a str,
    flow: i64,
    exit: Vec<(&'a str, i64)>,
}

struct IdCache<T> {
    cache: HashMap<T, usize>,
}

impl<T: PartialEq + Eq + Hash> IdCache<T> {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn id(&mut self, item: T) -> usize {
        let len = self.cache.len();
        match self.cache.entry(item) {
            Entry::Vacant(e) => {
                e.insert(len);
                len
            }
            Entry::Occupied(e) => *e.get(),
        }
    }
}

fn compress(valves: &[Valve]) -> HashMap<&str, CompressedValve> {
    let valves: HashMap<&str, &Valve> = valves.iter().map(|v| (v.name.as_ref(), v)).collect();
    let mut compressed = HashMap::new();

    for &k in valves.keys().filter(|&&k| k == START || valves[k].flow > 0) {
        let predecessors = dijkstra_all(&k, |n| {
            if *n != k && valves[n].flow > 0 {
                return vec![];
            }

            valves[n].exit.iter().map(|e| (e.as_str(), 1)).collect()
        });

        let exit = predecessors
            .iter()
            .filter(|(n, _)| valves[**n].flow > 0)
            .map(|(n, (_, c))| (*n, *c))
            .collect();

        compressed.insert(
            k,
            CompressedValve {
                name: k,
                flow: valves[k].flow,
                exit,
            },
        );
    }

    compressed
}

struct IdValve {
    name: usize,
    flow: i64,
    exit: Vec<(usize, i64)>,
}

fn identifiers(valves: &HashMap<&str, CompressedValve>) -> (HashMap<usize, IdValve>, usize) {
    let mut cache = IdCache::new();

    for &name in valves.keys() {
        cache.id(name);
    }

    let id_valves = valves
        .values()
        .map(|v| {
            (
                cache.id(v.name),
                IdValve {
                    name: cache.id(v.name),
                    flow: v.flow,
                    exit: v.exit.iter().map(|(n, c)| (cache.id(n), *c)).collect(),
                },
            )
        })
        .collect();

    let start = cache.id(START);

    (id_valves, start)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    remaining: i64,
    current: (usize, i64),
    opened: BitSet,
}

impl State {
    fn new(valves: &HashMap<usize, IdValve>, start: usize) -> Self {
        let mut opened = BitSet::default();
        if valves[&start].flow == 0 {
            opened.insert(start);
        }

        Self {
            remaining: 30,
            current: (start, 0),
            opened,
        }
    }

    fn opened(&self, valve: usize) -> bool {
        self.opened.contains(valve)
    }

    fn moves(&self, valves: &HashMap<usize, IdValve>) -> Vec<(Self, i64)> {
        let mut nexts = vec![];
        let (dest, distance) = self.current;

        if distance > 0 {
            let mut next = self.clone();
            next.remaining -= 1;
            next.current = (dest, distance - 1);
            nexts.push((next, 0));
            return nexts;
        }

        if !self.opened(dest) {
            let mut next = self.clone();
            next.remaining -= 1;
            next.opened.insert(dest);
            let cost = next.remaining * valves[&dest].flow;
            nexts.push((next, -cost));
        }

        for (next_dest, next_distance) in &valves[&dest].exit {
            let mut next = self.clone();
            next.remaining -= 1;
            next.current = (*next_dest, next_distance - 1);
            nexts.push((next, 0));
        }

        nexts
    }

    fn finished(&self, valves: &HashMap<usize, IdValve>) -> bool {
        self.remaining == 0 || self.opened.len() == valves.len()
    }

    fn heuristic(&self, sorted_by_flow: &[(usize, i64)]) -> i64 {
        let mut h = 0;
        let mut r = self.remaining;
        let mut i = 0;

        while r >= 1 && i < sorted_by_flow.len() {
            let inc = sorted_by_flow[i..]
                .iter()
                .position(|(name, _)| !self.opened(*name));

            if let Some(inc) = inc {
                i += inc;
            } else {
                break;
            }

            r -= 1;
            h += sorted_by_flow[i].1 * r;
            i += 1;
            r -= 1;
        }

        -h
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct StateWithElephant {
    remaining: i64,
    actors: [(usize, i64); 2],
    opened: BitSet,
}

impl StateWithElephant {
    fn new(valves: &HashMap<usize, IdValve>, start: usize) -> Self {
        let mut opened = BitSet::default();
        if valves[&start].flow == 0 {
            opened.insert(start);
        }

        Self {
            remaining: 26,
            actors: [(start, 0), (start, 0)],
            opened,
        }
    }

    fn opened(&self, valve: usize) -> bool {
        self.opened.contains(valve)
    }

    fn finished(&self, valves: &HashMap<usize, IdValve>) -> bool {
        self.remaining == 0 || self.opened.len() == valves.len()
    }

    fn actor_moves(&self, valves: &HashMap<usize, IdValve>, actor: usize) -> Vec<(Self, i64)> {
        let mut nexts = vec![];
        let (dest, distance) = self.actors[actor];

        if distance > 0 {
            let mut next = self.clone();
            next.actors[actor] = (dest, distance - 1);
            nexts.push((next, 0));
            return nexts;
        }

        if !self.opened(dest) {
            let mut next = self.clone();
            next.opened.insert(dest);
            let cost = next.remaining * valves[&dest].flow;
            nexts.push((next, -cost));
        }

        for (next_dest, next_distance) in &valves[&dest].exit {
            let mut next = self.clone();
            next.actors[actor] = (*next_dest, next_distance - 1);
            nexts.push((next, 0));
        }

        nexts
    }

    fn you_moves(&self, valves: &HashMap<usize, IdValve>) -> Vec<(Self, i64)> {
        self.actor_moves(valves, 0)
    }

    fn elephant_moves(&self, valves: &HashMap<usize, IdValve>) -> Vec<(Self, i64)> {
        self.actor_moves(valves, 1)
    }

    fn moves(&self, valves: &HashMap<usize, IdValve>) -> Vec<(Self, i64)> {
        let mut moved = self.clone();
        moved.remaining -= 1;

        moved
            .you_moves(valves)
            .into_iter()
            .flat_map(|(n, c0)| {
                n.elephant_moves(valves)
                    .into_iter()
                    .map(move |(n, c1)| (n, c0 + c1))
            })
            .collect()
    }

    fn heuristic(&self, sorted_by_flow: &[(usize, i64)], valves: &HashMap<usize, IdValve>) -> i64 {
        self.actors
            .into_iter()
            .map(|a| {
                one_actor_cost(
                    &(State {
                        remaining: self.remaining,
                        current: a,
                        opened: self.opened,
                    }),
                    valves,
                    sorted_by_flow,
                )
            })
            .sum()
    }
}

fn one_actor_cost(
    state: &State,
    valves: &HashMap<usize, IdValve>,
    sorted_by_flow: &[(usize, i64)],
) -> i64 {
    let (_, cost) = astar(
        state,
        |state| state.moves(valves),
        |state| state.heuristic(sorted_by_flow),
        |state| state.finished(valves),
    )
    .unwrap();
    cost
}

#[cfg(test)]
mod tests {
    use crate::{utils::load_sample, Year};

    use super::*;

    lazy_static! {
        static ref SAMPLE_INPUT: String = load_sample(Year::Y2022, "16.txt").unwrap();
    }

    #[test]
    fn test_a() {
        assert_eq!(
            Day16.solve_a(SAMPLE_INPUT.as_str()),
            Some(Answer::Int(1651))
        );
    }

    #[test]
    fn test_b() {
        assert_eq!(
            Day16.solve_b(SAMPLE_INPUT.as_str()),
            Some(Answer::Int(1707))
        );
    }
}
