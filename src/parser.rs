use std::ffi::CString;
pub fn parse(input: &str) -> Vec<String> {
    input.split_whitespace().map(|s| s.to_string()).collect()
}
pub fn cstring(input: &Vec<String>) -> Vec<CString> {
    input
        .iter()
        .map(|s| CString::new(s.as_str()).unwrap())
        .collect()
}
