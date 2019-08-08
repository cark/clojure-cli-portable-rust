use std::fs;
use std::process::exit;

fn main() {
    let version = fs::read_to_string("version.txt");
    match version {
        Ok(v) => println!("cargo:rustc-env=PROJECT_VERSION={}", v.trim()),
        Err(_) => exit(1),
    }
}
