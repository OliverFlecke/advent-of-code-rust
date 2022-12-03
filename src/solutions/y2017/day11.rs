use crate::solutions::{answer::Answer, Solution};

// Cool website to explain calculations on hexagons https://www.redblobgames.com/grids/hexagons/

pub struct Day11 {}

/// Representing the different directions one can move in a vertically
/// aligned hex grid.
enum HexDirection {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl From<&str> for HexDirection {
    fn from(value: &str) -> Self {
        match value {
            "n" => HexDirection::North,
            "ne" => HexDirection::NorthEast,
            "se" => HexDirection::SouthEast,
            "s" => HexDirection::South,
            "sw" => HexDirection::SouthWest,
            "nw" => HexDirection::NorthWest,
            _ => panic!("Connect convert {value}"),
        }
    }
}

/// Cube based coordinates for reaching any position in a hex based grid.
#[derive(Debug, Clone, Copy)]
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

    /// Calculates the coordinate in the hex grid by moving in the given
    /// direction.
    fn move_in_dir(self, direction: HexDirection) -> Self {
        match direction {
            HexDirection::North => CubeCoordinate {
                q: self.q,
                r: self.r - 1,
                s: self.s + 1,
            },
            HexDirection::South => CubeCoordinate {
                q: self.q,
                r: self.r + 1,
                s: self.s - 1,
            },
            HexDirection::NorthEast => CubeCoordinate {
                q: self.q + 1,
                r: self.r - 1,
                s: self.s,
            },
            HexDirection::SouthWest => CubeCoordinate {
                q: self.q - 1,
                r: self.r + 1,
                s: self.s,
            },
            HexDirection::SouthEast => CubeCoordinate {
                q: self.q + 1,
                r: self.r,
                s: self.s - 1,
            },
            HexDirection::NorthWest => CubeCoordinate {
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
            .trim_end()
            .split(',')
            .map(|dir_str| dir_str.into())
            .fold(CubeCoordinate::zero(), CubeCoordinate::move_in_dir)
            .distance_to_origin()
            .into()
    }

    fn solve_b(&self, input: &str) -> Answer {
        input
            .trim_end()
            .split(',')
            .map(|dir_str| dir_str.into())
            .fold((0, CubeCoordinate::zero()), |(max_dist, pos), dir| {
                let new_pos = CubeCoordinate::move_in_dir(pos, dir);
                (new_pos.distance_to_origin().max(max_dist), new_pos)
            })
            .0
            .into()
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
