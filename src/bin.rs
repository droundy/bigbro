extern crate bigbro;

use std::process::Command;
use bigbro::BigBro;
pub fn main() {
    println!("running cargo...");
    let status = Command::new("cargo")
        .args(&["--version"])
        .bigbro().unwrap();
    for f in status.read_from_files() {
        println!("read file: {}", f.to_string_lossy());
    }
}
