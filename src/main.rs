#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};

fn main() {
    print!("$ ");
    stdout().flush().unwrap();

    // Wait for user input
    let mut command = String::new();
    stdin().read_line(&mut command).unwrap();
    println!("{}: command not found", command.trim());
}
