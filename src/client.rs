use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderValue, COOKIE},
};
use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};
pub mod score;

use self::score::ScoreMap;

use super::{Day, Level, Year};

const TOKEN_NAME: &str = "AOC_TOKEN";

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

pub enum SubmissionResult {
    Correct,
    AlreadyCompleted,
    Incorrect,
    TooRecent(u32),
}

fn parse_submission_response_text(response: Response) -> SubmissionResult {
    let body = response.text().unwrap();

    if body.contains("That's the right answer") {
        SubmissionResult::Correct
    } else if body.contains("already complete it") {
        SubmissionResult::AlreadyCompleted
    } else if body.contains("answer too recently") {
        // TODO: Output how much time to wait for
        SubmissionResult::TooRecent(0)
    } else if body.contains("not the right answer") {
        println!("Body: {}", body);
        SubmissionResult::Incorrect
    } else {
        panic!("Unknown response:\n\n{}", body);
    }
}

pub fn submit(year: Year, day: Day, level: Level, answer: &String) {
    let mut scores = ScoreMap::load(year);
    let value = scores.get_score_for_day(day);

    if value.map(|x| x >= level).unwrap_or_default() {
        println!("Skipping submission - problem is already solved");
        return;
    }

    println!(
        "Submitting answer for {year:?}/{day}/{level:?} is: {answer}",
        year = year,
        day = day,
        level = level,
        answer = answer
    );

    // panic!("Don't submit");

    match post_answer(year, day, level, answer) {
        Ok(res) => match parse_submission_response_text(res) {
            SubmissionResult::Correct => {
                println!("Answer is correct");
                scores.set_score_for_day(day, &level);
            }
            SubmissionResult::AlreadyCompleted => {
                println!("Problem already solved, but answer was correct");
                scores.set_score_for_day(day, &level);
            }
            SubmissionResult::Incorrect => {
                println!("You answered incorrectly!");
            }
            SubmissionResult::TooRecent(_) => {
                println!("You have submitted an answer too recently. Wait a bit and try again")
            }
        },
        Err(err) => panic!("Error: {}", err),
    };
}

fn post_answer(
    year: Year,
    day: Day,
    level: Level,
    answer: &String,
) -> Result<Response, reqwest::Error> {
    build_client()
        .post(format!("{base}/answer", base = get_base_url(year, day)))
        .form(&[
            ("level", level.as_int().to_string()),
            ("answer", answer.to_string()),
        ])
        .send()
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

fn build_client() -> Client {
    reqwest::blocking::Client::builder()
        .default_headers(get_headers())
        .build()
        .unwrap()
}

fn download_input(year: Year, day: Day) -> String {
    let url = format!("{base}/input", base = get_base_url(year, day));
    let client = build_client();

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
mod test {
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
