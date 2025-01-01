pub mod cat;
pub mod cd;
pub mod cmd_type;
pub mod echo;
pub mod exit;
pub mod pwd;

pub use cat::Cat;
pub use cd::Cd;
pub use cmd_type::CmdType;
pub use echo::Echo;
pub use exit::Exit;
pub use pwd::Pwd;

pub fn builtins() -> Vec<Box<dyn Command>> {
    vec![
        Box::new(Cat),
        Box::new(Cd),
        Box::new(CmdType),
        Box::new(Echo),
        Box::new(Exit),
        Box::new(Pwd),
    ]
}

#[derive(Default, Debug, PartialEq, Eq)]
pub enum CommandType {
    #[default]
    Builtin,
    External,
    Unknown,
}

pub trait Command {
    fn name(&self) -> &str;

    fn run(&self, groups: &[String]) -> anyhow::Result<()>;

    fn command_type(&self) -> CommandType {
        CommandType::default()
    }

    fn is_built_in(&self) -> bool {
        self.command_type() == CommandType::Builtin
    }

    fn is_external(&self) -> bool {
        self.command_type() == CommandType::External
    }

    fn is_unknown(&self) -> bool {
        self.command_type() == CommandType::Unknown
    }
}
