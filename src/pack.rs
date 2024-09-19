use anyhow::Result;
use glob::glob;
use log::debug;
use log::error;
use log::info;
// use log::warn;
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

fn resolve_depends(package_name: &str, packags_vec: &mut Vec<String>) -> Result<Vec<AptDepends>> {
    info!("resolving: {package_name}");
    if packags_vec.contains(&package_name.to_string()) {
        return Ok(vec![]);
    } else {
        packags_vec.push(package_name.to_string());
    }

    let command = Command::new("apt-cache")
        .arg("depends")
        .arg(package_name)
        .output()?;

    let mut ret = Vec::new();
    let command_output = String::from_utf8_lossy(&command.stdout);
    for line in command_output.lines() {
        if line.contains("Depends:") && !line.contains("|") {
            debug!("depends line: {line}");
            let line_split: Vec<&str> = line
                .split(":")
                .map(|x| x.trim())
                .filter(|x| x.len() > 0)
                .collect();

            if line_split.len() == 2 {
                let depends_package_name = line_split[1];
                debug!("depends package: {depends_package_name}");
                let depends = resolve_depends(depends_package_name, packags_vec)?;
                let apt_depends = AptDepends::new(depends_package_name, &depends);
                ret.push(apt_depends);
            }
        }
    }

    Ok(ret)
}

fn resolve_depends_root(package_name: &str) -> Result<(AptDepends, Vec<String>)> {
    let mut packages_vec = Vec::new();
    let root_depends = resolve_depends(package_name, &mut packages_vec)?;
    let root = AptDepends::new(&package_name, &root_depends);
    debug!("depends root: {:?}", root);
    Ok((root, packages_vec))
}

fn download_depends(package_name: &str, target_dir: &str) -> Result<String> {
    let _ = Command::new("apt")
        .arg("download")
        .arg(package_name)
        .output()?;

    let pattern = format!("{}*.deb", package_name);
    // Searching the downloaded package and move it to tmp dir.
    for entry in glob(&pattern)? {
        match entry {
            Ok(path) => {
                let package_full_name = path.to_string_lossy().to_string();
                debug!("move {package_full_name} to {target_dir}");
                utils::move_file_to_dir(&target_dir, &package_full_name)?;
                return Ok(package_full_name);
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    Ok(String::new())
}

pub fn pack_deb(package_name: &str) -> Result<()> {
    let target_dir = package_name;
    match utils::create_dir(&target_dir) {
        Ok(_) => info!("create tmp dir success!"),
        Err(e) => {
            error!("create tmp dir failed: {e}!");
            return Err(e);
        }
    }

    let (apt_depends, packages_vec) = resolve_depends_root(package_name)?;
    if packages_vec.len() == 0 {
        error!("the [{}] package does not exist!", package_name);
        utils::remove_dir(target_dir)?;
        return Ok(());
    }

    let mut packages_map = HashMap::new();
    for p in &packages_vec {
        let package_full_name = download_depends(p, &target_dir)?;
        info!("downloading: {p}[{package_full_name}]");
        packages_map.insert(p.to_string(), package_full_name);
    }

    // serde config
    let serde_config = SerdeConfig {
        apt_depends,
        packages_vec,
        packages_map,
    };
    utils::serde_config_to_file(DEFAULT_CONFIG_NAME, serde_config)?;
    utils::move_file_to_dir(&target_dir, DEFAULT_CONFIG_NAME)?;

    // compress
    info!("saving...");
    let dest = format!("{}.{}", package_name, DEFAULT_PACKAGE_SUFFIX);
    sevenz_rust::compress_to_path(target_dir, &dest)?;

    // sha256 hash
    info!("hashing...");
    let hash_str = utils::file_sha256(&dest)?;
    let hash_filename = format!("{}.{}", dest, DEFAULT_SHA256_SUFFIX);
    let _ = utils::write_to_file(&hash_filename, &hash_str);

    // clean dir
    info!("removing tmp dir...");
    utils::remove_dir(target_dir)?;
    info!("done!");

    Ok(())
}
