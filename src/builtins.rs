use crate::executor::Job;
use crate::parser;
use nix::sys::signal::{SIGCONT, killpg};
use nix::sys::wait::{WaitPidFlag, WaitStatus, waitpid};
use nix::unistd::{Pid, getpgrp, tcsetpgrp};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
pub fn cd(pat: &parser::Command) {
    let path: &[String] = &pat.args;
    if path.len() < 2 {
        let home = match env::var("HOME") {
            Ok(val) => val,
            Err(_) => {
                env::set_current_dir("/").expect("failed to change dir");
                "/".to_string()
            }
        };
        match std::env::set_current_dir(home) {
            Ok(_) => return,
            Err(e) => {
                println!("{}", e)
            }
        };
        return;
    }
    match env::set_current_dir(&path[1]) {
        Ok(_) => return,
        Err(e) => {
            println!("Error : {}", e);
            return;
        }
    }
}
pub fn echo(input: &parser::Command) {
    let comm: String = input.args[1..].join(" ");
    if input.stdout.is_some() {
        let file = match input.append {
            true => OpenOptions::new()
                .create(true)
                .append(true)
                .open(input.stdout.as_ref().unwrap()),
            false => OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(input.stdout.as_ref().unwrap()),
        };
        match writeln!(file.unwrap(), "{}", comm) {
            Ok(_) => return,
            Err(e) => {
                println!("Error : {}", e);
                return;
            }
        };
    }
    println!("{}", comm);
}
pub fn pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("{}", e),
    }
}
pub fn jobs(job: &[Job]) {
    println!("Index Status Command PGID");
    for i in 0..job.len() {
        println!(
            "{} {:?} {} {}",
            i + 1,
            job[i].state,
            job[i].command,
            job[i].pgid
        );
    }
}
pub fn fg(jobs: &mut Vec<Job>, jobid: usize) {
    if jobid > jobs.len() {
        eprintln!("Index out of bounds");
        return;
    } else {
        let pgid = jobs[jobid - 1].pgid;
        match killpg(pgid, SIGCONT) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };
        match tcsetpgrp(std::io::stdin(), pgid) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };
        match waitpid(Pid::from_raw(-pgid.as_raw()), Some(WaitPidFlag::WUNTRACED)) {
            Ok(WaitStatus::Exited(_, _)) | Ok(WaitStatus::Signaled(_, _, _)) => {
                tcsetpgrp(std::io::stdin(), getpgrp()).unwrap();
                jobs.remove(jobid - 1);
            }
            Ok(WaitStatus::Stopped(_, _)) => {
                jobs[jobid - 1].state = crate::executor::JobState::Suspended;
                tcsetpgrp(std::io::stdin(), getpgrp()).unwrap();
            }
            _ => {}
        }
    }
}
pub fn bg(jobs: &mut Vec<Job>, jobid: usize) {
    if jobid > jobs.len() {
        eprintln!("Index out of bounds");
        return;
    } else {
        let pgid = jobs[jobid - 1].pgid;
        match killpg(pgid, SIGCONT) {
            Ok(_) => {
                jobs[jobid - 1].state = crate::executor::JobState::Running;
                return;
            }
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };
    }
}
