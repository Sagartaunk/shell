use crate::parser;
use nix::sys::wait::waitpid;
use nix::unistd::{ForkResult, execvp, fork, pipe};
use std::fs::OpenOptions;
use std::os::fd::{AsRawFd, IntoRawFd};
use std::os::unix::io::OwnedFd;
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
            waitpid(child, None).unwrap(); //Wait and free all the child process to prevent them from becoming zombie
        }
        Err(e) => {
            eprintln!("Error : {}", e);
        }
    }
}

pub fn exec_pipe(args: &[parser::Command]) {
    let mut pipes: Vec<(OwnedFd, OwnedFd)> = Vec::new();
    let mut pids = vec![];
    for _i in 0..(args.len() - 1) {
        let (read_fd, write_fd) = pipe().unwrap();
        pipes.push((read_fd, write_fd));
    }
    for i in 0..(args.len()) {
        match unsafe { fork() } {
            Ok(ForkResult::Child) => {
                if i == 0 {
                    unsafe {
                        match libc::dup2(pipes[i].1.as_raw_fd(), 1) {
                            -1 => {
                                eprintln!("Failed to open File discriptor");
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
                for p in &pipes {
                    //Close all pipe ends before continuing to prevent EOF stalls
                    unsafe {
                        libc::close(p.0.as_raw_fd());
                        libc::close(p.1.as_raw_fd());
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
                pids.push(child);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
    for pid in pids.iter() {
        waitpid(*pid, None).unwrap();
    }
}
