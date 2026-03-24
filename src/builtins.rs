use std::env;
pub fn cd(path: &[String]) {
    if path.len() < 2 {
        let _ = std::env::set_current_dir(std::env::var("HOME").unwrap());
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
pub fn echo(input: &[String]) -> String {
    input[1..].join(" ")
}
