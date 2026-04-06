use crate::builtins;
use crate::executor;
use crate::parser;
use nix::sys::signal::{SigHandler, Signal, signal};
use std::env;
use std::io::{self, Write};

pub fn run() {
    let mut jobs: Vec<executor::Job> = Vec::new(); // Todo : Make this store background jobs with their state
    unsafe {
        signal(Signal::SIGINT, SigHandler::SigIgn).unwrap();
        signal(Signal::SIGTSTP, SigHandler::SigIgn).unwrap();
        signal(Signal::SIGQUIT, SigHandler::SigIgn).unwrap();
    };
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
        let input: parser::Commands = parser::parse(&input);
        if input.command.is_empty() || input.command[0].args.is_empty() {
            continue;
        }
        if input.command.len() == 1 {
            match input.command[0].args[0].as_str() {
                "exit" => break, // Exit the shell
                "pwd" => builtins::pwd(),
                "cd" => builtins::cd(&input.command[0]),
                "echo" => builtins::echo(&input.command[0]),
                "jobs" => builtins::jobs(&jobs),
                _ => executor::exec(&input, &mut jobs),
            }
            continue;
        }
        executor::exec_pipe(&input, &mut jobs);
    }
}
