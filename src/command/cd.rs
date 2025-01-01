use super::Command;

pub struct Cd;

impl Command for Cd {
    fn name(&self) -> &str {
        "cd"
    }

    fn run(&self, groups: &[String]) -> anyhow::Result<()> {
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

        Ok(())
    }
}
