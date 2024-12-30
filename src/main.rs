#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env::{self},
    fs,
    path::{Path, PathBuf},
};

enum Command {
    Echo,
    Exit,
    Type,
}

impl From<&str> for Command {
    fn from(command_str: &str) -> Self {
        match command_str {
            "echo" => Self::Echo,
            "exit" => Self::Exit,
            "type" => Self::Type,
            _ => panic!("command does not exist"),
        }
    }
}

impl Command {
    fn handle_command(self, rest: &str) {
        let rest = rest.trim();

        match self {
            Command::Echo => {
                println!("{}", rest);
            }
            Command::Exit => {
                std::process::exit(rest.parse::<i32>().expect("code should be a valid number"))
            }
            Command::Type => {
                if matches!(rest, "echo" | "exit" | "type") {
                    println!("{} is a shell builtin", rest);
                } else {
                    let paths = env::var("PATH").expect("PATH should be set");

                    for path in paths.split(":") {
                        let mut path = PathBuf::from(path);
                        path.push(rest);

                        if path.exists() {
                            println!(
                                "{} is {}",
                                rest,
                                path.to_str().expect("Path should be shown as string")
                            );
                            return;
                        }
                    }

                    println!("{}: not found", rest);
                }
            }
        }
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();

        let mut input = String::new();
        stdin.read_line(&mut input).expect("stdin to read input");

        let Some((command, rest)) = input.trim_end().split_once(" ") else {
            println!("{}: command not found", input.trim());
            continue;
        };

        let command = Command::from(command);

        command.handle_command(rest);
    }
}
