use crate::config::Config;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let filtered_lines = match config.ignore_case {
        true => search_insensitive(&config.query, &contents),
        false => search(&config.query, &contents),
    };

    filtered_lines.iter().for_each(|line| println!("{line}"));

    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|x| x.contains(query)).collect()
}

pub fn search_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    content
        .lines()
        .filter(|x| x.to_lowercase().contains(&query))
        .collect()
}
