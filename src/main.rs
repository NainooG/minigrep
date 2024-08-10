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
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();
    
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been aple to read the file");

    println!("With text:\n{contents}");
}
