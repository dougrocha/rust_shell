#[allow(unused_imports)]
use anyhow::anyhow;
use std::io::Write;
use std::path::Path;
use std::{env, io::BufRead, process::Command};

#[derive(Clone, Copy)]
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

impl Iterator for CaptureGroups<'_> {
    type Item = anyhow::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.rest.chars();
        let c = chars.next()?;
        self.byte += c.len_utf8();

        #[derive(Debug)]
        enum Started {
            SingleQuote,
            DoubleQuote,
            BackSlash,
            Spaces,
            // Group that is surrounded by spaces
            Default,
        }

        let started = match c {
            '\'' => Started::SingleQuote,
            '\"' => Started::DoubleQuote,
            '\\' => Started::BackSlash,
            ' ' => Started::Spaces,
            '\n' => return None,
            _ => Started::Default,
        };

        match started {
            Started::SingleQuote => {
                let stop = self.rest[1..]
                    .find('\'')
                    .map(|x| x + 1)
                    .unwrap_or(self.rest.len());

                let group = &self.rest[1..stop];
                self.rest = &self.rest[stop + 1..];
                self.byte = stop + 1;

                Some(Ok(group.trim().to_string()))
            }
            Started::DoubleQuote => {
                let stop = self.rest[1..]
                    .find('\"')
                    .map(|x| x + 1)
                    .unwrap_or(self.rest.len());

                let group = &self.rest[1..stop];
                self.rest = &self.rest[stop + 1..];
                self.byte = stop + 1;

                Some(Ok(group.trim().to_string()))
            }
            Started::BackSlash => {
                let symbol = chars.next().unwrap_or_default();
                self.rest = chars.as_str();
                self.byte += symbol.len_utf8();

                Some(Ok(symbol.to_string()))
            }
            Started::Default => {
                let stop = self.rest.find([' ', '\\']).unwrap_or(self.rest.len());

                let group = &self.rest[..stop];
                self.rest = &self.rest[stop..];
                self.byte = stop;

                Some(Ok(group.trim().to_string()))
            }
            Started::Spaces => {
                let stop = self.rest.find(|c| c != ' ').unwrap_or(self.rest.len());

                self.rest = &self.rest[stop..];
                self.byte += stop;

                Some(Ok(" ".to_string()))
            }
        }
    }
}

fn handle_command(args: &str) {
    let (command, rest) = args
        .split_once(" ")
        .map(|(x, y)| (x.trim(), y.trim_start()))
        .unwrap_or((args.trim(), ""));

    let groups = CaptureGroups::new(rest.trim());

    let groups: Vec<_> = groups.filter_map(Result::ok).collect();

    match command {
        "echo" => {
            for group in groups {
                print!("{}", group);
            }
            println!();
        }
        "pwd" => {
            let pwd = std::env::current_dir().expect("current dir to exist");
            println!("{}", pwd.display());
        }
        "cd" => {
            if groups.len() > 1 {
                eprintln!("cd: too many arguments");
            }

            let dir = groups.first().map(|x| x.as_str().trim()).unwrap_or("~");

            let mut pwd = std::env::current_dir().expect("current dir to exist");

            if dir == "~" {
                pwd.push(std::env::var("HOME").expect("HOME var to exist"));
            } else {
                pwd.push(dir);
            }

            if std::env::set_current_dir(&pwd).is_err() {
                eprintln!("cd: {}: No such file or directory", pwd.display());
            }
        }
        "cat" => {
            groups
                .iter()
                .filter(|x| !x.trim().is_empty())
                .for_each(|group| {
                    let file_path = Path::new(group.as_str().trim());

                    let Ok(content) = std::fs::read_to_string(file_path) else {
                        panic!(
                            "cat: {}: No such file or directory",
                            file_path.to_string_lossy(),
                        );
                    };
                    print!("{}", content);
                });
        }
        "exit" => {
            let exit_code = groups
                .first()
                .and_then(|x| x.as_str().parse::<i32>().ok())
                .unwrap_or(0);

            std::process::exit(exit_code);
        }
        "type" => {
            let commands: Vec<_> = groups.iter().map(|x| x.as_str().trim()).collect();

            for command in commands {
                if matches!(command, "echo" | "exit" | "type" | "pwd" | "cd") {
                    println!("{} is a shell builtin", command);
                } else {
                    let paths = std::env::var("PATH").expect("PATH should be set");

                    if let Some(path) = env::split_paths(&paths).find_map(|path| {
                        let path = path.join(command);
                        if path.is_file() {
                            return Some(path);
                        }
                        None
                    }) {
                        println!(
                            "{} is {}",
                            command,
                            path.into_os_string().into_string().unwrap()
                        );
                    } else {
                        println!("{}: not found", command);
                    }
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
                    .arg(rest.trim())
                    .status()
                    .expect("failed to execute process");
            } else {
                eprintln!("{}: command not found", command);
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
