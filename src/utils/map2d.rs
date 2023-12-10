use std::ops::Add;

/// Coordinates coordinates to navigate a 2D world.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const fn inverse(&self) -> Self {
        use Direction::*;
        match *self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }

    pub const fn to_vector(&self) -> (isize, isize) {
        use Direction::*;
        match self {
            North => (-1, 0),
            South => (1, 0),
            East => (0, 1),
            West => (0, -1),
        }
    }
}

/// Represents a position in 2D. It is designed to work closely with [array2d::Array2d].
/// Note that in is generally oriented with the `y`/`row` first, which might
/// be the inverse to what you might think. This is true for all tuple addtions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn move_direction(self, direction: Direction) -> Self {
        self + direction.to_vector()
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Self {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Add<(usize, usize)> for Position {
    type Output = Position;

    fn add(self, (row, col): (usize, usize)) -> Self::Output {
        Self {
            row: self.row + row,
            col: self.col + col,
        }
    }
}
impl Add<(isize, isize)> for Position {
    type Output = Position;

    fn add(self, (row, col): (isize, isize)) -> Self::Output {
        Self {
            row: (self.row as isize).wrapping_add(row) as usize,
            col: (self.col as isize).wrapping_add(col) as usize,
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from((row, col): (usize, usize)) -> Self {
        Self { row, col }
    }
}

// impl AsRef<(usize, usize)> for Position {
//     fn as_ref(&self) -> &(usize, usize) {
//         (self.row, self.col)
//     }
// }

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case((1,0), (-1,0), (0,0))]
    fn add_tuple(
        #[case] a: (usize, usize),
        #[case] b: (isize, isize),
        #[case] expected: (usize, usize),
    ) {
        let a = Position { row: a.0, col: a.1 };

        assert_eq!(
            a + b,
            Position {
                row: expected.1,
                col: expected.0
            }
        );
    }
}
