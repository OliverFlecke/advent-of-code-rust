use std::{collections::HashMap, str::FromStr};

use crate::solutions::{answer::Answer, Solution};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    cells: Vec<Vec<bool>>,
}

impl Grid {
    fn new(size: usize) -> Self {
        let mut row = Vec::with_capacity(size);
        let mut cells = Vec::with_capacity(size);
        row.resize(size, false);
        cells.resize(size, row);

        Self { cells }
    }

    fn size(&self) -> usize {
        self.cells.len()
    }

    fn get(&self, (x, y): (usize, usize)) -> bool {
        self.cells[y][x]
    }

    fn set(&mut self, (x, y): (usize, usize), val: bool) {
        self.cells[y][x] = val;
    }

    fn flip(&self) -> Self {
        Self {
            cells: self
                .cells
                .iter()
                .map(|row| row.iter().rev().copied().collect())
                .collect(),
        }
    }

    fn rotate(&self) -> Self {
        let l = self.size();
        let mut rv = Self::new(l);
        for x in 0..l {
            for y in 0..l {
                rv.set((y, l - x - 1), self.get((x, y)));
            }
        }

        rv
    }

    fn variants(&self) -> Vec<Grid> {
        let mut rv = Vec::with_capacity(8);
        rv.push(self.clone());
        rv.push(self.flip());
        for _ in 0..6 {
            let g = rv[rv.len() - 2].rotate();
            rv.push(g);
        }
        rv
    }

    fn count_lights(&self) -> usize {
        self.cells.iter().flatten().filter(|c| **c).count()
    }

    fn split(&self) -> Vec<Grid> {
        let l = self.size();
        let m = match l {
            _ if l % 2 == 0 => 2,
            _ if l % 3 == 0 => 3,
            _ => panic!("Unsupported size"),
        };

        let s = l / m;

        (0..(s * s))
            .map(|i| {
                let mut part = Grid::new(m);
                let gx = i % s;
                let gy = i / s;
                for x in 0..m {
                    for y in 0..m {
                        part.set((x, y), self.get((x + gx * m, y + gy * m)));
                    }
                }
                part
            })
            .collect()
    }

    fn assemble_from(parts: Vec<Grid>) -> Grid {
        let size = (parts.len() as f64).sqrt() as usize;
        let subgrid_size = parts[0].size();

        let mut rv = Grid::new(subgrid_size * size);

        for (idx, subgrid) in parts.iter().enumerate() {
            let gx = idx % size;
            let gy = idx / size;
            for x in 0..subgrid_size {
                for y in 0..subgrid_size {
                    rv.set(
                        (x + gx * subgrid_size, y + gy * subgrid_size),
                        subgrid.get((x, y)),
                    );
                }
            }
        }

        rv
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            cells: vec![
                vec![false, true, false],
                vec![false, false, true],
                vec![true, true, true],
            ],
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            cells: input
                .split('/')
                .map(|row| {
                    row.chars()
                        .map(|c| match c {
                            '#' => true,
                            '.' => false,
                            _ => panic!("unexpected char in input: '{}'", c),
                        })
                        .collect()
                })
                .collect(),
        })
    }
}

#[derive(Debug)]
struct Rules {
    patterns: HashMap<Grid, Grid>,
}

impl Rules {
    fn new() -> Self {
        Self {
            patterns: HashMap::new(),
        }
    }

    fn add_rule(&mut self, from: Grid, to: Grid) {
        for variant in from.variants() {
            self.patterns.insert(variant, to.clone());
        }
    }

    fn apply_to(&self, from: &Grid) -> Grid {
        self.patterns.get(from).unwrap_or(from).clone()
    }
}

impl FromStr for Rules {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(input
            .lines()
            .map(|l| {
                let mut splits = l.split(" => ").map(|p| p.parse::<Grid>().unwrap());
                (splits.next().unwrap(), splits.next().unwrap())
            })
            .fold(Self::new(), |mut rules, (from, to)| {
                rules.add_rule(from, to);
                rules
            }))
    }
}

pub struct Day21;

impl Solution for Day21 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let rules: &Rules = &input.parse().unwrap();

        Some(run_iterations(rules, 5).count_lights().into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        let rules: &Rules = &input.parse().unwrap();

        Some(run_iterations(rules, 18).count_lights().into())
    }
}

fn run_iterations(rules: &Rules, iterations: usize) -> Grid {
    let mut image = Grid::default();

    for _ in 0..iterations {
        let parts: Vec<Grid> = image.split().iter().map(|g| rules.apply_to(g)).collect();
        image = Grid::assemble_from(parts);
    }

    image
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";

    const FINAL: &str = "##.##./#..#../....../##.##./#..#../......";

    #[test]
    fn parse() {
        let rules: Rules = INPUT.parse().unwrap();

        assert_eq!(
            rules.apply_to(&"../.#".parse::<Grid>().unwrap()),
            "##./#../...".parse().unwrap()
        );
        assert_eq!(
            rules.apply_to(&".#./..#/###".parse::<Grid>().unwrap()),
            "#..#/..../..../#..#".parse().unwrap()
        );
    }

    #[test]
    fn splits() {
        let image: Grid = "#..#/..../..../#..#".parse().unwrap();

        let divided = image.split();
        let mut it = divided.iter();
        assert_eq!(it.next(), Some(&"#./..".parse::<Grid>().unwrap()));
        assert_eq!(it.next(), Some(&".#/..".parse::<Grid>().unwrap()));
        assert_eq!(it.next(), Some(&"../#.".parse::<Grid>().unwrap()));
        assert_eq!(it.next(), Some(&"../.#".parse::<Grid>().unwrap()));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn count_lights() {
        assert_eq!(FINAL.parse::<Grid>().unwrap().count_lights(), 12);
    }

    #[test]
    fn test_a() {
        let rules: &Rules = &INPUT.parse().unwrap();

        assert_eq!(run_iterations(rules, 2).count_lights(), 12);
    }
}
