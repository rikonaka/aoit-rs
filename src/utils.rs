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
    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

pub fn read_file_str(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn create_dir(dirname: &str) -> Result<()> {
    fs::create_dir(dirname)?;
    Ok(())
}

pub fn create_file(filename: &str) -> Result<File> {
    let f = File::create(filename)?;
    Ok(f)
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
    let serialized = serde_json::to_string(&serde_config)?;
    write_to_file(filename, &serialized)
}

pub fn serde_from_file(filename: &str) -> Result<SerdeConfig> {
    let contents = read_file_str(filename)?;
    let s = serde_json::from_str(&contents)?;
    Ok(s)
}
