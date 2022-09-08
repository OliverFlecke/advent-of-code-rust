pub mod client;

#[derive(Copy, Clone, Debug)]
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
