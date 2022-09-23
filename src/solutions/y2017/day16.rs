use std::iter::Map;

use crate::solutions::{answer::Answer, Solution};

pub struct Day16;

impl Solution for Day16 {
    fn solve_a(&self, input: &str) -> Answer {
        Self::helper_a(input, 16).into()
    }

    fn solve_b(&self, input: &str) -> Answer {
        let times = 1_000_000_000;
        Self::helper_b(input, 16, times).into()
    }
}

impl Day16 {
    fn parse(input: &str) -> Map<std::str::Split<'_, char>, fn(&str) -> DanceMove> {
        input.split(',').map(|s| match s.as_bytes()[0] as char {
            's' => DanceMove::Spin(s.get(1..).unwrap().parse().unwrap()),
            'x' => {
                let mut split = s.get(1..).unwrap().split('/');
                DanceMove::Exchange(
                    split.next().unwrap().parse().unwrap(),
                    split.next().unwrap().parse().unwrap(),
                )
            }
            'p' => {
                let mut split = s.get(1..).unwrap().split('/');
                DanceMove::Partner(
                    split.next().unwrap().as_bytes()[0] as char,
                    split.next().unwrap().as_bytes()[0] as char,
                )
            }
            x => panic!("unable to understand input: `{}`", x),
        })
    }

    fn dance(moves: impl Iterator<Item = DanceMove>, size: usize) -> String {
        let mut dancers = Self::create_dancers(size);

        moves.for_each(|m| m.dance(&mut dancers));

        dancers.iter().collect()
    }

    fn create_dancers(size: usize) -> Vec<char> {
        (0..size)
            .map(|i| (('a' as u8) + (i as u8)) as char)
            .collect()
    }

    fn helper_a(input: &str, size: usize) -> String {
        Self::dance(Self::parse(input), size)
    }

    fn helper_b(input: &str, size: usize, times: usize) -> String {
        let moves: Vec<DanceMove> = Self::parse(input).collect();
        let mut dancers = Self::create_dancers(size);
        let mut seen = vec![dancers.clone()];

        for _ in 0..times {
            moves.iter().for_each(|m| m.dance(&mut dancers));

            if seen.contains(&dancers) {
                if let Some((rep, _)) = seen.iter().enumerate().find(|&(_, d)| d == &dancers) {
                    let cycle_length = seen.len() - rep;
                    let index = rep + (times % cycle_length);
                    return seen[index].iter().collect();
                }
            } else {
                seen.push(dancers.clone());
            }
        }

        dancers.iter().collect()
    }
}

#[derive(PartialEq, Debug)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl DanceMove {
    fn dance(&self, dancers: &mut Vec<char>) {
        match self {
            DanceMove::Spin(x) => dancers.rotate_right(*x),
            DanceMove::Exchange(i, j) => dancers.swap(*i, *j),
            DanceMove::Partner(a, b) => {
                let i = dancers.iter().position(|x| *x == *a).unwrap();
                let j = dancers.iter().position(|x| *x == *b).unwrap();
                dancers.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "s1,x3/4,pe/b";

    #[test]
    fn parse() {
        let mut it = Day16::parse(INPUT);

        assert_eq!(it.next(), Some(DanceMove::Spin(1)));
        assert_eq!(it.next(), Some(DanceMove::Exchange(3, 4)));
        assert_eq!(it.next(), Some(DanceMove::Partner('e', 'b')));
    }

    #[test]
    fn create_dancers() {
        assert_eq!(Day16::create_dancers(5).iter().collect::<String>(), "abcde");
    }

    #[test]
    fn dance() {
        let mut dancers = Day16::create_dancers(5);
        let mut it = Day16::parse(INPUT);

        it.next().unwrap().dance(&mut dancers);
        assert_eq!(dancers.iter().collect::<String>(), "eabcd");
        it.next().unwrap().dance(&mut dancers);
        assert_eq!(dancers.iter().collect::<String>(), "eabdc");
        it.next().unwrap().dance(&mut dancers);
        assert_eq!(dancers.iter().collect::<String>(), "baedc");
    }

    #[test]
    fn helper_a() {
        assert_eq!(Day16::helper_a(INPUT, 5), "baedc");
    }

    #[test]
    fn helper_b() {
        assert_eq!(Day16::helper_b(INPUT, 5, 2), "ceadb");
    }
}
