use sha2::{Digest, Sha256};
use std::fs;
use std::fs::{read, File};
use std::io::{Read, Write};
use std::process::Command;

use crate::SerdeConfig;

pub fn move_file_to_dir(filename: &str, target_dir: &str) {
    let _ = Command::new("mv")
        .arg(filename)
        .arg(target_dir)
        .output()
        .expect("Failed to execute apt download");
}

pub fn remove_dir(target_dir: &str) {
    let _ = Command::new("rm")
        .arg("-rf")
        .arg(target_dir)
        .output()
        .expect("Failed to execute apt download");
}

pub fn read_file_bytes(path: &str) -> Option<Vec<u8>> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to read aoit file: {}", e);
            return None;
        }
    };
    let mut contents = Vec::new();
    match file.read_to_end(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to read aoit as bytes: {}", e);
            return None;
        }
    }
    Some(contents)
}

pub fn read_file_str(path: &str) -> Option<String> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            println!("Failed to read aoit file: {}", e);
            return None;
        }
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to read aoit as bytes: {}", e);
            return None;
        }
    }
    Some(contents)
}

pub fn create_dir(dirname: &str) -> bool {
    match fs::create_dir(dirname) {
        Ok(_) => true,
        Err(e) => {
            println!("Create target dir failed: {}", e);
            false
        }
    }
}

pub fn create_file(filename: &str) -> Option<File> {
    match File::create(filename) {
        Ok(f) => Some(f),
        Err(e) => {
            println!("Failed create sha256 file: {}", e);
            None
        }
    }
}

pub fn file_sha256(filename: &str) -> Option<String> {
    let contents = read(filename).unwrap();
    let hash = Sha256::digest(&contents);
    // println!("{:x}", hash);
    let hash_str = format!("{:x}", hash);
    Some(hash_str)
}

pub fn write_to_file(filename: &str, contents: &str) -> bool {
    let mut file = create_file(filename).unwrap();
    match file.write_all(contents.as_bytes()) {
        Ok(_) => true,
        Err(e) => {
            println!("Failed to write config file: {}", e);
            false
        }
    }
}

pub fn serde_to_file(filename: &str, serde_config: SerdeConfig) -> bool {
    let serialized = match serde_json::to_string(&serde_config) {
        Ok(s) => s,
        _ => return false,
    };
    write_to_file(filename, &serialized)
}

pub fn serde_from_file(filename: &str) -> Option<SerdeConfig> {
    let contents = read_file_str(filename).unwrap();
    match serde_json::from_str(&contents) {
        Ok(s) => Some(s),
        Err(e) => {
            println!("serde from file {} failed: {}", filename, e);
            None
        }
    }
}
