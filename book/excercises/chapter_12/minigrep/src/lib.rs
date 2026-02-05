pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Iterating over the vector with a for
    // let mut result: Vec<&str> = vec![];
    //
    // for line in contents.lines() {
    // if line.contains(query) {
    // result.push(line);
    // }
    // }
    //
    // result

    // FP style filter on each line
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result: Vec<&str> = vec![];
    // let query = query.to_lowercase();
    //
    // for line in contents.lines() {
    // if line.to_lowercase().contains(&query) {
    // result.push(line);
    // }
    // }
    //
    // result
    //

    // to_lowercase returns a String, but contains ask for a pattern, which String doesnt
    // implement. Which is why I had to dereference.
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_more_results() {
        let query = "u";
        let contents = "\
Rust:
safe, fast, productive
bla bla.";
        assert_eq!(
            vec!["Rust:", "safe, fast, productive"],
            search(query, contents)
        )
    }

    #[test]
    fn search_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
bla bla.";
        assert_eq!(vec!["safe, fast, productive"], search(query, contents))
    }

    #[test]
    fn search_no_result() {
        let query = "mist";
        let contents = "\
Rust:
safe, fast, productiv
bla bla.";
        let expected: Vec<&str> = vec![];
        assert_eq!(expected, search(query, contents))
    }

    #[test]
    fn search_case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productiv
bla bla.
Trust me";
        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_insensitive(query, contents)
        );
    }
}
