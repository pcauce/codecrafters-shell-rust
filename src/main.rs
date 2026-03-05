use std::io::{Write, stdin, stdout};
use codecrafters_shell::commands::run;

fn main() {
    loop {
        // Print command prompt
        print!("$ ");
        stdout().flush().unwrap();

        // Read user command
        let mut command = String::new();
        stdin().read_line(&mut command).unwrap();

        // Parse input and run it
        run(command.trim());
    }
}


