use std::env;

pub fn get_args() -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for (i, argument) in env::args().enumerate() {
        if i != 0 {
            result.push(argument);
        }
    };
    result
}
