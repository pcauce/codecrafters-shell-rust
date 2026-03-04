#[allow(unused_imports)]
use std::io::stdin;

fn main() {
    let mut command = String::new();

    print!("$ ");
    stdin().read_line(&mut command).unwrap();

    println!("{}: command not found", command.trim());

    io::stdout().flush().unwrap();
}
