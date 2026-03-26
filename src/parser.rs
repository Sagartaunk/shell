use std::ffi::CString;
pub fn cstring(input: &[String]) -> Vec<CString> {
    input
        .iter()
        .map(|s| CString::new(s.as_str()).unwrap())
        .collect()
}
pub fn parse_pipeline(input: &str) -> Vec<Vec<String>> {
    let initial: Vec<String> = input.split("|").map(|s| s.to_string()).collect();
    let mut res: Vec<Vec<String>> = Vec::new();
    for i in initial.iter() {
        res.push(i.split_whitespace().map(|s| s.to_string()).collect());
    }
    res.retain(|r| !r.is_empty());
    res
}
