use std::fmt::Display;

use derive_getters::{Dissolve, Getters};

/// Represents a problem for Advent of Code.
///
/// There are a few different ways to create a `Problem`:
/// with `new`, a `(Year, Day)` tuple implements `From`, or
/// `(u16, u8)` tuple implements `TryFrom` by validating a
/// valid year and day is provided.
///
/// ```rust
/// # use advent_of_code_client::{Problem, Year};
/// let a = Problem::new(Year::Y2017, 4);
/// let b = (Year::Y2017, 4).into();
/// let c = (2017_u16, 4_u8).try_into().unwrap();
///
/// assert_eq!(a, b);
/// assert_eq!(b, c);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Getters, Dissolve)]
pub struct Problem {
    year: Year,
    day: Day,
}

impl Problem {
    /// Create a new problem for a given year and day.
    pub const fn new(year: Year, day: Day) -> Self {
        Self { year, day }
    }
}

impl From<(Year, Day)> for Problem {
    fn from((year, day): (Year, Day)) -> Self {
        Self { year, day }
    }
}

impl TryFrom<(u16, u8)> for Problem {
    type Error = String;

    fn try_from((year, day): (u16, u8)) -> Result<Self, Self::Error> {
        let Some(year) = Year::from_repr(year) else {
            return Err(format!(
                "Invalid year provided. Valid years are from 2015 to {}",
                Year::max()
            ));
        };
        if day == 0 || day > 25 {
            return Err("Invalid day provided. Valid days are from 1 to 25".to_string());
        }

        Ok(Self { year, day })
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}/{}", self.year, self.day)
    }
}

/// Represents a year of advent of code challenges.
#[derive(Copy, Clone, Debug, PartialEq, Eq, clap::ValueEnum, strum::FromRepr)]
#[cfg_attr(test, derive(fake::Dummy))]
#[repr(u16)]
pub enum Year {
    Y2016 = 2016,
    Y2017 = 2017,
    Y2018 = 2018,
    Y2019 = 2019,
    Y2020 = 2020,
    Y2021 = 2021,
    Y2022 = 2022,
    Y2023 = 2023,
}

impl Year {
    pub const fn as_int(self) -> u16 {
        self as u16
    }

    pub const fn max() -> Year {
        Self::Y2023
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Indicates the day for a given problem.
pub type Day = u8;

/// Indicates the level for a given daily problem. Also known as part 1 and 2.
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub enum Level {
    A = 1,
    B = 2,
}

impl Level {
    pub fn as_int(self) -> u8 {
        self as u8
    }
}

impl From<u8> for Level {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::A,
            2 => Self::B,
            _ => panic!("Cannot convert value {} to Level", value),
        }
    }
}

#[cfg(test)]
mod test {
    use std::convert::Into;

    use fake::{Fake, Faker};

    use super::*;

    #[test]
    fn from_year_and_day_tuple() {
        let year: Year = Faker.fake();
        let day: Day = (1..=25).fake();
        assert_eq!(Into::<Problem>::into((year, day)), Problem { year, day });
    }
}
