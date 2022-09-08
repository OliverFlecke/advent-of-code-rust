use reqwest::header::{HeaderMap, HeaderValue, COOKIE};
use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use super::Year;

const TOKEN_NAME: &str = "AOC_TOKEN";

type Day = u8;

pub fn get_token() -> String {
    match env::var(TOKEN_NAME) {
        Ok(token) => token,
        Err(_) => panic!("Session token to authenticate against advent of code was not found. It should be an environment variable named 'AOC_TOKEN'"),
    }
}

pub fn get_base_url(year: Year, day: u8) -> String {
    format!(
        "https://adventofcode.com/{year}/day/{day}",
        year = year.as_int(),
        day = day
    )
}

fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    let value = format!("session={token}", token = get_token());
    headers.insert(COOKIE, HeaderValue::from_str(value.as_str()).unwrap());
    headers
}

pub fn get_input(year: Year, day: u8) -> Result<String, Box<dyn Error>> {
    match fs::read_to_string(get_input_cache_full_filename(year, day)) {
        Ok(content) => Ok(content),
        Err(_) => {
            let input = download_input(year, day);
            store_input_in_cache(year, day, &input)?;
            Ok(input)
        }
    }
}

fn download_input(year: Year, day: Day) -> String {
    let url = format!("{base}/input", base = get_base_url(year, day));
    let client = reqwest::blocking::Client::builder()
        .default_headers(get_headers())
        .build()
        .unwrap();

    client.get(url).send().unwrap().text().unwrap()
}

fn store_input_in_cache(year: Year, day: Day, input: &String) -> std::io::Result<()> {
    fs::create_dir_all(get_input_cache_directory(year))?;
    fs::write(get_input_cache_full_filename(year, day), &input)
}

fn get_input_cache_directory(year: Year) -> String {
    format!(".input/{year}/", year = year.as_int())
}

fn get_input_cache_full_filename(year: Year, day: Day) -> PathBuf {
    Path::new(&get_input_cache_directory(year)).join(format!("{day}.txt", day = day))
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
        assert_eq!(
            "https://adventofcode.com/2016/day/17",
            get_base_url(Year::Y2016, 17)
        );
    }

    #[test]
    fn get_input_test() {
        let input = get_input(Year::Y2017, 1).unwrap();
        assert_ne!("", input);
    }
}
