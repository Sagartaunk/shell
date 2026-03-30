use crate::parser;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
pub fn cd(pat: &parser::Command) {
    let path: &[String] = &pat.args;
    if path.len() < 2 {
        match std::env::set_current_dir(std::env::var("HOME").unwrap()) {
            Ok(_) => return,
            Err(e) => {
                println!("{}", e)
            }
        }
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
    } else if input.stdin.is_some() {
        println!("{}", input.stdin.as_ref().unwrap());
        return;
    }
    println!("{}", comm);
}
pub fn pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("{}", e),
    }
}
