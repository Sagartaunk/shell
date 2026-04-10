use crate::parser;
use nix::sys::wait::{WaitPidFlag, WaitStatus, waitpid};
use nix::unistd::{ForkResult, Pid, execvp, fork, pipe, setpgid};
use nix::unistd::{getpgrp, tcsetpgrp};
use std::fs::OpenOptions;
use std::os::fd::{AsRawFd, IntoRawFd};
use std::os::unix::io::OwnedFd;

#[derive(Debug)]
pub enum JobState {
    Running,
    Suspended,
}

pub struct Job {
    pub pgid: Pid,
    pub state: JobState,
    pub command: String,
}
fn reset_signals() {
    //Reset signal handelling for processes, The shell ignores signals otherwise
    unsafe {
        libc::signal(libc::SIGTSTP, libc::SIG_DFL);
        libc::signal(libc::SIGINT, libc::SIG_DFL);
        libc::signal(libc::SIGQUIT, libc::SIG_DFL);
        libc::signal(libc::SIGTTOU, libc::SIG_DFL);
        libc::signal(libc::SIGTTIN, libc::SIG_DFL);
    }
}

pub fn exec(command: &parser::Commands, jobs: &mut Vec<Job>) {
    //Only for single commands
    let comm = &command.command;
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            reset_signals();
            setpgid(Pid::from_raw(0), Pid::from_raw(0)).expect("Failed to set PGID");
            let arg = parser::cstring(&comm[0].args);
            if comm[0].stdin.is_some() {
                let file = OpenOptions::new()
                    .read(true)
                    .open(comm[0].stdin.as_ref().unwrap())
                    .expect("Failed to open file");
                let fd = file.into_raw_fd();
                unsafe {
                    match libc::dup2(fd, 0) {
                        -1 => {
                            eprintln!("Error Reading File Discryptor");
                            libc::close(fd);
                            libc::_exit(1); // Exit if dup2 fails
                        }
                        _ => {}
                    };
                }
            }
            if comm[0].stdout.is_some() {
                let file = match comm[0].append {
                    true => OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open(comm[0].stdout.as_ref().unwrap()),
                    false => OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(comm[0].stdout.as_ref().unwrap()),
                }
                .expect("Failed to open output file");
                let fd = file.into_raw_fd();
                unsafe {
                    match libc::dup2(fd, 1) {
                        -1 => {
                            eprintln!("Error");
                            libc::close(fd);
                            libc::_exit(1); // Exit if dup2 fails with
                        }
                        _ => {}
                    };
                }
            }
            let args: Vec<&std::ffi::CStr> = arg.iter().map(|s| s.as_c_str()).collect();
            match execvp(args[0], &args) {
                Ok(_) => unreachable!(), // Successful case, code should end here
                Err(e) => eprintln!("{}", e),
            }
            unsafe { libc::_exit(1) }; // If code reaches here exit with an error
        }
        Ok(ForkResult::Parent { child }) => {
            setpgid(child, child).expect("Failed to set pgid");
            let name: String = parser::construct_string(&comm);
            if command.bg {
                jobs.push(Job {
                    pgid: child,
                    state: JobState::Running,
                    command: name,
                });
            } else {
                tcsetpgrp(std::io::stdin(), child).unwrap();
                match waitpid(child, Some(WaitPidFlag::WUNTRACED)) {
                    Ok(WaitStatus::Exited(_, _)) => {
                        tcsetpgrp(std::io::stdin(), getpgrp()).unwrap();
                    }
                    Ok(WaitStatus::Stopped(_, _)) => {
                        jobs.push(Job {
                            pgid: child,
                            state: JobState::Suspended,
                            command: name,
                        });
                        match tcsetpgrp(std::io::stdin(), getpgrp()) {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("tcsetpgrp failed: {}", e);
                                return;
                            }
                        };
                    }
                    Ok(WaitStatus::Signaled(_, _, _)) => {}
                    Err(e) => eprintln!("{}", e),
                    _ => {}
                }
            }
        }
        Err(e) => {
            eprintln!("Error : {}", e);
        }
    }
}

