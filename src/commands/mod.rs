use pathsearch::find_executable_in_path;

pub fn run(input: &str) {
    if let Some((name, arg)) = input.split_once(' ') {
        match name {
            "echo" => println!("{}", arg),
            "type" => run_type(arg),
            _ => println!("{}: command not found", name)
        }
    } else {
        match input {
            "exit" => std::process::exit(0),
            _ => println!("{}: command not found", input)
        }
    }
}

fn run_type(arg: &str) {
    match arg {
        "echo" | "type" | "exit" => println!("{} is a shell builtin", arg),
        _ => {
            if let Some(path) = find_executable_in_path(arg) {
                println!("{} is {}", arg, path.display());
            } else {
                println!("{}: not found", arg);
            }
        },
    }
}
