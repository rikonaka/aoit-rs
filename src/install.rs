use anyhow::Result;
use log::debug;
use log::error;
use log::info;
use log::warn;
// use log::warn;
use sevenz_rust;
use std::process::Command;

use crate::utils;
use crate::DEFAULT_CONFIG_NAME;
use crate::DEFAULT_SHA256_SUFFIX;

fn install_depends(package_path: &str) -> Result<()> {
    let c = Command::new("dpkg")
        .arg("--install")
        .arg(package_path)
        .output()?;
    debug!("install output: {}", String::from_utf8_lossy(&c.stdout));
    Ok(())
}

pub fn install_deb(aoitfile_name: &str) -> Result<()> {
    // aoitfile_name: vim.aoit
    // sha256 check
    info!("checking...");
    let hash_filename = format!("{}.{}", aoitfile_name, DEFAULT_SHA256_SUFFIX);
    let hash_str = utils::file_sha256(aoitfile_name)?;
    let contents = utils::read_file_bytes(&hash_filename)?;
    let contents = String::from_utf8_lossy(&contents).to_string();
    if hash_str.trim() != contents.trim() {
        error!("calc hash: {hash_str}, file hash: {contents}");
        panic!("check sha256 failed!");
    } else {
        info!("check sha256 success!");
    }

    // get target dir name
    let aoitfile_name_split: Vec<&str> = aoitfile_name.split(".").collect();
    let target_dir = if aoitfile_name_split.len() >= 2 {
        aoitfile_name_split[0].to_string()
    } else {
        panic!("wrong file name, standard files should end with aoit");
    };

    // decompress 7z package
    info!("decompress aoit...");
    // let dest = format!("./{}", target_dir);
    utils::create_dir(&target_dir)?;
    sevenz_rust::decompress_file(aoitfile_name, &target_dir).expect("complete");

    let config_file_path = format!("{}/{}", target_dir, DEFAULT_CONFIG_NAME);
    let serde_config = utils::serde_from_file(&config_file_path)?;
    let mut reverse_packages_vec = serde_config.packages_vec.clone();
    reverse_packages_vec.reverse();

    // reverse install package
    for packages_name in reverse_packages_vec {
        match serde_config.packages_map.get(&packages_name) {
            Some(packages_full_name) => {
                let package_path = format!("{}/{}", target_dir, packages_full_name);
                info!("installing: {}", packages_full_name);
                install_depends(&package_path)?;
            }
            None => warn!("package {packages_name} not found"),
        }
    }

    // delete decompress dir
    info!("removing tmp dir...");
    utils::remove_dir(&target_dir)?;
    info!("done!");
    Ok(())
}
