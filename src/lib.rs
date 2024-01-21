mod search;
pub mod config;
use std::error::Error;
use std::{fs};
pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search::insensitive(&config.query, &contents)
    } else {
        search::sensitive(&config.query, &contents)
    };

    for line in results {
        println!("{line}")
    }
    Ok(())
}

