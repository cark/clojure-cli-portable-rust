
pub fn to_clj_string(source: &str) -> String {
    let mut result = "\"".to_string();
    let mut it = source.chars();
    while let Some(c) = it.next() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            c => result.push(c),
        }            
    }
    result.push('\"');
    result
}
