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
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input: Vec<String> = parser::parse(&input);
        if input.is_empty() {
            continue;
        }
        match input[0].as_str() {
            "exit" => break, // Exit the shell
            "pwd" => println!("{}", env::current_dir().unwrap().display()),
            "cd" => builtins::cd(input),
            _ => executor::exec(&input),
        }
    }
}
