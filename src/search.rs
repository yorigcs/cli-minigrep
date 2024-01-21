pub fn sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

pub fn insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }
    results
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
            Rust:\n\
            safe, fast, productive.\n\
            Duct tape.\n\
            Pick three.";

        assert_eq!(vec!["safe, fast, productive."], sensitive(query,contents))
    }
    #[test]

    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
            Rust:\n\
            safe, fast, productive.\n\
            safe, fast, productive.\n\
            Trust me.\n\
            Pick three.";

        assert_eq!(vec!["Rust:", "Trust me."], insensitive(query,contents))
    }
}