use clap::Parser;
use env_logger;
use log::error;
use log::info;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

use pack::AptDepends;

mod install;
mod pack;
mod utils;

const DEFAULT_CONFIG_NAME: &str = "config";
const DEFAULT_PACKAGE_SUFFIX: &str = "aoit";
const DEFAULT_SHA256_SUFFIX: &str = "sha256";

/// Apt offline installation tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Packaging deb dependencies
    #[arg(short, long, default_value = "null")]
    pack: String,

    /// Install the packaged deb dependencies
    #[arg(short, long, default_value = "null")]
    install: String,

    /// Verbose
    #[arg(short, long, action)]
    verbose: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerdeConfig {
    apt_depends: AptDepends,
    packages_vec: Vec<String>,
    packages_map: HashMap<String, String>,
}

fn main() {
    let args = Args::parse();
    if args.verbose {
        // env_logger::init();
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .try_init()
            .unwrap();
    } else {
        let _ = env_logger::builder()
            .filter_level(log::LevelFilter::Info)
            .is_test(true)
            .try_init()
            .unwrap();
    }
    if args.pack != "null" {
        match pack::pack_deb(&args.pack) {
            Ok(_) => (),
            Err(e) => error!("pack deb failed: {e}"),
        };
    } else if args.install != "null" {
        match install::install_deb(&args.install) {
            Ok(_) => (),
            Err(e) => error!("install deb failed: {e}"),
        };
    } else {
        info!("Use --help for more infomation");
    }
}
