use std::env;
use std::fs;

fn main() {
    // Args will panic if any argument contains invalid Unicode.
    // If the program needs to accept arguments containing invalid
    // Unicode, use std::env::args_os instead
    let args: Vec<String> = env::args().collect();
    
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    // Read the contents of a file
    let contents = fs::read_to_string(file_path)
    .expect("Failed to read file");

    println!("With text: \n{contents}"); // Print text
}