pub fn exec_pipe(commands: &parser::Commands, jobs: &mut Vec<Job>) {
    //Executes and handels pipes
    let mut pipes: Vec<(OwnedFd, OwnedFd)> = Vec::new();
    let mut pgid: Option<Pid> = None; //Leader Pid
    let mut pids = vec![];
    let args = &commands.command;
    for _i in 0..(args.len() - 1) {
        let (read_fd, write_fd) = pipe().unwrap();
        pipes.push((read_fd, write_fd));
    }
    let name: String = parser::construct_string(&commands.command);
    for i in 0..(args.len()) {
        match unsafe { fork() } {
            Ok(ForkResult::Child) => {
                reset_signals();
                if i == 0 {
                    //Leader process (The first process which sets the Pgid)
                    setpgid(Pid::from_raw(0), Pid::from_raw(0)).expect("Failed to set PGID");
                    unsafe {
                        match libc::dup2(pipes[i].1.as_raw_fd(), 1) {
                            -1 => {
                                eprintln!("Failed to initialize fork");
                                libc::_exit(1);
                            }
                            _ => {}
                        };
                    };
                    if args[i].stdin.is_some() {
                        // Read input from file
                        let file = OpenOptions::new()
                            .read(true)
                            .open(args[i].stdin.as_ref().unwrap())
                            .expect("Failed to open file");
                        let fd = file.into_raw_fd();
                        unsafe {
                            match libc::dup2(fd, 0) {
                                -1 => {
                                    eprintln!("Failed to read file");
                                    libc::close(fd);
                                    libc::_exit(1);
                                }
                                _ => libc::close(fd),
                            };
                        }
                    }
                } else if i == args.len() - 1 {
                    // Process n-1  (the last process)
                    setpgid(Pid::from_raw(0), pgid.unwrap()).expect("Set pgid failed");
                    // Write output to a file
                    unsafe {
                        match libc::dup2(pipes[i - 1].0.as_raw_fd(), 0) {
                            -1 => {
                                eprintln!("Error reading fd");
                                libc::_exit(1);
                            }
                            _ => {}
                        }
                    };
                    if args[i].stdout.is_some() {
                        let file = match args[i].append {
                            true => OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(args[i].stdout.as_ref().unwrap()),
                            false => OpenOptions::new()
                                .create(true)
                                .write(true)
                                .truncate(true)
                                .open(args[i].stdout.as_ref().unwrap()),
                        };
                        let fd = file.unwrap().into_raw_fd();
                        unsafe {
                            match libc::dup2(fd, 1) {
                                -1 => {
                                    eprintln!("Failed to create output File");
                                    libc::close(fd);
                                    libc::_exit(1);
                                }
                                _ => {
                                    libc::close(fd); // explicitly close the file fd , on both successful and error cases to prevent leaks
                                }
                            };
                        }
                    }
                } else {
                    // Process I (the middle process)
                    setpgid(Pid::from_raw(0), pgid.unwrap()).expect("Set pgid failed");
                    unsafe {
                        match libc::dup2(pipes[i - 1].0.as_raw_fd(), 0) {
                            -1 => {
                                eprintln!("Failed to create output File");
                                libc::_exit(1);
                            }
                            _ => {}
                        }
                    };
                    unsafe {
                        match libc::dup2(pipes[i].1.as_raw_fd(), 1) {
                            -1 => {
                                eprintln!("Failed to create output File");
                                libc::_exit(1);
                            }
                            _ => {}
                        };
                    };
                }
                for (r, w) in &pipes {
                    unsafe {
                        libc::close(r.as_raw_fd());
                        libc::close(w.as_raw_fd());
                    }
                }
                let command = parser::cstring(&args[i].args);
                let comm: Vec<&std::ffi::CStr> = command.iter().map(|c| c.as_c_str()).collect();
                match execvp(comm[0], &comm) {
                    Ok(_) => unreachable!(), //Successful execution , code exits
                    Err(e) => eprintln!("{}", e),
                }
                unsafe { libc::_exit(1) };
            }
            Ok(ForkResult::Parent { child }) => {
                if i == 0 {
                    //Parent sets Pgid's too to avoid race conditions
                    pgid = Some(child);
                    setpgid(child, child).expect("Failed to set pgid");
                } else {
                    setpgid(child, pgid.unwrap()).expect("Failed to set pgid");
                }
                pids.push(child);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
    for p in pipes {
        unsafe {
            libc::close(p.0.into_raw_fd());
            libc::close(p.1.into_raw_fd());
        }
    }
    if commands.bg {
        jobs.push(Job {
            pgid: pgid.unwrap(),
            state: JobState::Running,
            command: name,
        });
    } else {
        tcsetpgrp(std::io::stdin(), pgid.unwrap()).unwrap();
        loop {
            match waitpid(
                Pid::from_raw(-pgid.unwrap().as_raw()),
                Some(WaitPidFlag::WUNTRACED),
            ) {
                Ok(WaitStatus::Exited(_, _)) => {
                    tcsetpgrp(std::io::stdin(), getpgrp()).unwrap();
                }
                Ok(WaitStatus::Signaled(_, _, _)) => {
                    continue;
                }
                Ok(WaitStatus::Stopped(_, _)) => {
                    jobs.push(Job {
                        pgid: pgid.unwrap(),
                        state: JobState::Suspended,
                        command: name.clone(),
                    });
                    if let Err(e) = tcsetpgrp(std::io::stdin(), getpgrp()) {
                        eprintln!("tcsetpgrp failed: {}", e);
                        return;
                    }
                    break;
                }
                Err(nix::errno::Errno::ECHILD) => break,
                Err(e) => {
                    eprintln!("{}", e);
                    break;
                }
                _ => {}
            }
        }
    }
}
