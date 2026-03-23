use crate::executor;
use crate::parser;
use std::io::{self, Write};
pub fn run() {
    let path = String::from("$");
    loop {
        print!("{} -> ", path);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = parser::parse(&input);
        if input.is_empty() {
            continue;
        }
        match input[0].as_str() {
            "exit" => break, // Exit the shell
            _ => executor::exec(&input),
        }
    }
}
