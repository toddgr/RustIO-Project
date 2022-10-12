use std::error::Error;
use std::fs;
use std::env;

// Holds input arguments, parsed with parse_config
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}


// Configures how the program will work --> parses args
// Instead of using 'panic!', we will use Result<T, E> to get 
// a friendlier output for users.
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();    // Oftentimes people will avoid clone because
        let file_path = args[2].clone();// of its runtime cost. Only using for this tutorial
                                        // because the tradeoff here is relatively small
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path, 
            ignore_case,
        })
    }
}


// Reads the contents of the file
// Here we implement Box<dyn Error>, which lets us tell the function to return
// an error without specifying its type
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}


// Find the line in the input file that contains the query
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}


// Return the line in the input file, not sensitive to lower case
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn case_insensitive() {
        // Does the program return lines regardless of the letter cases?
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }


    #[test]
    fn case_sensitive() {
        // Does the program only return the line with the correct casing?
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}