use std::env;
pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool
}
impl <'a> Config<'a> {
    pub fn build(args: &'a [String]) -> Result<Self, &'static str > {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = &args[1];
        let file_path = &args[2];
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_build_correctly() {
        env::set_var("IGNORE_CASE", "1");
        let args = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        let config = Config::build(&args);
        let result = config.unwrap();
        assert_eq!(result.query, "2");
        assert_eq!(result.file_path, "3");
        assert!(result.ignore_case)
    }

    #[test]
    #[should_panic(expected = "not enough arguments")]
    fn should_panic() {
        let args = vec!["1".to_string(), "2".to_string()];
        let config = Config::build(&args);
        config.unwrap();
    }

}