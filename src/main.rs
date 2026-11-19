use std::io::{Write, stdin, stdout};

fn main() {
    let mut input = String::new();

    loop {
        input_handler(&mut input);
        if Commands::is_exit(&input) {
            break;
        };
        Commands::command_handler(&input);
    }
}

struct Commands {}

impl Commands {
    const COMMAND_LIST: [&str; 5] = ["exit", "help", "add", "commit", "diff"];

    const EXIT: &str = Self::COMMAND_LIST[0];
    const HELP: &str = Self::COMMAND_LIST[1];
    const ADD: &str = Self::COMMAND_LIST[2];
    const COMMIT: &str = Self::COMMAND_LIST[3];
    const DIFF: &str = Self::COMMAND_LIST[4];

    fn is_exit(input: &str) -> bool {
        match input.split_once(" ") {
            Some(res) => res.0.trim() == Self::EXIT,
            _ => false,
        }
    }
    fn command_handler(command: &str) {
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

fn input_handler(input: &mut String) {
    print!("> ");
    stdout().flush().unwrap();
    if !input.is_empty() {
        input.clear();
    }

    match stdin().read_line(input) {
        Ok(_) => (),
        Err(error) => println!("not working! {}", error),
    };
}
