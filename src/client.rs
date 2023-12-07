use self::score::ScoreMap;
use super::{Day, Level, Year};
use colored::Colorize;
use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderValue, COOKIE},
    Url,
};
use std::{env, error::Error, fs, process::exit};

pub mod score;

const TOKEN_NAME: &str = "AOC_TOKEN";

#[derive(Debug)]
pub struct AocClient {
    base_url: Url,
    http_client: Client,
}

impl Default for AocClient {
    fn default() -> Self {
        let url = Url::parse("https://adventofcode.com/").expect("Failed to create URL for AoC");

        Self::new(url, get_token())
    }
}

impl AocClient {
    /// Create a new client to interact with Advent of Code.
    pub fn new(base_url: Url, aoc_token: String) -> Self {
        let http_client = build_client(&aoc_token);

        AocClient {
            base_url,
            http_client,
        }
    }

    /// Get the personal input for a user for a given problem.
    pub fn get_input(&self, year: Year, day: u8) -> Result<String, Box<dyn Error>> {
        match fs::read_to_string(cache::get_input_cache_full_filename(year, day)) {
            Ok(content) => Ok(content),
            Err(_) => {
                let input = self.download_input(year, day);
                cache::store_input_in_cache(year, day, &input)?;
                Ok(input)
            }
        }
    }

    /// Submit an answer for a problem on a given year, day, and level.
    pub fn submit(&self, year: Year, day: Day, level: Level, answer: &String) {
        let mut scores = ScoreMap::load(year);
        let value = scores.get_score_for_day(day);

        if value.map(|x| x >= level).unwrap_or_default() {
            println!(
                "{} {}",
                "Skipping submission - problem is already solved. Answer given:".green(),
                answer.bold().green()
            );
            return;
        }

        println!("Submitting answer for {year:?}/{day}/{level:?} is: {answer}",);

        match self.post_answer(year, day, level, answer) {
            Ok(res) => match parse_submission_response_text(res) {
                SubmissionResult::Correct => {
                    println!("{}", "Answer is correct".green());
                    scores.set_score_for_day(day, &level);
                }
                SubmissionResult::AlreadyCompleted => {
                    println!(
                        "{}",
                        "Problem already solved, but answer was correct".green()
                    );
                    scores.set_score_for_day(day, &level);
                }
                SubmissionResult::Incorrect => {
                    println!("{}", "You answered incorrectly!".red());
                }
                SubmissionResult::TooRecent(_) => {
                    println!("You have submitted an answer too recently. Wait a bit and try again")
                }
            },
            Err(err) => panic!("Error: {}", err),
        };
    }

    /// Send a HTTP POST request with the answer for the problem at a given year,
    /// day, and level. The answer must always be provided as a string.
    fn post_answer(
        &self,
        year: Year,
        day: Day,
        level: Level,
        answer: &String,
    ) -> Result<Response, reqwest::Error> {
        self.http_client
            .post(format!(
                "{base}/answer",
                base = self.get_base_url(year, day)
            ))
            .form(&[
                ("level", level.as_int().to_string()),
                ("answer", answer.to_string()),
            ])
            .send()
    }

    /// Download the input for a given problem.
    fn download_input(&self, year: Year, day: Day) -> String {
        let url = format!("{base}/input", base = self.get_base_url(year, day));

        match self.http_client.get(url).send() {
            Ok(response) if response.status().is_success() => {
                response.text().expect("input to be valid")
            }
            Ok(response) => {
                print!(
                    "Invalid status code: {}. Message from server:\n{}",
                    response.status(),
                    response.text().unwrap()
                );
                exit(1);
            }
            Err(e) => panic!("Failed to download input {e:?}"),
        }
    }

    /// Get the base url for a problem.
    fn get_base_url(&self, year: Year, day: u8) -> String {
        self.base_url
            .join(&format!("{year}/day/{day}", year = year.as_int()))
            .unwrap()
            .as_str()
            .to_string()
        // format!(
        //     "{base}/{year}/day/{day}",
        //     base = self.base_url,
        //     year = year.as_int(),
        // )
    }
}

/// Read the token required to authenticate against the Advent of Code server.
/// Panics if it cannot be found.
fn get_token() -> String {
    match env::var(TOKEN_NAME) {
        Ok(token) => token,
        Err(_) => panic!("Session token to authenticate against advent of code was not found. It should be an environment variable named 'AOC_TOKEN'"),
    }
}

/// Result of a submission of an answer to a problem.
pub enum SubmissionResult {
    Correct,
    AlreadyCompleted,
    Incorrect,
    TooRecent(u32),
}

fn parse_submission_response_text(response: Response) -> SubmissionResult {
    let body = get_main_part_from_html_response(response);

    if body.contains("That's the right answer") {
        SubmissionResult::Correct
    } else if body.contains("already complete it") {
        SubmissionResult::AlreadyCompleted
    } else if body.contains("answer too recently") {
        // TODO: Output how much time to wait for
        println!("Body: {}", body);
        SubmissionResult::TooRecent(0)
    } else if body.contains("not the right answer") {
        println!("Body: {}", body);
        SubmissionResult::Incorrect
    } else {
        panic!("Unknown response:\n\n{}", body);
    }
}

fn get_main_part_from_html_response(response: Response) -> String {
    let pattern = regex::RegexBuilder::new(r"<main>[\s\S]*</main>")
        .multi_line(true)
        .build()
        .unwrap();
    let body = response.text().unwrap();
    let m = pattern.find(body.as_str()).unwrap();
    m.as_str().to_string()
}

/// Build a HTTP client to send request to Advent of Code.
fn build_client(token: &str) -> Client {
    reqwest::blocking::Client::builder()
        .default_headers({
            let mut headers = HeaderMap::new();
            headers.insert(
                COOKIE,
                HeaderValue::from_str(&format!("session={token}"))
                    .expect("Failed to make header value with token"),
            );
            headers
        })
        .user_agent("github.com/OliverFlecke/advent-of-code-rust by oliverfl@live.dk")
        .build()
        .expect("Failed to create reqwest client")
}

/// Functions to cache input locally in files.
mod cache {
    use crate::{Day, Year};
    use std::{
        fs,
        path::{Path, PathBuf},
    };

    pub fn store_input_in_cache(year: Year, day: Day, input: &String) -> std::io::Result<()> {
        fs::create_dir_all(get_input_cache_directory(year))?;
        fs::write(get_input_cache_full_filename(year, day), input)
    }

    pub fn get_input_cache_full_filename(year: Year, day: Day) -> PathBuf {
        Path::new(&get_input_cache_directory(year)).join(format!("{day}.txt", day = day))
    }

    /// Directory where input is cached at.
    fn get_input_cache_directory(year: Year) -> String {
        format!(".input/{year}/", year = year.as_int())
    }
}

#[cfg(test)]
mod test {
    use fake::{Fake, Faker};
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

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
            AocClient::default().get_base_url(Year::Y2016, 17)
        );
    }

    #[test]
    fn get_input_test() {
        let input = AocClient::default().get_input(Year::Y2017, 1).unwrap();
        assert_ne!("", input);
    }

    #[async_std::test]
    async fn submit_answer() {
        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/2017/day/1/answer"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let answer: String = Faker.fake();
        let client = AocClient::new(Url::parse(&mock_server.uri()).unwrap(), Faker.fake());

        client.submit(Year::Y2017, 1, Level::A, &answer);
    }
}
