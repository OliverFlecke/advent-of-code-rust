pub mod client;
pub mod solutions;
pub mod utils;

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
