use glob::glob;
use sevenz_rust;
use std::{collections::HashMap, process::Command};

use crate::utils;
use crate::SerdeConfig;
use crate::DEFAULT_CONFIG_NAME;
use crate::DEFAULT_PACKAGE_SUFFIX;
use crate::DEFAULT_SHA256_SUFFIX;

fn resolve_depends(package_name: &str) -> Option<Vec<String>> {
    let mut depend_vec: Vec<String> = Vec::new();
    let command = Command::new("apt-cache")
        .arg("depends")
        .arg(package_name)
        .output()
        .expect("failed to execute apt-cache");

    let command_output = String::from_utf8_lossy(&command.stdout);
    // println!("{}", command_output);
    let mut lines = command_output.lines();
    let _ = lines.next(); // jump over first line
    for l in lines {
        if l.contains("Depends:") {
            let l_split: Vec<&str> = l.split(": ").collect();
            let depend_name = if l_split.len() == 2 {
                l_split[1]
            } else {
                panic!("Depends length error please contact developer");
            };
            depend_vec.push(depend_name.trim().to_string())
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
    for entry in glob(&pattern).expect("Failed to read glob pattern") {
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
        true => println!("Create dir {} success", &target_dir),
        false => {
            println!("Create dir {} failed", &target_dir);
            return;
        }
    }

    let mut depends_all_vec: Vec<String> = Vec::new();
    let mut index = 0;
    let mut fake_tree: Vec<HashMap<String, String>> = Vec::new();

    let depends_vec = resolve_depends(package_name).unwrap();
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
