use std::fs::File;
use std::error::Error;
use std::io::Read;
use std::env;

// IGNORE_CASE=1 cargo run -- to poem.txt set ignore_case so that we can do a case insensitive search.

// use struct to convey relationship between query and file_path in configuring program
pub struct Config {
    pub query: String, 
    pub file_path: String,
    pub ignore_case: bool,
}

// implement constructor for Config struct to make code more idiomatic
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // acceptable to use clone here, despite slight loss of performance for simplicity
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { 
            query, 
            file_path, 
            ignore_case 
        })
    }
}

// Box<dyn Error> means that the function will return a type that implements the Error trait,
// but we don't have to specify what particular tpye the return value will be
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.file_path)?;

    let mut contents = String::new();

    let _ = file.read_to_string(&mut contents);

    // check which kind of search we want to do 
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
        
    // using () like this is idiomatic to indicate that we're calling run for its side effects only, it doesn't
    // return a value we need
    Ok(())
}

// use lifetime parameters here to indicate that the returned vector should 
// contain string slices that reference slices of the argument contents (rather than the argument query)

// in other words, data returned by search will live as long as the data passed into the search
// function in the contents argument
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // lines method returns iterator
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// allow case insensitive search to work by lowercasing query and each line of contents
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    // .to_lowercase() makes query a String because it creates new data and probably returns a String
    let query = query.to_lowercase();
    let mut results =  Vec::new();

    // pass in query with ampersand because contains method is defined to take in a string slice
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
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
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}