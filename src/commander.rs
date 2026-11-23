pub struct Commands;

impl Commands {
    const COMMAND_LIST: [&str; 5] = ["exit", "help", "add", "commit", "diff"];

    const EXIT: &str = Self::COMMAND_LIST[0];
    const HELP: &str = Self::COMMAND_LIST[1];
    const ADD: &str = Self::COMMAND_LIST[2];
    const COMMIT: &str = Self::COMMAND_LIST[3];
    const DIFF: &str = Self::COMMAND_LIST[4];

    pub fn is_exit(input: &str) -> bool {
        input.split_whitespace().next() == Some(Self::EXIT)
    }

    pub fn command_handler(command: &str) {
        dbg!(command);
        match command.trim() {
            Self::EXIT => println!("working {}", command),
            Self::HELP => println!("working {}", command),
            Self::ADD => println!("working {}", command),
            Self::COMMIT => println!("working {}", command),
            Self::DIFF => println!("working {}", command),
            _ => println!("Command not found!"),
        }
    }
}
