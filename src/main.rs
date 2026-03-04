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
            _ if command.starts_with("type ") => {
                let arg = &command[5..];

                match arg {
                    "echo" | "exit" | "type" => println!("{} is a shell builtin", arg),
                    _ => println!("{}: not found", arg),
                }
            }
            _ => {
                println!("{}: command not found", command.trim());
            },
        }

        command.clear();
    }
}
