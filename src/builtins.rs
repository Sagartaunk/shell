use std::env;
pub fn cd(path: &[String]) {
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
pub fn echo(input: &[String]) -> String {
    input[1..].join(" ")
}
pub fn pwd() {
    match env::current_dir() {
        Ok(path) => println!("{}", path.display()),
        Err(e) => println!("{}", e),
    }
}
