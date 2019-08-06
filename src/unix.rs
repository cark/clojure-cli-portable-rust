use std::env;
use std::env::VarError;
use std::path::PathBuf;

pub fn get_args() -> Vec<String> {
    env::args().collect()
}

pub fn get_default_config_dir() -> Result<String, VarError> {
    env::var("XDG_CONFIG_HOME")
        .or_else(|_| env::var("HOME"))
        .map(| value |
             PathBuf::from(value).join(".clojure")
             .to_str().expect("Couldn't convert path")
             .to_owned())
}
