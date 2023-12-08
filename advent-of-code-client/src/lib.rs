use self::score::ScoreMap;
use colored::Colorize;
use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderValue, COOKIE},
    Url,
};
use std::{env, fs};

mod cache;
mod problem;
mod score;

pub use problem::{Day, Level, Problem, Year};

const TOKEN_NAME: &str = "AOC_TOKEN";

/// Client for interacting with `https://adventofcode.com`.
#[derive(Debug)]
pub struct AocClient {
    base_url: Url,
    http_client: Client,
}

impl Default for AocClient {
    fn default() -> Self {
        Self::new(default_url_for_advent_of_code(), get_token())
    }
}

impl AocClient {
    /// Create a new client to interact with Advent of Code.
    fn new(base_url: Url, aoc_token: String) -> Self {
        let http_client = Self::build_client(&aoc_token);

        AocClient {
            base_url,
            http_client,
        }
    }

    /// Create a new client from a AoC session token.
    pub fn from_token(aoc_token: String) -> Self {
        Self {
            base_url: default_url_for_advent_of_code(),
            http_client: Self::build_client(&aoc_token),
        }
    }

    /// Get the personal input for a user for a given problem.
    pub fn get_input(&self, problem: Problem) -> anyhow::Result<String> {
        match fs::read_to_string(cache::get_input_cache_full_filename(problem)) {
            Ok(content) => Ok(content),
            Err(_) => {
                let input = self.download_input(problem);
                cache::store_input_in_cache(problem, &input)?;
                Ok(input)
            }
        }
    }

    /// Submit an answer for a problem on a given year, day, and level.
    pub fn submit(&self, problem: Problem, level: Level, answer: &String) {
        let mut scores = ScoreMap::load(*problem.year());
        let value = scores.get_score_for_day(*problem.day());

        if value.map(|x| x >= level).unwrap_or_default() {
            println!(
                "{} {}",
                "Skipping submission - problem is already solved. Answer given:".green(),
                answer.bold().green()
            );
            return;
        }

        println!("Submitting answer for {problem}/{level:?} is: {answer}",);

        match self.post_answer(problem, level, answer) {
            Ok(res) => match res.try_into().unwrap() {
                SubmissionResult::Correct => {
                    println!("{}", "Answer is correct".green());
                    scores.set_score_for_day(*problem.day(), &level);
                }
                SubmissionResult::AlreadyCompleted => {
                    println!(
                        "{}",
                        "Problem already solved, but answer was correct".green()
                    );
                    scores.set_score_for_day(*problem.day(), &level);
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
        problem: Problem,
        level: Level,
        answer: &String,
    ) -> Result<Response, reqwest::Error> {
        self.http_client
            .post(
                self.get_base_url_for_problem(problem)
                    .join("answer")
                    .expect("Failed to create `answer` URL"),
            )
            .form(&[
                ("level", level.as_int().to_string()),
                ("answer", answer.to_string()),
            ])
            .send()
    }

    /// Download the input for a given problem.
    fn download_input(&self, problem: Problem) -> String {
        let url = self
            .get_base_url_for_problem(problem)
            .join("input")
            .expect("Failed to create download URL for `input`");

        match self.http_client.get(url).send() {
            Ok(response) if response.status().is_success() => {
                response.text().expect("input to be valid")
            }
            Ok(response) => {
                panic!(
                    "Invalid status code: {}. Message from server:\n{}",
                    response.status(),
                    response.text().unwrap()
                );
            }
            Err(e) => panic!("Failed to download input {e:?}"),
        }
    }

    /// Get the base url for a problem.
    fn get_base_url_for_problem(&self, problem: Problem) -> Url {
        self.base_url
            .join(&format!(
                "{year}/day/{day}/",
                year = problem.year().as_int(),
                day = problem.day()
            ))
            .expect("Failed to create URL for problem")
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
}

fn default_url_for_advent_of_code() -> Url {
    Url::parse("https://adventofcode.com/").expect("Failed to create URL for AoC")
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubmissionResult {
    Correct,
    AlreadyCompleted,
    Incorrect,
    TooRecent(u32),
}

impl TryFrom<Response> for SubmissionResult {
    type Error = anyhow::Error;

    fn try_from(response: Response) -> Result<Self, Self::Error> {
        let body = get_main_part_from_html_response(response);

        if body.contains("That's the right answer") {
            Ok(SubmissionResult::Correct)
        } else if body.contains("already complete it") {
            Ok(SubmissionResult::AlreadyCompleted)
        } else if body.contains("answer too recently") {
            // TODO: Output how much time to wait for
            println!("Body: {}", body);
            Ok(SubmissionResult::TooRecent(0))
        } else if body.contains("not the right answer") {
            println!("Body: {}", body);
            Ok(SubmissionResult::Incorrect)
        } else {
            Err(anyhow::anyhow!("Unknown response:\n\n{}", body))
        }
    }
}

/// This extracts the part of the submission response within the `<main>` tags.
/// As this contains the primary message from AoC, the rest can be thrown away
/// when you just want to know whether your answer was right or not.
fn get_main_part_from_html_response(response: Response) -> String {
    let pattern = regex::RegexBuilder::new(r"<main>[\s\S]*</main>")
        .multi_line(true)
        .build()
        .unwrap();
    let body = response.text().unwrap();
    let m = pattern.find(body.as_str()).unwrap();
    m.as_str().to_string()
}

#[cfg(test)]
mod test {
    use fake::{Fake, Faker};
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    use crate::Year;

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
            "https://adventofcode.com/2016/day/17/"
                .parse::<Url>()
                .unwrap(),
            AocClient::default().get_base_url_for_problem((Year::Y2016, 17).into())
        );
    }

    #[test]
    fn get_input_test() {
        let input = AocClient::default()
            .get_input((Year::Y2017, 1).into())
            .unwrap();
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

        assert!(client
            .post_answer((Year::Y2017, 1).into(), Level::A, &answer)
            .is_ok());
    }
}
