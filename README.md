## A small Unix shell written in Rust 
This project was started as a way to learn about systems concepts like process creation, pipes, job controls etc.  
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


# Architecture  
## The shell is divided into 5 major modules:  
-> main.rs - Intiializes the shell  
-> repl.rs - Interactive loop  
-> parser.rs - Command parsing  
-> builtins.rs - built-in commands  
-> executor.rs - process execution and job control   

### These modules interact in the following order 
user input -> repl -> parser -> builtin | executor -> job control -> repl 

## What each module does 
### parser.rs -> 
The parser is responsible for understanding user input, It converts raw text into structured data the executor can use. 

Capabilities -> 
Split Tokens  
Detect pipelines 
detect redirections   
detect background 

### executor.rs ->
The executor is responsible for running programs. 

Capabilities -> 
Handels pipelines , redirections , foreground/background, process groups , job tracking 

### builtins.rs ->
builtins include :  
cd  
pwd  
echo  
jobs  
fg  
bg  

Builtins do not fork because some of them must modify shell state


### repl.rs ->
Repl coordinates everything essentially it is the brain of the shell.


# Some interesting decisions i made along the way 
#### Why _exit() instead of exit() in child processes ?
After a fork happens, the parent and the child share stdio buffers , memory and atexit handelers. Thus, if the child calls exit() it causes subtle bugs, whereas _exit() immediately terminates the process and hence is considered safe after fork.

#### Why both parent and child call setpgid ?
It is to avoid race conditions as if the child has already executed when the parent tried stepgid() , the kerned will return EPERM or ESRCH. Hence, by making both of them call setpgid either the parent or the child will succeed and the other will fail harmlessly. Thus, providing us the gurantee that the process group is set correctly 

#### WUNTRACED (FOREGROUND REAPING) ->
This flag tells waitpid to return when either of the following conditions have been satisfied 
-> process exits 
-> process killed 
-> process stopped via ctrl + z
without Wuntraced the shell would never detect suspensions.

#### WNOHANG (BACKGROUND REAPING) ->
This is used when we dont want to shell to block for the job to finish and just check if the job is finished or not.
WNOHANG makes waitpid return immidiately with one of the following 2 states :
StillAlive -> job is currently running 
Exited 

without this , the shell would block waiting for the background jobs


#### Signal Inheritance problem
While building the shell i found out that the child also inherits shell's signal handellers which makes it ignore the signals given to it. 
The fix:
the reset_signals() function , it sets the signal handelling for the child processes to their default 

#### Why libc::dup2 instead of nix's dup 2 ?
libc's dup2 gives finer control over when fd's are closed relative to dup2 calls as well as i found the nix's implementation to be kind of quirky .
