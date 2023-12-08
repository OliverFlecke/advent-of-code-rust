use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::{Day, Level, Year};

const STARS_DIRECTORY: &str = ".stars";

pub(crate) struct ScoreMap {
    year: Year,
    map: HashMap<String, u8>,
}

impl ScoreMap {
    pub fn load(year: Year) -> ScoreMap {
        let filename = Self::get_filename(year);
        let content = match fs::read_to_string(filename.as_path()) {
            Ok(value) => value,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    return ScoreMap {
                        year,
                        map: HashMap::new(),
                    }
                }
                _ => panic!("{}", err),
            },
        };
        match serde_json::from_str(&content) {
            Ok(map) => ScoreMap { year, map },
            Err(_) => panic!("Unable to parse stars"),
        }
    }

    pub fn get_score_for_day(&self, day: Day) -> Option<Level> {
        self.map
            .get(&Self::day_to_key(day))
            .map(|x| Level::from(*x))
    }

    pub fn set_score_for_day(&mut self, day: Day, score: &Level) {
        self.map.insert(Self::day_to_key(day), score.as_int());
        self.store();
    }

    fn day_to_key(day: Day) -> String {
        format!("day{}", day)
    }

    fn get_filename(year: Year) -> PathBuf {
        Path::new(STARS_DIRECTORY).join(format!("{}.json", year.as_int()))
    }

    fn store(&self) {
        fs::create_dir_all(STARS_DIRECTORY).unwrap();
        let filename = Self::get_filename(self.year);
        match serde_json::to_string(&self.map) {
            Ok(content) => fs::write(filename, content).unwrap(),
            Err(err) => panic!("Unable to serialize scores: {}", err),
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_empty_test() {
        let data: HashMap<&str, u8> = HashMap::new();
        assert_eq!(serde_json::to_string(&data).unwrap(), "{}");
    }
}
