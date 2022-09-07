use std::env;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE};

const TOKEN_NAME: &str = "AOC_TOKEN";

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

pub fn get_token() -> String {
    env::var(TOKEN_NAME).unwrap()
}

pub fn get_base_url(year: Year, day: u8) -> String {
    format!("https://adventofcode.com/{year}/day/{day}", year = year.as_int(), day = day)
}

fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let value = format!("session={}", get_token());
    headers.insert(COOKIE, HeaderValue::from_str(value.as_str()).unwrap());
    headers
}

pub fn get_input(year: Year, day: u8) -> String {
    let url = format!("{base}/input", base = get_base_url(year, day));
    let client = reqwest::blocking::Client::builder()
        .default_headers(get_headers()) 
        .build().unwrap();

    client.get(url)
        .send().unwrap()
        .text().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_token_test() {
        let value = "abc";
        env::set_var(TOKEN_NAME, value);

        assert_eq!(value, get_token());
    }

    #[test]
    fn get_base_url_test() {
        assert_eq!("https://adventofcode.com/2016/day/17", 
                   get_base_url(Year::Y2016, 17));
    }

    #[test]
    fn get_input_test() {
        let input = get_input(Year::Y2017, 1);
        assert_ne!("", input);
    }
}
