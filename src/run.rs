use crate::config::Config;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("The contents of file is:\n {}", contents);

    Ok(())
}
