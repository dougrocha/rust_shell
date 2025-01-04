use rust_shell::{handle_command, ShellContext};
use std::io::BufRead;
use std::io::Write;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    let mut shell_context = ShellContext::new();
    let mut input = String::new();

    loop {
        if input.is_empty() {
            print!("$ ");
        } else {
            print!("∙ ");
        }
        stdout.flush().unwrap();

        stdin.read_line(&mut input).expect("stdin to read input");

        if input.is_empty() {
            continue;
        }

        let res = handle_command(&mut shell_context, &input);

        match res {
            Ok(_) => {
                input.clear();
                continue;
            }
            Err(err) => {
                if err
                    .downcast_ref::<rust_shell::parser::UnclosedQuote>()
                    .is_some()
                {
                    continue;
                }

                eprintln!("{err:?}");
                input.clear();
            }
        };

        input.clear();
    }
}
