mod parsing; 
use widestring::U16CString;
use winapi::um::processenv;
use std::path::PathBuf;
use std::env;
use std::env::VarError;

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
