use anyhow::Result;
use sha2::Digest;
use sha2::Sha256;
use std::fs;
use std::fs::read;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::process::Command;

use crate::SerdeConfig;

pub fn move_file_to_dir(target_dir: &str, filename: &str) -> Result<()> {
    let _ = Command::new("mv").arg(filename).arg(target_dir).output()?;
    Ok(())
}

pub fn remove_dir(target_dir: &str) -> Result<()> {
    let _ = Command::new("rm").arg("-rf").arg(target_dir).output()?;
    Ok(())
}

pub fn read_file_bytes(path: &str) -> Result<Vec<u8>> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e.into()),
    };
    let mut contents = Vec::new();
    match file.read_to_end(&mut contents) {
        Ok(_) => (),
        Err(e) => return Err(e.into()),
    }
    Ok(contents)
}

pub fn read_file_str(path: &str) -> Result<String> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e.into()),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(e) => return Err(e.into()),
    }
    Ok(contents)
}

pub fn create_dir(dirname: &str) -> Result<()> {
    match fs::create_dir(dirname) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

pub fn create_file(filename: &str) -> Result<File> {
    match File::create(filename) {
        Ok(f) => Ok(f),
        Err(e) => Err(e.into()),
    }
}

pub fn file_sha256(filename: &str) -> Result<String> {
    let contents = read(filename)?;
    let hash = Sha256::digest(&contents);
    // println!("{:x}", hash);
    let hash_str = format!("{:x}", hash);
    Ok(hash_str)
}

pub fn write_to_file(filename: &str, contents: &str) -> Result<()> {
    let mut file = create_file(filename)?;
    match file.write_all(contents.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

pub fn serde_config_to_file(filename: &str, serde_config: SerdeConfig) -> Result<()> {
    let serialized = match serde_json::to_string(&serde_config) {
        Ok(s) => s,
        Err(e) => return Err(e.into()),
    };
    write_to_file(filename, &serialized)
}

pub fn serde_from_file(filename: &str) -> Result<SerdeConfig> {
    let contents = read_file_str(filename)?;
    match serde_json::from_str(&contents) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.into()),
    }
}
