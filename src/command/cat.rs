use super::Command;
use std::path::Path;

pub struct Cat;

impl Command for Cat {
    fn name(&self) -> &str {
        "cat"
    }

    fn run(&self, groups: &[String]) -> anyhow::Result<()> {
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

        Ok(())
    }
}
