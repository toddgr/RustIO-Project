use std::error::Error;
use std::fs;

// Holds input arguments, parsed with parse_config
pub struct Config {
    pub query: String,
    pub file_path: String,
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
        Ok(Config { query, file_path })
    }
}


// Reads the contents of the file
// Here we implement Box<dyn Error>, which lets us tell the function to return
// an error without specifying its type
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With test:\n{contents}");

    Ok(())
}