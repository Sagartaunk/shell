mod builtins;
mod executor;
mod parser;
mod repl;
use nix::unistd::{getpid, setpgid, tcsetpgrp};
fn main() {
    let shell_pid = getpid();
    setpgid(shell_pid, shell_pid).expect("Failed to start Shell");
    tcsetpgrp(std::io::stdin(), shell_pid).expect("Failed to set process group for shell");
    repl::run();
}
