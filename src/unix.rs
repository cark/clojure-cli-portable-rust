use std::env;
use std::env::VarError;
use std::path::{PathBuf, Path};
extern crate exec;

pub fn get_args() -> Vec<String> {
    env::args().collect()
}

pub fn get_default_config_dir() -> Result<String, VarError> {
    env::var("XDG_CONFIG_HOME")
        .or_else(|_| env::var("HOME"))
        .map(|value|
             PathBuf::from(value).join(".clojure")
             .to_str().expect("Couldn't convert path")
             .to_owned())
}

pub fn get_user_cache_dir(config_dir: &str) -> Result<String,  VarError> {
    env::var("XDG_CONFIG_HOME")
        .or(Ok(config_dir.to_string()))
        .map(|value|
             PathBuf::from(value).join(".cpcache")
             .to_str().expect("Couldn't convert path")
             .to_owned())
}

pub fn exec(command: &Path, args: Vec<&str>) -> () {
    let err = exec::Command::new(command)
        .args(&args)
        .exec();
    println!("Error executing '{:?}' : '{}'", command, err);
}

pub const PATH_LIST_SEPARATOR : &str = ":";
