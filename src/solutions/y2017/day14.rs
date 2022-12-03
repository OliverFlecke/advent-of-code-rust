use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Result},
};

use crate::solutions::{answer::Answer, Solution};

use super::day10::KnotHash;

pub struct Day14 {}

impl Solution for Day14 {
    fn solve_a(&self, input: &str) -> Option<Answer> {
        let mut count: u32 = 0;
        for row in 0..128 {
            let row_input = format!("{}-{}", input, row);
            KnotHash::compute_hash(row_input)
                .into_iter()
                .for_each(|x| count += x.count_ones());
        }

        Some(count.into())
    }

    fn solve_b(&self, input: &str) -> Option<Answer> {
        Some(Grid::new(input).count_groups().into())
    }
}

#[derive(PartialEq)]
enum CellType {
    Free,
    Used,
}

struct Grid {
    map: HashMap<(usize, usize), CellType>,
}

impl Grid {
    const SIZE: usize = 128;

    pub fn new(input: impl AsRef<str>) -> Self {
        let mut map = HashMap::new();
        for row in 0..Self::SIZE {
            let row_input = format!("{}-{}", input.as_ref(), row);
            let hash = KnotHash::compute_hash(row_input);

            for (i, byte) in hash.into_iter().enumerate() {
                let rev = byte.reverse_bits();
                for x in 0..8 {
                    let col = i * 8 + x;
                    let mask = 1 << x;
                    let is_set = rev & mask;
                    map.insert(
                        (col, row),
                        match is_set != 0 {
                            true => CellType::Used,
                            false => CellType::Free,
                        },
                    );
                }
            }
        }

        Grid { map }
    }

    pub fn count_groups(&mut self) -> usize {
        let neighbors: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut count = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        while let Some(current) = self
            .map
            .iter()
            .find(|x| !visited.contains(x.0) && *x.1 == CellType::Used)
        {
            let mut queue = vec![*current.0];
            visited.insert(*current.0);

            while let Some(p) = queue.pop() {
                let ns = neighbors
                    .iter()
                    .map(|(dx, dy)| (*dx + p.0 as i32, *dy + p.1 as i32))
                    .filter(|(x, y)| {
                        0 <= *x && *x <= Self::SIZE as i32 && 0 <= *y && *y <= Self::SIZE as i32
                    })
                    .map(|(x, y)| (x as usize, y as usize));

                for n in ns {
                    if !visited.contains(&n) {
                        visited.insert(n);

                        if self
                            .map
                            .get(&n)
                            .map(|x| *x == CellType::Used)
                            .unwrap_or(false)
                        {
                            queue.push(n);
                        }
                    }
                }
            }

            count += 1;
        }

        count
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for row in 0..Self::SIZE {
            for col in 0..Self::SIZE {
                write!(
                    f,
                    "{}",
                    self.map
                        .get(&(col, row))
                        .map(|v| match v {
                            CellType::Free => '.',
                            CellType::Used => '#',
                        })
                        .unwrap()
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "flqrgnkx";

    #[test]
    fn test_a() {
        assert_eq!(Day14 {}.solve_a(TEST_INPUT), Some(Answer::UInt(8108)))
    }

    #[test]
    fn test_b() {
        assert_eq!(Day14 {}.solve_b(TEST_INPUT), Some(Answer::UInt(1242)))
    }
}
