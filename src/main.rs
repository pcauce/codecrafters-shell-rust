#[allow(unused_imports)]
use std::io::{self, stdin, stdout, Write};

fn main() {
    let mut command = String::new();

    print!("$ ");
    stdin().read_line(&mut command).unwrap();

    println!("{}: command not found", command.trim());

    stdout().flush().unwrap();
}
