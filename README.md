## A small Unix shell written in Rust 
This project was started as a way to learn about systems concepts like process creation, pipes, job controls etc.
I have used the nix crate to understand how to use raw syscalls instead of the std::commands wrapper, and libc's version of dup 2 calls because nix's version was kind of quirky as it wanted safe handelling of ownership which has lead to manually closing all fd's.
Moreover, I will be making an attempt to find out and fix any bugs in this code to learn how to write safe, production level code properly, Feel free to use the project for your own learning and point out any bugs i may have missed as this project has not gone through an extensive test process.
 
 
## What this shell supports
->Running external programs (ls, cat, grep, etc.)  
->Pipelines (|)  
->Input redirection (<)  
->Output redirection (>)  
->Append redirection (>>)  
->Background execution (&)  
->Job control (jobs, fg, bg)  
->Built-in commands (cd, pwd, echo, exit)  
->Basic foreground process handling  
->Suspended job tracking (Ctrl+Z)  
->Multiple process pipeline execution  

## Known limitations 
-> Proper Quotes handeling
-> Too much unwrap use which can lead to panics 

## Todo :
-> Do proper testing and bug fixes  
-> Remove unwrap calls wherever possible with proper error handeling  
-> Add proper quotes handeling to the parser (Maybe)  

## Steps To use the shell ->
#### First clone the shell with 
git clone https://github.com/Sagartaunk/shell.git
#### Then cd into the shell directory and run the project with
cargo build --release
./target/release/shell
