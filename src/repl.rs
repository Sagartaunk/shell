use crate::builtins;
use crate::executor;
use crate::parser;
use std::env;
use std::io::{self, Write};

pub fn run() {
    loop {
        print!("{} -> ", env::current_dir().unwrap().display());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
        let input: Vec<String> = parser::parse(&input);
        if input.is_empty() {
            continue;
        }
        match input[0].as_str() {
            "exit" => break, // Exit the shell
            "pwd" => builtins::pwd(),
            "cd" => builtins::cd(&input),
            "echo" => println!("{}", builtins::echo(&input)),
            _ => executor::exec(&input),
        }
    }
}
