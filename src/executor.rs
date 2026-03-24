use crate::parser;
use libc::c_char;
use libc::{execvp, fork, waitpid};
use std::cmp::Ordering;
pub fn exec(args: &[String]) {
    unsafe {
        let pid = fork();
        match pid.cmp(&0) {
            Ordering::Equal => {
                let arg = parser::cstring(args);
                let mut args: Vec<*const c_char> = arg.iter().map(|s| s.as_ptr()).collect();
                args.push(std::ptr::null());
                execvp(args[0], args.as_ptr());
                eprintln!("Nothing ......");
                std::process::exit(1);
            }
            Ordering::Less => {
                println!("Failed to execute")
            }
            Ordering::Greater => {
                let mut status = 0;
                waitpid(pid, &mut status, 0);
            }
        }
    }
}
