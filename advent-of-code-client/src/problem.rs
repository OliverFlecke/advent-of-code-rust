use derive_getters::{Dissolve, Getters};
use std::fmt::Display;

/// Represents a problem for Advent of Code.
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

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}/{}", self.year, self.day)
    }
}

/// Represents a year of advent of code challenges.
#[derive(Copy, Clone, Debug, PartialEq, Eq, clap::ValueEnum)]
#[cfg_attr(test, derive(fake::Dummy))]
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
    pub fn as_int(self) -> u16 {
        self as u16
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
    use super::*;
    use fake::{Fake, Faker};
    use std::convert::Into;

    #[test]
    fn from_year_and_day_tuple() {
        let year: Year = Faker.fake();
        let day: Day = (1..=25).fake();
        assert_eq!(Into::<Problem>::into((year, day)), Problem { year, day });
    }
}
