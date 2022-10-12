use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // Args will panic if any argument contains invalid Unicode.
    // If the program needs to accept arguments containing invalid
    // Unicode, use std::env::args_os instead
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}

