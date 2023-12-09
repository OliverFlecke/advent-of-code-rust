//! Functions to cache input locally in files.
use std::{
    fs,
    path::{Path, PathBuf},
};

use super::Problem;
use crate::Year;

pub fn store_input_in_cache(problem: Problem, input: &String) -> std::io::Result<()> {
    fs::create_dir_all(get_input_cache_directory(problem.year()))?;
    fs::write(get_input_cache_full_filename(problem), input)
}

pub fn get_input_cache_full_filename(problem: Problem) -> PathBuf {
    Path::new(&get_input_cache_directory(problem.year()))
        .join(format!("{day}.txt", day = problem.day()))
}

/// Directory where input is cached at.
fn get_input_cache_directory(year: &Year) -> String {
    format!(".input/{year}/", year = year.as_int())
}
