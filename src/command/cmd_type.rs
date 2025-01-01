use super::Command;
use std::env;

pub struct CmdType;

impl Command for CmdType {
    fn name(&self) -> &str {
        "type"
    }

    fn run(&self, groups: &[String]) -> anyhow::Result<()> {
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

        Ok(())
    }
}
