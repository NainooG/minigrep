use std::fs::File;
use std::error::Error;
use std::io::Read;

// use struct to convey relationship between query and file_path in configuring program
pub struct Config {
    pub query: String, 
    pub file_path: String,
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

        Ok(Config { query, file_path })
    }
}

// Box<dyn Error> means that the function will return a type that implements the Error trait,
// but we don't have to specify what particular tpye the return value will be
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.file_path)?;

    let mut contents = String::new();

    let _ = file.read_to_string(&mut contents);

    for line in search(&config.query, &contents) {
        println!("{}", line);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}