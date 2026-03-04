use std::io::{stdin, stdout, Write};

fn main() {
    let mut command = String::new();

    loop {
        print!("$ ");
        stdout().flush().unwrap();

        stdin().read_line(&mut command).unwrap();
        command = command.trim().to_string();
        if command == "exit" {
            break;
        } else {
            println!("{}: command not found", command.trim());
            command.clear();
        }
    }
}
