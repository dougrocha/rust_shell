use super::Command;

pub struct Exit;

impl Command for Exit {
    fn name(&self) -> &str {
        "exit"
    }

    fn run(&self, groups: &[String]) -> anyhow::Result<()> {
        let exit_code = groups
            .first()
            .and_then(|x| x.as_str().parse::<i32>().ok())
            .unwrap_or(0);

        std::process::exit(exit_code);
    }
}
