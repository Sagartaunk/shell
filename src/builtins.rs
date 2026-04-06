use crate::executor::Job;
use crate::parser;
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
