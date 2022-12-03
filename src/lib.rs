pub mod client;

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum Year {
    Y2016 = 2016,
    Y2017 = 2017,
    Y2018 = 2018,
    Y2019 = 2019,
    Y2020 = 2020,
    Y2021 = 2021,
    Y2022 = 2022,
}

impl From<u32> for Year {
    fn from(value: u32) -> Self {
        match value {
            2016 => Year::Y2016,
            2017 => Year::Y2017,
            2018 => Year::Y2018,
            2019 => Year::Y2019,
            2020 => Year::Y2020,
            2021 => Year::Y2021,
            2022 => Year::Y2022,
            _ => panic!("Cannot convert {} to a valid year", value),
        }
    }
}

impl Year {
    pub fn as_int(self) -> u16 {
        self as u16
    }
}

pub type Day = u8;

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
