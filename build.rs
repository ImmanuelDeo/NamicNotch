// Build script for embedding resources
use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to rerun if resource files change
    println!("cargo:rerun-if-changed=resources/icon.ico");
    println!("cargo:rerun-if-changed=resources/icon.png");
    println!("cargo:rerun-if-changed=resources/icon.rc");
    
    // Compile Windows resource file if it exists
    let resource_path = PathBuf::from("resources/icon.rc");
    if resource_path.exists() {
        embed_resource::compile(resource_path);
    }
    
    // Set version info
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.1.0".to_string());
    println!("cargo:rustc-env=APP_VERSION={}", version);
}