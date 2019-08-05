mod parsing; 
use widestring::U16CString;
use winapi::um::processenv;

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
