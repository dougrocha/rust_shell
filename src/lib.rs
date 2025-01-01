use command::Command;
use parser::Parser;
use std::collections::HashMap;

pub mod command;
pub mod parser;
pub mod tokenizer;

pub struct ShellContext {
    pub commands: HashMap<String, Box<dyn Command>>,
}

impl Default for ShellContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ShellContext {
    pub fn new() -> Self {
        let mut shell_context = ShellContext {
            commands: HashMap::new(),
        };

        for command in command::builtins() {
            shell_context
                .commands
                .insert(command.name().to_string(), command);
        }

        shell_context
    }
}

pub fn handle_command(context: &mut ShellContext, args: &str) {
    let groups: Vec<String> = Parser::new(args).filter_map(Result::ok).collect();

    let Some(command) = groups.first().cloned() else {
        println!();
        return; // No command provided
    };
    let command_args = &groups[1..];

    let builtins = &context.commands;

    if let Some(command) = builtins.values().find(|x| x.name() == command.as_str()) {
        if let Err(err) = command.run(command_args) {
            eprintln!("Error running command: {}", err);
        }
    } else {
        let paths = std::env::var("PATH").expect("PATH should be set");

        if let Some(path) = std::env::split_paths(&paths).find_map(|path| {
            let path = path.join(&command);
            if path.is_file() {
                Some(path)
            } else {
                None
            }
        }) {
            let status = std::process::Command::new(path)
                .args(groups.iter().map(|x| x.as_str()))
                .status();

            match status {
                Ok(status) if status.success() => {}
                Ok(status) => eprintln!("Command exited with status: {}", status),
                Err(err) => eprintln!("Failed to execute command: {}", err),
            }
        } else {
            eprintln!("{}: command not found", command);
        }
    };
}
