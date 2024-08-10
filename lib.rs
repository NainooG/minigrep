use std::fs;
use std::error::Error;

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
    let contents = fs::read_to_string(config.file_path)?;
        

    println!("With text:\n{contents}");

    // using () like this is idiomatic to indicate that we're calling run for its side effects only, it doesn't
    // return a value we need
    Ok(())
}