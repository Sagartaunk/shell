use crate::parser;
use nix::sys::wait::waitpid;
use nix::unistd::{ForkResult, close, execvp, fork, pipe};
use std::fs::OpenOptions;
use std::os::unix::io::{IntoRawFd, RawFd};
pub fn exec(comm: &parser::Command) {
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            let arg = parser::cstring(&comm.args);
            if comm.stdin.is_some() {
                let file = OpenOptions::new()
                    .read(true)
                    .open(comm.stdin.as_ref().unwrap())
                    .expect("Failed to open file");
                let fd = file.into_raw_fd();
                unsafe {
                    libc::dup2(fd, 0);
                }
            }
            if comm.stdout.is_some() {
                let file = match comm.append {
                    true => OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(comm.stdout.as_ref().unwrap()),
                    false => OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(comm.stdout.as_ref().unwrap()),
                };
                let fd = file.unwrap().into_raw_fd();
                unsafe {
                    libc::dup2(fd, 1);
                }
            }
            let args: Vec<&std::ffi::CStr> = arg.iter().map(|s| s.as_c_str()).collect();
            match execvp(args[0], &args) {
                Ok(_) => return,
                Err(e) => println!("{}", e),
            }
            std::process::exit(1);
        }
        Ok(ForkResult::Parent { child }) => {
            waitpid(child, None).unwrap();
        }
        Err(e) => {
            eprintln!("Error : {}", e);
        }
    }
}

//Todo -> Restructure exec_pipe according to new parser
pub fn exec_pipe(args: &Vec<Vec<String>>) {
    let mut pipes: Vec<(RawFd, RawFd)> = Vec::new();
    let mut pids = vec![];
    for _i in 0..(args.len() - 1) {
        let (read_fd, write_fd) = pipe().unwrap();
        let read = read_fd.into_raw_fd();
        let write = write_fd.into_raw_fd();
        pipes.push((read, write));
    }
    for i in 0..(args.len()) {
        match unsafe { fork() } {
            Ok(ForkResult::Child) => {
                if i == 0 {
                    unsafe {
                        libc::dup2(pipes[i].1, 1);
                    };
                    for p in &pipes {
                        close(p.0).unwrap();
                        close(p.1).unwrap();
                    }
                    let command = parser::cstring(&args[i]);
                    let comm: Vec<&std::ffi::CStr> = command.iter().map(|c| c.as_c_str()).collect();
                    match execvp(comm[0], &comm) {
                        Ok(_) => continue,
                        Err(e) => eprintln!("{}", e),
                    }
                    std::process::exit(1);
                } else if i == args.len() - 1 {
                    unsafe { libc::dup2(pipes[i - 1].0, 0) };
                    for p in &pipes {
                        close(p.0).unwrap();
                        close(p.1).unwrap();
                    }
                    let command = parser::cstring(&args[i]);
                    let comm: Vec<&std::ffi::CStr> = command.iter().map(|c| c.as_c_str()).collect();
                    match execvp(comm[0], &comm) {
                        Ok(_) => continue,
                        Err(e) => eprintln!("{}", e),
                    }
                    std::process::exit(1);
                } else {
                    unsafe { libc::dup2(pipes[i - 1].0, 0) };
                    unsafe {
                        libc::dup2(pipes[i].1, 1);
                    };
                    for p in &pipes {
                        close(p.0).unwrap();
                        close(p.1).unwrap();
                    }
                    let command = parser::cstring(&args[i]);
                    let comm: Vec<&std::ffi::CStr> = command.iter().map(|c| c.as_c_str()).collect();
                    match execvp(comm[0], &comm) {
                        Ok(_) => continue,
                        Err(e) => eprintln!("{}", e),
                    }
                    std::process::exit(1);
                }
            }
            Ok(ForkResult::Parent { child }) => {
                pids.push(child);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    for p in &pipes {
        close(p.0).unwrap();
        close(p.1).unwrap();
    }
    for pid in pids.iter() {
        waitpid(*pid, None).unwrap();
    }
}
