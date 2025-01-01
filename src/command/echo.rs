use super::Command;

pub struct Echo;

impl Command for Echo {
    fn name(&self) -> &str {
        "echo"
    }

    fn run(&self, groups: &[String]) -> anyhow::Result<()> {
        // Skip 1 since it is usually the first space after calling echo
        for group in groups.iter().skip(1) {
            print!("{}", group);
        }
        println!();

        Ok(())
    }
}
