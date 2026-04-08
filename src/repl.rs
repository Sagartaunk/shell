use crate::builtins;
use crate::executor;
use crate::parser;
use nix::sys::signal::{SigHandler, Signal, signal};
use nix::sys::wait::{WaitPidFlag, WaitStatus, waitpid};
use nix::unistd::Pid;
use std::env;
use std::io::{self, Write};

pub fn run() {
    let mut jobs: Vec<executor::Job> = Vec::new(); // Stores background jobs , Todo : Make it store their state
    unsafe {
        signal(Signal::SIGINT, SigHandler::SigIgn).unwrap();
        signal(Signal::SIGTSTP, SigHandler::SigIgn).unwrap();
        signal(Signal::SIGQUIT, SigHandler::SigIgn).unwrap();
        signal(Signal::SIGTTOU, SigHandler::SigIgn).unwrap();
        signal(Signal::SIGTTIN, SigHandler::SigIgn).unwrap();
    };
    loop {
        reap_jobs(&mut jobs);
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
                "fg" => builtins::fg(&mut jobs, input.command[0].args[1].parse().unwrap()),
                "bg" => builtins::bg(&mut jobs, input.command[0].args[1].parse().unwrap()),
                _ => executor::exec(&input, &mut jobs),
            }
            continue;
        }
        executor::exec_pipe(&input, &mut jobs);
    }
}
fn reap_jobs(jobs: &mut Vec<executor::Job>) {
    jobs.retain(|job| {
        loop {
            match waitpid(
                Pid::from_raw(-job.pgid.as_raw()),
                Some(WaitPidFlag::WNOHANG),
            ) {
                Ok(WaitStatus::Exited(pid, status)) => {
                    println!(
                        "[{}] process {} exited with status {}",
                        job.pgid, pid, status
                    );
                    continue;
                }
                Ok(WaitStatus::Signaled(pid, sig, _)) => {
                    println!("[{}] process {} terminated by {}", job.pgid, pid, sig);
                    continue;
                }
                Ok(WaitStatus::StillAlive) => return true,
                Err(nix::errno::Errno::ECHILD) => {
                    println!("[{}] job finished", job.pgid);
                    return false;
                }
                Err(e) => {
                    eprintln!("waitpid error for [{}]: {}", job.pgid, e);
                    return true;
                }
                _ => return true,
            }
        }
    });
}
