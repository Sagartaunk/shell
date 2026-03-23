use std::env;
pub fn cd(path: Vec<String>) {
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
