use crate::{Day, Year};
use derive_getters::{Dissolve, Getters};
use std::fmt::Display;

/// Represents a problem for Advent of Code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Getters, Dissolve)]
pub struct Problem {
    year: Year,
    day: Day,
}

impl Problem {
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
