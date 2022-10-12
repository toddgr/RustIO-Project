# RustIO-Project
Rust command line tool `minigrep` that interacts with file and command line input/output. For practice implementing Rust! From the [Rust Programming Language Docs Tutorial](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).  

`minigrep` reads in a word or phrase and prints the lines of a given text file that contain that word or phrase. 

## Concepts
This project implements a number of Rust concepts, but I used this project to especially focus on:
* Organizing code according to Rust formatting
* Using vectors and strings
* Handling errors
* Using traits and lifetimes where appropriate
* Writing tests for Rust

## How to use

1. Download repository
2. Place text files in same directory as `minigrep`. Poem.txt is included as an example file
3. `cd minigrep`
4. Run `cargo run -- [query] [file path]`

> `minigrep` has an environment variable for case insensitivity. If you would like the program to return all lines of code that contain the query, regardless of whether or not it matches the case, run the following:  
> `IGNORE_CASE=1 cargo run -- [query] [file path]`    

 If you're using PowerShell, you will need to set the environment variable and run the program as separate commands:  
 `PS> $Env:IGNORE_CASE=1; cargo run -- [query] [file path]`  

 This will make IGNORE_CASE persist for the remainder of your shell session. To unset it, you can simply use the `Remove-Item` cmdlet:  
 `PS> Remove-Item Env:IGNORE_CASE`

## Examples