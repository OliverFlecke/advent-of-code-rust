use crate::solutions::{answer::Answer, Solution};

// Cool website to explain calculations on hexagons https://www.redblobgames.com/grids/hexagons/

pub struct Day11 {}

enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

#[derive(Debug, Clone)]
struct CubeCoordinate {
    q: i32,
    r: i32,
    s: i32,
}

impl CubeCoordinate {
    fn zero() -> Self {
        CubeCoordinate { q: 0, r: 0, s: 0 }
    }

    fn distance_to_origin(self) -> u32 {
        self.q.abs().max(self.r.abs()).max(self.s.abs()) as u32
    }

    // pub fn distances(&self, other: &Self) -> u32 {
    //     self.q
    //         .abs_diff(other.q)
    //         .max(self.r.abs_diff(other.r))
    //         .max(self.s.abs_diff(other.s))
    // }

    // pub fn subtract(&self, other: &Self) -> Self {
    //     CubeCoordinate {
    //         q: self.q - other.q,
    //         r: self.r - other.r,
    //         s: self.s - other.s,
    //     }
    // }

    fn move_in_dir(self, direction: Direction) -> Self {
        match direction {
            Direction::North => CubeCoordinate {
                q: self.q,
                r: self.r - 1,
                s: self.s + 1,
            },
            Direction::South => CubeCoordinate {
                q: self.q,
                r: self.r + 1,
                s: self.s - 1,
            },
            Direction::NorthEast => CubeCoordinate {
                q: self.q + 1,
                r: self.r - 1,
                s: self.s,
            },
            Direction::SouthWest => CubeCoordinate {
                q: self.q - 1,
                r: self.r + 1,
                s: self.s,
            },
            Direction::SouthEast => CubeCoordinate {
                q: self.q + 1,
                r: self.r,
                s: self.s - 1,
            },
            Direction::NorthWest => CubeCoordinate {
                q: self.q - 1,
                r: self.r,
                s: self.s + 1,
            },
        }
    }
}

impl Solution for Day11 {
    fn solve_a(&self, input: &str) -> Answer {
        input
            .split(',')
            .map(|x| match x {
                "n" => Direction::North,
                "ne" => Direction::NorthEast,
                "se" => Direction::SouthEast,
                "s" => Direction::South,
                "sw" => Direction::SouthWest,
                "nw" => Direction::NorthWest,
                _ => panic!("Connect convert {x}"),
            })
            .fold(CubeCoordinate::zero(), CubeCoordinate::move_in_dir)
            .distance_to_origin()
            .into()
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
        assert_eq!(Day11 {}.solve_a("ne,ne,ne"), Answer::UInt(3));
        assert_eq!(Day11 {}.solve_a("ne,ne,sw,sw"), Answer::UInt(0));
        assert_eq!(Day11 {}.solve_a("ne,ne,s,s"), Answer::UInt(2));
        assert_eq!(Day11 {}.solve_a("se,sw,se,sw,sw"), Answer::UInt(3));
    }
}
