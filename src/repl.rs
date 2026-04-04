use crate::builtins;
use crate::executor;
use crate::parser;
use nix::sys::signal::{SigHandler, Signal, signal};
use std::env;
use std::io::{self, Write};

pub fn run() {
    loop {
        unsafe {
            signal(Signal::SIGINT, SigHandler::SigIgn).unwrap();
            signal(Signal::SIGTSTP, SigHandler::SigIgn).unwrap();
            signal(Signal::SIGQUIT, SigHandler::SigIgn).unwrap();
        };
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
        let input: Vec<parser::Command> = parser::parse(&input);
        if input.is_empty() || input[0].args.is_empty() {
            continue;
        }
        if input.len() == 1 {
            match input[0].args[0].as_str() {
                "exit" => break, // Exit the shell
                "pwd" => builtins::pwd(),
                "cd" => builtins::cd(&input[0]),
                "echo" => builtins::echo(&input[0]),
                _ => executor::exec(&input[0]),
            }
            continue;
        }
        executor::exec_pipe(&input);
    }
}
