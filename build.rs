use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).parent().unwrap();
    let dest_path = target_dir.join("log4rs.yaml");

    fs::copy("config/log4rs.yaml", &dest_path).expect("Failed to copy log4rs.yaml");

    println!("cargo:rerun-if-changed=config/log4rs.yaml");
}