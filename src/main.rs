#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();

        let mut input = String::new();
        stdin.read_line(&mut input).expect("stdin to read input");

        match input.trim_end() {
            "exit 0" => std::process::exit(0),
            _ => {
                println!("{}: command not found", input.trim());
            }
        }
    }
}
