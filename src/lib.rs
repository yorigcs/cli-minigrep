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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn should_panic() {
        let args = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        let cfg = config::Config::build(&args).unwrap();
        run(cfg).unwrap()
    }
    #[test]
    fn should_read_file() {
        let args = vec!["1".to_string(), "2".to_string(), "./tests/fixtures/poem.txt".to_string()];
        let cfg = config::Config::build(&args).unwrap();
        run(cfg).unwrap()
    }
}
