use crate::parser;
use nix::sys::wait::waitpid;
use nix::unistd::{execvp, fork, ForkResult};
pub fn exec(args: &[String]) {
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            let arg = parser::cstring(args);
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
