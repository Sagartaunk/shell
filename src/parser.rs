use std::ffi::CString;
pub fn cstring(input: &[String]) -> Vec<CString> {
    input
        .iter()
        .map(|s| CString::new(s.as_str()).unwrap())
        .collect()
}
#[derive(Debug, Clone)]
pub struct Command {
    pub args: Vec<String>,
    pub stdin: Option<String>,
    pub stdout: Option<String>,
    pub append: bool,
}
#[derive(Debug)]
pub struct Commands {
    //Wrapper for Command structure crate::executor;
    pub command: Vec<Command>,
    pub bg: bool,
}

pub fn parse(input: &str) -> Commands {
    let tokens: Vec<&str> = input
        .split_whitespace()
        .map(|s| s.trim_matches('"'))
        .collect();
    let mut commands = Vec::new();
    let mut comm = Commands {
        command: Vec::new(),
        bg: false,
    };
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
            "&" => {
                comm.bg = true;
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
    comm.command = commands;
    comm
}
pub fn construct_string(args: &[Command]) -> String {
    args.iter()
        .map(|c| c.args.join(" "))
        .collect::<Vec<_>>()
        .join(" | ")
}
