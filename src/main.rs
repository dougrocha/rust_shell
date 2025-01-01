use rust_shell::{handle_command, ShellContext};
use std::io::BufRead;
use std::io::Write;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    let mut shell_context = ShellContext::new();

    loop {
        print!("$ ");
        stdout.flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).expect("stdin to read input");

        if input.is_empty() {
            continue;
        }

        handle_command(&mut shell_context, &input);
    }
}
