use std::io::{stdin, stdout, Write};

fn main() {
    let mut command = String::new();

    loop {
        print!("$ ");
        stdout().flush().unwrap();

        stdin().read_line(&mut command).unwrap();
        command = command.trim().to_string();

        match command.as_str() {
            "exit" => break,
            _ if command.starts_with("echo ") => {
                println!("{}", &command[5..]);
            }
            _ => {
                println!("{}: command not found", command.trim());
            },
        }

        command.clear();
    }
}
