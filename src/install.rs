use sevenz_rust;
use std::process::Command;

use crate::utils;
use crate::DEFAULT_CONFIG_NAME;
use crate::DEFAULT_SHA256_SUFFIX;

fn install_depends(package_path: &str) {
    let _ = Command::new("dpkg")
        .arg("--install")
        .arg(package_path)
        .output()
        .expect("Failed to excute apt install");

    // println!("{}", String::from_utf8_lossy(&c.stdout));
}

pub fn install_deb(aoitfile_name: &str) {
    // aoitfile_name: vim.aoit

    // sha256 check
    println!("Checking...");
    let hash_filename = format!("{}.{}", aoitfile_name, DEFAULT_SHA256_SUFFIX);
    let hash_str = utils::file_sha256(aoitfile_name).unwrap();
    let contents = utils::read_file_bytes(&hash_filename).unwrap();
    let contents = String::from_utf8_lossy(&contents).to_string();
    if hash_str.trim() != contents.trim() {
        panic!("Check sha256 failed");
    } else {
        println!("Check sha256 success");
    }

    // get target dir name
    let aoitfile_name_split: Vec<&str> = aoitfile_name.split(".").collect();
    let target_dir = if aoitfile_name_split.len() >= 2 {
        aoitfile_name_split[0].to_string()
    } else {
        panic!("Filename error, standard files should end with aoit");
    };

    // decompress 7z package
    println!("Decompress aoit...");
    // let dest = format!("./{}", target_dir);
    utils::create_dir(&target_dir);
    sevenz_rust::decompress_file(aoitfile_name, &target_dir).expect("complete");

    let target_config = format!("{}/{}", target_dir, DEFAULT_CONFIG_NAME);
    let serde_config = utils::serde_from_file(&target_config).unwrap();
    let mut fake_tree = serde_config.data;
    fake_tree.reverse();

    // reverse install package
    for hm in fake_tree {
        let package_path = format!("{}/{}", target_dir, hm.get("full_name").unwrap());
        println!("Install: {}", hm.get("full_name").unwrap());
        install_depends(&package_path);
    }

    // delete decompress dir
    println!("Removing tmp dir...");
    utils::remove_dir(&target_dir);

    println!("Done");
}
