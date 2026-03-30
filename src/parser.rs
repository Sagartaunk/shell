use std::ffi::CString;
pub fn cstring(input: &[String]) -> Vec<CString> {
    input
        .iter()
        .map(|s| CString::new(s.as_str()).unwrap())
        .collect()
}
#[derive(Debug)]
pub struct Command {
    args: Vec<String>,
    stdin: Option<String>,
    stdout: Option<String>,
    append: bool,
}
pub fn parse(input: &str) -> Vec<Command> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut commands = Vec::new();
    let mut current = Command {
        args: Vec::new(),
        stdin: None,
        stdout: None,
        append: false,
    };
    let mut i = 0;
    while i < tokens.len() {
        match tokens[i] {
            "|" => {
                commands.push(current);
                current = Command {
                    args: Vec::new(),
                    stdin: None,
                    stdout: None,
                    append: false,
                };
            }
            "<" => {
                i += 1;
                if i < tokens.len() {
                    current.stdin = Some(tokens[i].to_string());
                }
            }
            ">" => {
                i += 1;
                if i < tokens.len() {
                    current.stdout = Some(tokens[i].to_string());
                    current.append = false;
                }
            }
            ">>" => {
                i += 1;
                if i < tokens.len() {
                    current.stdout = Some(tokens[i].to_string());
                    current.append = true;
                }
            }
            _ => {
                current.args.push(tokens[i].to_string());
            }
        }
        i += 1;
    }
    if !current.args.is_empty() {
        commands.push(current);
    }
    commands
}
