use std::io::{Write, stdin, stdout};

use crate::commander::Commands;
use crate::encriptor::Encriptor;
use crate::file_handler::FileHandler;
pub mod commander;
pub mod encriptor;
pub mod file_handler;

fn main() {
    let value_to_encript = &String::from("This is the value I wanto to encript");

    let encriptor_value = Encriptor::encript(value_to_encript);
    println!("{:?}", encriptor_value);
    // loop {
    //     input_handler(&mut input);
    //     if Commands::is_exit(&input) {
    //         break;
    //     } else {
    //         Commands::command_handler(&input);
    //     }
    // }
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
