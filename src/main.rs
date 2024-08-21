    /* let args: Vec<String> = env::args().collect();
    dbg!(args);
    [src/main.rs:7:5] args = [
    "target\\debug\\minigrep.exe",
    "needle",
    "haystack",
    ]  
    */ 

// bring env into scope
use std::env;
use std::process;

use minigrep::Config;

fn main() {

    let args: Vec<String> = env::args().collect();

    // inform user of problem parsing arguments by printing error, and exit with nonzero error code 
    // to indicate our program exited with an error state
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}





