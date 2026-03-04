#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};

fn main() {
    let mut command = String::new();

    loop {
        print!("$ ");
        stdout().flush().unwrap();

        stdin().read_line(&mut command).unwrap();
        println!("{}: command not found", command.trim());
    }
}
