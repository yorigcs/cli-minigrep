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