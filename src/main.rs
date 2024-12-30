#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();

        let mut input = String::new();
        stdin.read_line(&mut input).expect("stdin to read input");

        let Some(command) = input.trim_end().split_once(" ") else {
            println!("{}: command not found", input.trim());
            continue;
        };

        match command {
            ("echo", str) => println!("{}", str),
            ("exit", _) => std::process::exit(0),
            (command, _) => println!("{}: command not found", command),
        }
    }
}
