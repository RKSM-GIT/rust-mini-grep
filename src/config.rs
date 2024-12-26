use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let ignore_case_var = env::var("IGNORE_CASE").unwrap_or("false".to_string());

        match ignore_case_var.parse::<bool>() {
            Ok(x) => Ok(Config {
                query: args[1].clone(),
                file_path: args[2].clone(),
                ignore_case: x,
            }),
            Err(_) => Err("Invalid IGNORE_CASE env variable."),
        }
    }
}
