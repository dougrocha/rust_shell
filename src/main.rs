#[allow(unused_imports)]
use std::io::Write;
use std::{env, io::BufRead, path::Path, process::Command};

fn handle_command(args: Vec<&str>) {
    let mut args = args.iter();
    let Some(command) = args.next() else {
        return;
    };

    let rest: Vec<&str> = args.copied().collect();

    let rest = rest.join(" ");
    let rest: &str = rest.as_ref();

    match *command {
        "echo" => {
            println!("{}", rest);
        }
        "pwd" => {
            let pwd = std::env::current_dir().expect("current dir to exist");
            println!("{}", pwd.display());
        }
        "cd" => {
            let mut pwd = std::env::current_dir().expect("current dir to exist");
            if rest == "~" {
                pwd.push(std::env::var("HOME").expect("HOME var to exist"));
            } else {
                pwd.push(rest);
            }
            if std::env::set_current_dir(&pwd).is_err() {
                eprintln!("cd: {}: No such file or directory", pwd.display());
            }
        }
        "exit" => std::process::exit(rest.parse::<i32>().expect("code should be a valid number")),
        "type" => {
            if matches!(rest, "echo" | "exit" | "type" | "pwd" | "cd") {
                println!("{} is a shell builtin", rest);
            } else {
                let paths = std::env::var("PATH").expect("PATH should be set");

                if let Some(path) = env::split_paths(&paths).find_map(|path| {
                    let path = path.join(rest);
                    if path.is_file() {
                        return Some(path);
                    }
                    None
                }) {
                    println!(
                        "{} is {}",
                        rest,
                        path.into_os_string().into_string().unwrap()
                    );
                } else {
                    println!("{}: not found", rest);
                }
            }
        }
        _ => {
            let paths = std::env::var("PATH").expect("PATH should be set");

            if let Some(path) = env::split_paths(&paths).find_map(|path| {
                let path = path.join(command);
                if path.is_file() {
                    return Some(path);
                }
                None
            }) {
                Command::new(path.into_os_string().into_string().unwrap())
                    .arg(rest)
                    .status()
                    .expect("failed to execute process");
            } else {
                println!("{}: command not found", command);
            }
        }
    }
}

fn main() {
    let mut stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    loop {
        print!("$ ");
        stdout.flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).expect("stdin to read input");

        let args: Vec<&str> = input.trim_end().split(" ").collect();

        if args.is_empty() {
            continue;
        }

        handle_command(args);
    }
}
