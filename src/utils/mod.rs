use std::{env, fs, io::Error, string::String};

use advent_of_code_client::Year;

pub mod bit_set;
pub mod map2d;
pub mod math;
pub mod ocr;
/// Module for utility functions
pub mod take_until_inclusive;

pub fn load_sample(year: Year, name: &str) -> Result<String, Error> {
    let path =
        env::var("CARGO_MANIFEST_DIR").unwrap() + &format!("/samples/{}/{name}", year.as_int());
    fs::read_to_string(path)
}
