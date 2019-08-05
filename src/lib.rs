#[cfg(windows)]
mod windows;
#[cfg(unix)]
mod unix;

#[cfg(windows)]
use windows as compat;
#[cfg(unix)]
use unix as compat;


pub fn yoh() -> () {
    let args = compat::get_args();
    println!("{:?}", args)
}
