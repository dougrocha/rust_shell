use super::Command;

pub struct Pwd;

impl Command for Pwd {
    fn name(&self) -> &str {
        "pwd"
    }

    fn run(&self, _groups: &[String]) -> anyhow::Result<()> {
        let pwd = std::env::current_dir().expect("current dir to exist");
        println!("{}", pwd.display());

        Ok(())
    }
}
