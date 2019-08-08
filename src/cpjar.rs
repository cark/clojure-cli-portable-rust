use super::{compat, path_to_str};
use std::fs;
use zip;
use std::process::exit;
use std::path::PathBuf;
use std::io::Write;

const MAX_BYTE_COUNT: usize = 72;
const MAX_CP_LENGTH: usize = 30000;

// builds a manifest attribute, limiting line length to MAX_BYTE_COUNT
fn to_attr(name: &str, value: &str) -> String {
    let attr_string = format!("{}: {}", name, value);
    let mut result = String::from("");
    let mut line_pos : usize = 0;
    let mut chars = attr_string.chars();
    while let Some(c) = chars.next() {
        let size = c.len_utf8();
        if line_pos + size >= MAX_BYTE_COUNT {
            result.push_str("\n ");
            line_pos = 1;
        }
        result.push(c);
        line_pos += size;
    }
    result.push('\n');
    result
}

fn build_manifest(cp: &str) -> String {
    let mut it = cp.split(compat::PATH_LIST_SEPARATOR);
    let mut paths : Vec<String> = vec![];
    while let Some(path) = it.next() {
        let path = PathBuf::from(path);
        let mut p = path_to_str(&path);
        if path.is_dir() {
            if let Some(c) = p.chars().nth(1) {
                if c == ':' {
                    p = format!("\\{}", path_to_str(&path));
                }
            }
            p.push(std::path::MAIN_SEPARATOR);
        } else if path.is_file() {
            if let Some(c) = p.chars().nth(1) {
                if c == ':' {
                    p = format!("\\{}", path_to_str(&path));
                }
            }            
        } else {
            println!("'{}' is neither a file or a directory.", p);
            exit(1);
        }
        paths.push(p.replace(" ", "%20"));
    }
    let cp = to_attr("Class-Path", &paths.join(" "));
    format!("Manifest-Version: 1.0\n{}Created-By: Clojure \n\n", cp)
}

fn write_cp_jar(filename: &str, cp: &str) -> zip::result::ZipResult<()> {
    let manifest = build_manifest(cp);
    let w = std::fs::File::create(filename).expect("Couldn't create jar file.");
    let mut zip = zip::ZipWriter::new(w);
    let options = zip::write::FileOptions::default();
    zip.start_file("META-INF/MANIFEST.MF", options)?;
    zip.write(&manifest.as_bytes())?;
    zip.finish()?;
    Ok(())
}

pub fn file_to_cp(filename: &str, forced: bool) -> String {
    let cp = fs::read_to_string(filename).expect("Couldn't read classpath cache file.");
    if forced || (cfg![windows] && (cp.len() > MAX_CP_LENGTH)) {
        let jar_name = PathBuf::from(filename).with_extension("jar");
        let create = match jar_name.metadata() {
            Err(_) => true,
            Ok(md) => md.modified().unwrap() < PathBuf::from(filename).metadata().unwrap().modified().unwrap(),
        };
        if create {
            write_cp_jar(&path_to_str(&jar_name), &cp).expect("Error creating classpath jar.");
        }
        path_to_str(&jar_name)
    } else {
        cp.to_string()
    }
}
