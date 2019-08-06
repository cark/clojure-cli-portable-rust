#[cfg(windows)]
mod windows;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
use windows as compat;
#[cfg(unix)] 
use unix as compat;

mod help;

use std::process::exit;
use std::collections::HashSet;
use std::env;
use std::io;
use std::path::PathBuf;
use which::which;

#[derive(Hash, Eq, PartialEq, Debug)]
enum Flag {
    PrintClasspath, Describe, Verbose, Force, Repro, Tree, Pom, ResolveTags, CpJar, Help
}

fn current_dir() -> io::Result<PathBuf> {
    let exe = env::current_exe()?; 
    let dir = exe.parent().expect("Executable must be in some directory.");
    Ok(PathBuf::from(dir))
}

fn insert(s : &mut HashSet<Flag>, val : Flag) -> () {
    s.insert(val); 
}

pub fn main() -> () {
    let install_dir = current_dir().expect("Couldn't find executable directory.");
    let mut resolve_aliases : Vec<String> = vec![];
    let mut classpath_aliases : Vec<String> = vec![];
    let mut jvm_aliases : Vec<String> = vec![];
    let mut main_aliases : Vec<String> = vec![];
    let mut all_aliases : Vec<String> = vec![];
    let mut extra_args : Vec<String> = vec![];
    let mut jvm_options : Vec<String> = vec![];
    let mut flags: HashSet<Flag> = HashSet::new();
    let mut force_cp : Option<String> = None;
    let mut deps_data : Option<String> = None;    
    let args = compat::get_args();
    let mut arg_iter = args.iter().skip(1);
    let java_command : PathBuf;
    
    while let Some(arg) = arg_iter.next() {
        match arg.as_ref() { 
            "-h" | "--help" | "-?" => 
                if ! main_aliases.is_empty() || ! all_aliases.is_empty() {
                    extra_args.push(arg.to_string());
                    extra_args.extend(arg_iter.map(|str| str.to_string()));
                    break;
                } else {
                    insert(&mut flags, Flag::Help); 
                },
            "-Sdeps" => 
                match arg_iter.next() {
                    None => {
                        println!("-Sdeps requires an additional parameter !");
                        exit(1);
                    },
                    Some(s) => deps_data = Some(s.to_string()),
                },
            "-Scp" =>
                match arg_iter.next() {
                    None => {
                        println!("-Scp requires an additional parameter !");
                        exit(1);
                    },
                    Some(s) => force_cp = Some(s.to_string()),
                },
            "-Spath" => insert(&mut flags, Flag::PrintClasspath),
            "-Sverbose" => insert(&mut flags, Flag::Verbose),
            "-Sdescribe" => insert(&mut flags, Flag::Describe),
            "-Sforce" => insert(&mut flags, Flag::Force),
            "-Srepro" => insert(&mut flags, Flag::Repro),
            "-Stree" => insert(&mut flags, Flag::Tree),
            "-Spom" => insert(&mut flags, Flag::Pom),
            "-Srecolve-tags" => insert(&mut flags, Flag::ResolveTags),
            "-Scp-jar" => insert(&mut flags, Flag::CpJar),
            arg => match arg.chars().take(2).collect::<String>().as_ref() {
                "-J" => jvm_options.push(arg.chars().skip(2).collect()),
                "-R" => resolve_aliases.push(arg.chars().skip(2).collect()),
                "-C" => classpath_aliases.push(arg.chars().skip(2).collect()),
                "-O" => jvm_aliases.push(arg.chars().skip(2).collect()),
                "-M" => main_aliases.push(arg.chars().skip(2).collect()),
                "-A" => all_aliases.push(arg.chars().skip(2).collect()),
                "-S" => {
                    println!("Invalid option: {}", arg);
                    exit(1);
                }
                _ => {
                    extra_args.push(arg.to_string());
                    extra_args.extend(arg_iter.map(|str| str.to_string()));
                    break;                    
                }
            }
        }
    }

    match which::which("java") {
        Ok(s) => (java_command = s),
        Err(_) => {
            match env::var("JAVA_HOME") {
                Ok(s) => {
                    let mut p = PathBuf::from(s);
                    p.push("bin");
                    match which::which_in("java", p.to_str(), "") {
                        Ok(s) => (java_command = s),
                        Err(_) => {
                            println!("Couldn't find 'java'.");
                            exit(1);
                        }
                    }
                }
                Err(_) => {
                    println!("Couldn't find 'java'. Please set JAVA_HOME.");
                    exit(1);
                }
            }
        }
    }

    if flags.contains(&Flag::Help) {
        help::print();
        exit(0);
    }
}
