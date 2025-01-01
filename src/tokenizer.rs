#[derive(Default, Debug, Clone, Copy)]
enum FileDescriptor {
    Stdin = 0,
    #[default]
    Stdout = 1,
    Stderr = 2,
}

enum Operator {
    Append(FileDescriptor),   // >, <, 2>
    Redirect(FileDescriptor), // >>, 2>>
    Pipe,                     // |
    And,                      // &&
    Or,                       // ||
}

enum ShellToken {
    Command(String),
    Argument(String),
    Operator(FileDescriptor),
}
