use anyhow::Result;
use glob::glob;
use log::debug;
use log::error;
use log::info;
use log::warn;
use serde::Deserialize;
use serde::Serialize;
use sevenz_rust;
use std::collections::HashMap;
use std::process::Command;

use crate::utils;
use crate::SerdeConfig;
use crate::DEFAULT_CONFIG_NAME;
use crate::DEFAULT_PACKAGE_SUFFIX;
use crate::DEFAULT_SHA256_SUFFIX;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AptDepends {
    pub name: String,
    pub depends: Vec<AptDepends>,
}

impl AptDepends {
    pub fn new(name: &str, depends: &[AptDepends]) -> AptDepends {
        AptDepends {
            name: name.to_string(),
            depends: depends.to_vec(),
        }
    }
}

fn apt_cache_depends_parser(package_name: &str) -> Result<Vec<AptDepends>> {
    debug!("apt cache depends parser: {package_name}");
    println!("apt cache depends parser: {package_name}");
    let command = Command::new("apt-cache")
        .arg("depends")
        .arg(package_name)
        .output()?;

    let mut ret = Vec::new();
    let command_output = String::from_utf8_lossy(&command.stdout);
    for line in command_output.lines() {
        if line.contains("Depends:") && !line.contains("|") {
            let line_split: Vec<&str> = line
                .split(":")
                .map(|x| x.trim())
                .filter(|x| x.len() > 0)
                .collect();

            if line_split.len() == 2 {
                let depends_package_name = line_split[1];
                let depends = apt_cache_depends_parser(depends_package_name)?;
                let apt_depends = AptDepends::new(depends_package_name, &depends);
                ret.push(apt_depends);
            }
        }
    }

    Ok(ret)
}

fn resolve_depends_new(package_name: &str) -> Result<AptDepends> {
    println!("apt cache depends parser: {package_name}");
    let root_depends = apt_cache_depends_parser(package_name)?;
    let root = AptDepends::new(&package_name, &root_depends);
    Ok(root)
}

fn resolve_depends(package_name: &str) -> Option<Vec<String>> {
    let mut depend_vec: Vec<String> = Vec::new();
    let command = Command::new("apt-cache")
        .arg("depends")
        .arg(package_name)
        .output()
        .expect("Failed to execute apt-cache");

    let command_output = String::from_utf8_lossy(&command.stdout);
    // println!("{}", command_output);
    let mut lines = command_output.lines();
    let _ = lines.next(); // jump over first line
    let mut sp_depends_flag = false;
    for l in lines {
        if l.contains("Depends:") {
            if l.contains("<") && l.contains(">") {
                sp_depends_flag = true;
            } else {
                let l_split: Vec<&str> = l.split(": ").collect();
                let depend_name = if l_split.len() == 2 {
                    l_split[1]
                } else {
                    panic!("Depends length error please contact developer");
                };
                let depend_name = depend_name.trim().to_string();
                if !depend_vec.contains(&depend_name) {
                    depend_vec.push(depend_name);
                }
            }
        } else if sp_depends_flag {
            let depend_name = l.trim().to_string();
            if !depend_vec.contains(&depend_name) {
                depend_vec.push(depend_name);
            }
            sp_depends_flag = false;
        }
    }
    Some(depend_vec)
}

fn download_depends(package_name: &str, target_dir: &str) -> Option<String> {
    let _ = Command::new("apt")
        .arg("download")
        .arg(package_name)
        .output()
        .expect("failed to execute apt download");

    let pattern = format!("{}*.deb", package_name);
    for entry in glob(&pattern).expect("failed to read glob pattern") {
        match entry {
            Ok(path) => {
                // println!("{:?}", path.display());
                let package_full_name = path.to_string_lossy().to_string();
                utils::move_file_to_dir(&package_full_name, &target_dir);
                return Some(package_full_name);
            }
            Err(e) => println!("{:?}", e),
        }
    }
    None
}

pub fn pack_deb(package_name: &str) {
    // let target_dir = format!("./{}", package_name);
    let target_dir = package_name;
    match utils::create_dir(&target_dir) {
        true => println!("Create tmp dir success!"),
        false => {
            println!("Create tmp dir failed!");
            return;
        }
    }

    let mut depends_all_vec: Vec<String> = Vec::new();
    let mut index = 0;
    let mut fake_tree: Vec<HashMap<String, String>> = Vec::new();

    let depends_vec = resolve_depends(package_name).unwrap();
    if depends_vec.len() == 0 {
        println!("The [{}] package does not exist", package_name);
        utils::remove_dir(target_dir);
        return;
    }

    let package_full_name = download_depends(package_name, &target_dir).unwrap();
    // println!("{}", package_full_name);
    let mut hm: HashMap<String, String> = HashMap::new();
    hm.insert("name".to_string(), package_name.to_string());
    hm.insert("full_name".to_string(), package_full_name.to_string());
    fake_tree.push(hm);
    depends_all_vec.append(&mut depends_vec.clone());

    while depends_all_vec.len() > index {
        let package_name = depends_all_vec.get(index).unwrap();
        // print this in update mode
        println!("Resolving depends: {}", package_name);
        let depends_vec = resolve_depends(package_name).unwrap();
        let package_full_name = download_depends(package_name, &target_dir).unwrap();

        let mut hm: HashMap<String, String> = HashMap::new();
        hm.insert("name".to_string(), package_name.to_string());
        hm.insert("full_name".to_string(), package_full_name.to_string());
        fake_tree.push(hm);

        // println!("{}", package_full_name);
        let mut depends_all_vec = depends_all_vec.clone();
        depends_all_vec.append(&mut depends_vec.clone());
        index = index + 1;
    }

    // serde config
    let serde_config = SerdeConfig { data: fake_tree };
    utils::serde_to_file(DEFAULT_CONFIG_NAME, serde_config);
    utils::move_file_to_dir(DEFAULT_CONFIG_NAME, &target_dir);

    // compress
    println!("Saving...");
    let dest = format!("{}.{}", package_name, DEFAULT_PACKAGE_SUFFIX);
    sevenz_rust::compress_to_path(target_dir, &dest).expect("compress ok");

    // sha256 hash
    println!("Hashing...");
    let hash_str = utils::file_sha256(&dest).unwrap();
    let hash_filename = format!("{}.{}", dest, DEFAULT_SHA256_SUFFIX);
    let _ = utils::write_to_file(&hash_filename, &hash_str);

    // clean dir
    println!("Removing tmp dir...");
    utils::remove_dir(target_dir);
    println!("Done");
}

#[cfg(test)]
mod tests {
    use super::*;
    use env_logger;
    #[test]
    fn test_resolve_depends() {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init()
            .unwrap();

        println!("Test");
        let package_name = "postgresql"; // for test
        let ret = resolve_depends_new(package_name).unwrap();
        println!("{:?}", ret);
    }
}
