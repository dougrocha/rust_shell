use command::Command;
use miette::LabeledSpan;
use parser::Parser;
use std::{collections::HashMap, path::PathBuf};

pub mod command;
pub mod parser;

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

pub fn handle_command(
    context: &mut ShellContext,
    input: &str,
) -> anyhow::Result<(), miette::Error> {
    let parser = Parser::new(input);

    // Handle parser error
    if let Some(err) = parser.filter_map(Result::err).next() {
        return Err(err);
    }

    let groups: Vec<String> = parser.filter_map(Result::ok).collect();

    let command = if let Some(command) = groups.first() {
        command.trim()
    } else {
        return Ok(());
    };

    let command_args = &groups[1..];

    let builtins = &context.commands;

    if let Some(command) = builtins.values().find(|x| x.name() == command) {
        // TODO: use question mark and throw error inside of run
        if let Err(err) = command.run(command_args) {
            eprintln!("Error running command: {}", err);
        }
    } else {
        let paths: Vec<PathBuf> =
            std::env::split_paths(&std::env::var("PATH").expect("PATH should be set")).collect();

        if let Some(command_path) = find_command_in_path(&paths, command) {
            let status = std::process::Command::new(command_path)
                .args(groups.iter().map(|x| x.as_str()))
                .status();

            match status {
                Ok(status) if status.success() => {}
                Ok(status) => eprintln!("Command exited with status: {}", status),
                Err(err) => eprintln!("Failed to execute command: {}", err),
            }
        } else {
            let cmd_len = input.find(" ").unwrap_or(input.len());

            return Err(miette::miette! {
                labels = vec![
                    LabeledSpan::at(0..cmd_len, "here"),
                ],
                help = format!("{command:?} is neither a built-in or a known external command"),
                "Command failed to run: {command:?}",
            }
            .with_source_code(input.to_string()));
        }
    };

    Ok(())
}

fn find_command_in_path(paths: &[PathBuf], command: &str) -> Option<PathBuf> {
    paths.iter().find_map(|path| {
        let path = path.join(command);
        if path.is_file() {
            Some(path)
        } else {
            None
        }
    })
}
