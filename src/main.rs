#[allow(unused_imports)]
use anyhow::anyhow;
use std::io::Write;
use std::path::Path;
use std::{env, fmt::Display, io::BufRead, process::Command};

#[derive(Debug)]
enum Group<'a> {
    SingleQuote(&'a str),
    DoubleQuote(&'a str),
    Default(&'a str),
}

impl<'a> Group<'a> {
    fn as_str(&self) -> &'a str {
        match self {
            Group::SingleQuote(s) | Group::DoubleQuote(s) | Group::Default(s) => s,
        }
    }
}

impl Display for Group<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

struct CaptureGroups<'a> {
    whole: &'a str,
    rest: &'a str,
    byte: usize,
}

impl<'a> CaptureGroups<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            whole: input,
            rest: input,
            byte: 0,
        }
    }
}

impl<'a> Iterator for CaptureGroups<'a> {
    type Item = anyhow::Result<Group<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut start_byte = self.byte;

        let mut chars = self.rest.chars();
        let c = chars.next()?;
        self.rest = chars.as_str();
        self.byte += 1;

        #[derive(Debug)]
        enum Started {
            SingleQuote,
            DoubleQuote,
            // Group that is surrounded by spaces
            Default,
        }

        let started = match c {
            '\'' => Started::SingleQuote,
            '\"' => Started::DoubleQuote,
            ' ' => return Some(Err(anyhow!("Will not parse space char"))),
            _ => Started::Default,
        };

        match started {
            Started::SingleQuote => {
                start_byte += 1;
                loop {
                    let Some(c) = chars.next() else {
                        self.byte += 1;
                        break;
                    };

                    self.byte += 1;
                    if c == '\'' {
                        break;
                    }
                }
            }
            Started::DoubleQuote => {
                return Some(Err(anyhow!("DoubleQuote not implemented yet!")));
            }
            Started::Default => loop {
                let Some(c) = chars.next() else {
                    self.byte += 1;
                    break;
                };

                self.byte += 1;
                if c == ' ' {
                    break;
                }
            },
        };

        let group = &self.whole[start_byte..self.byte - 1];
        self.rest = chars.as_str();

        Some(Ok(Group::Default(group)))
    }
}

fn handle_command(args: &str) {
    let (command, rest) = args
        .split_once(" ")
        .map(|(x, y)| (x.trim(), y.trim()))
        .unwrap_or((args.trim(), ""));

    let groups = CaptureGroups::new(rest);

    let groups: Vec<Group> = groups.filter_map(Result::ok).collect();

    match command {
        "echo" => {
            for group in groups {
                print!("{} ", group);
            }
            println!();
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
        "cat" => {
            groups
                .iter()
                .filter(|group| matches!(group, Group::Default(_) | Group::SingleQuote(_)))
                .for_each(|group| {
                    let file_path = Path::new(group.as_str());

                    let Ok(content) = std::fs::read_to_string(file_path) else {
                        panic!(
                            "cat: {}: No such file or directory",
                            file_path.to_string_lossy(),
                        );
                    };
                    print!("{}", content);
                });
        }
        "exit" => std::process::exit(rest.parse::<i32>().unwrap_or(0)),
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

        if input.is_empty() {
            continue;
        }

        handle_command(&input);
    }
}
