use std::env;
use std::io::{self, BufRead};

fn main() {
    // Get the user's home directory
    let home_dir = env::var("HOME").unwrap();

    // Read from stdin
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        match line {
            Ok(mut line) => {
                // Find the position of the home directory in the line
                if let Some(pos) = line.find("~") {
                    if pos == 0 {
                        // Replace "~" with home directory
                        line = line.replacen("~", &home_dir, 1);
                    }
                }
                // Print the line, preserving escape sequences
                println!("{}", line);
            }
            Err(_e) => {}
        }
    }
}
