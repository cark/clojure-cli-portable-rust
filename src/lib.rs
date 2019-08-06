#[cfg(windows)]
mod windows;
#[cfg(unix)]
mod unix;
#[cfg(windows)]
use windows as compat;
#[cfg(unix)] 
use unix as compat;

extern crate which;
extern crate md5;

mod help;

use std::process::exit;
use std::collections::HashSet;
use std::env;
use std::io;
use std::process::{Command};
use std::path::{Path, PathBuf};
use std::fs;

const PROJECT_VERSION : &str = "1.10.1.466"; 

#[derive(Hash, Eq, PartialEq, Debug)]
enum Flag {
    PrintClasspath, Describe, Verbose, Force, Repro, Tree, Pom, ResolveTags, CpJar, Help
}

fn exe_dir() -> io::Result<PathBuf> {
    let exe = env::current_exe()?; 
    let dir = exe.parent().expect("Executable must be in some directory.");
    Ok(PathBuf::from(dir))
}

fn insert(s : &mut HashSet<Flag>, val : Flag) -> () {
    s.insert(val); 
}

fn path_to_str(p: &Path) -> String {
    p.to_str().expect("Error building path.").to_owned()
}

pub fn main() -> () {
    let install_dir = path_to_str(&exe_dir().expect("Couldn't find executable directory."));
    
    let tools_cp = PathBuf::from(&install_dir)
        .join("libexec")
        .join(format!("clojure-tools-{}.jar", &PROJECT_VERSION))
        .to_str().expect("Couldn't produce a tools_cp string.")
        .to_owned();    

    let config_dir : String;
    let user_cache_dir : String;
    let config_paths : Vec<String>;
    let config_str : String;
    let cache_dir : String;
    
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
    let java_command : PathBuf;

    // Parse arguments
    let args = compat::get_args();
    let mut arg_iter = args.iter().skip(1);
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
            "-Sresolve-tags" => insert(&mut flags, Flag::ResolveTags),
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

    // Find java
    match which::which("java") {
        Ok(s) => (java_command = s),
        Err(_) =>  match env::var("JAVA_HOME") {
            Ok(s) => match which::which_in("java", PathBuf::from(s).join("bin").to_str(), "") {
                Ok(s) => (java_command = s),
                Err(_) => {
                    println!("Couldn't find 'java'.");
                    exit(1);
                }
            }
            Err(_) => {
                println!("Couldn't find 'java'. Please set JAVA_HOME.");
                exit(1);
            }            
        }
    }

    // Show help
    if flags.contains(&Flag::Help) {
        help::print();
        exit(0);
    }

    // launch process child
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

    // Execute resolve tags command
    if flags.contains(&Flag::ResolveTags) {
        if Path::new("deps.edn").exists() {
            exit(launch(&java_command,
                        vec!["-Xms256m", "-classpath", &tools_cp, "clojure.main", "-m",
                             "clojure.tools.deps.alpha.script.resolve-tags", "--deps-file=deps.edn"]));
        } else {
            println!("deps.edn does not exist.");
            exit(1);
        }
    }

    // Determine user config directory
    config_dir = env::var("CLJ_CONFIG")
        .or_else(|_| compat::get_default_config_dir())
        .expect("Couldn't determine user config directory.")
        .to_string();

    // Ensure user config directory exists
    if ! Path::new(&config_dir).exists() {
        fs::create_dir_all(&config_dir)
            .expect(&format!("Couldn't create deps.edn in '{}'", &config_dir)); 
    }

    // Ensure user level deps.edn exists
    if ! Path::new(&config_dir).join("deps.edn").exists() {
        fs::copy(Path::new(&install_dir).join("example-deps.edn"),
                 Path::new(&config_dir).join("deps.edn"))
            .expect(&format!("Couldn't create deps.edn in '{}'", &config_dir));
    }

    // Determine user cache directory
    user_cache_dir = env::var("CLJ_CACHE") 
        .or_else(|_| compat::get_user_cache_dir(&config_dir))
        .expect("Couldn't determine user cache directory.")
        .to_string();

    // Chain deps.edn in config paths. repro=skip config dir
    if flags.contains(&Flag::Repro) {
        config_paths = vec![path_to_str(&PathBuf::from(&install_dir).join("deps.edn")),
                            "deps.edn".to_string()];         
    } else {
        config_paths = vec![path_to_str(&PathBuf::from(&install_dir).join("deps.edn")),
                            path_to_str(&PathBuf::from(&config_dir).join("deps.edn")),
                            "deps.edn".to_string()];
    }
    config_str = config_paths.iter()
        .map(|s| format!("\"{}\"", s))
        .collect::<Vec<String>>()
        .join("'");

    // Determine wether to use user or project cache
    if Path::new("deps.edn").exists() {
        cache_dir = ".cpcache".to_string()
    } else {
        cache_dir = user_cache_dir
    }

    // Construct location of cached classpath file
    // *** added project_version to the cache_key, so we rebuild cache on version change
    let dd = if let Some(s) = deps_data {
        s
    } else {
        "".to_string()
    };
    let mut cache_key = [&resolve_aliases.join(""),
                         &classpath_aliases.join(""),
                         &all_aliases.join(""),
                         &jvm_aliases.join(""),
                         &main_aliases.join(""),
                         &dd, PROJECT_VERSION].join(""); 
    for config_path in &config_paths {
        if PathBuf::from(&config_path).exists() {
            cache_key.push_str(&config_path)
        } else {
            cache_key.push_str("NIL");
        }
    }

    // Opting for md5
    let cache_key_hash : String = format!("{:x}", md5::compute(cache_key)).chars().take(8).collect();
    let base_file = path_to_str(&PathBuf::from(&cache_dir).join(&cache_key_hash));
    let libs_file = format!("{}.libs", &base_file);
    let cp_file = format!("{}.cp", &base_file);
    let jvm_file = format!("{}.jvm", &base_file);
    let main_file = format!("{}.main", &base_file);

    // Print paths in verbose mode
    if flags.contains(&Flag::Verbose) {
        println!("version      = {}", &PROJECT_VERSION);
        println!("install_dir  = {}", &install_dir);
        println!("config_dir   = {}", &config_dir);
        println!("config_paths = {}", &config_paths.join(", "));
        println!("cache_dir    = {}", &cache_dir);
        println!("cp_file      = {}", &cp_file);
        println!("");
    }

    // Check for stale cache
    let mut stale = false;
    if flags.contains(&Flag::Force) || ! PathBuf::from(&cp_file).exists() {
        stale = true
    } else {
        let cp_time = fs::metadata(&cp_file).unwrap().modified().expect("Couldn't get file modification time.");
        for config_path in &config_paths {
            match fs::metadata(&config_path) {
                Ok(md) => if cp_time < md.modified().expect("Couldn't get file modification time.") {
                    stale =  true;
                    break;
                }
                Err(_) => (),
            }
        }
    }
}
