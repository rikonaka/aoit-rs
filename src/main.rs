use clap::Parser;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

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
    /// For packaging deb dependencies
    #[arg(short, long, default_value = "null")]
    pack: String,

    /// Install the packaged deb dependencies
    #[arg(short, long, default_value = "null")]
    install: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SerdeConfig {
    data: Vec<HashMap<String, String>>,
}

fn main() {
    let args = Args::parse();
    if args.pack != "null" {
        pack::pack_deb(&args.pack);
    } else if args.install != "null" {
        install::install_deb(&args.install);
    } else {
        println!("Use --help for more infomation");
    }
}
