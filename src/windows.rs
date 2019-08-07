mod parsing; 
use widestring::U16CString;
use winapi::um::processenv;
use std::env;
use std::env::VarError;
use std::process::{Command};
use std::path::{Path, PathBuf};
use std::process::exit;

pub fn get_command_line() -> String {
    let u16_str: U16CString;
    unsafe {
        let command_line = processenv::GetCommandLineW();
        u16_str = U16CString::from_ptr_str(command_line)
    }
    u16_str.to_string().unwrap()
}

pub fn get_args() -> Vec<String> {
    parsing::args_vec(&get_command_line())
}

pub fn get_default_config_dir() -> Result<String, VarError> {
    env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map(| value |
             PathBuf::from(value).join(".clojure")
             .to_str().expect("Couldn't convert path")
             .to_owned())
}

pub fn get_user_cache_dir(config_dir: &str) -> Result<String,  VarError> {
    Ok(PathBuf::from(config_dir)
       .join(".cpcache")
       .to_str().expect("Couldn't convert path")
       .to_owned())
}

fn launch(command: &Path, args: Vec<&str>) -> i32 {
    match Command::new(command)
        .args(args)
        .status()
        .expect(&format!("Failed to execute '{:?}'.", command))
        .code()
    {
        None => 128,
        Some(v) => v,
    }
} 

pub fn exec(command: &Path, args: Vec<&str>) -> () {
    exit(launch(command, args));
}
