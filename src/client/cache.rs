//! Functions to cache input locally in files.
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
