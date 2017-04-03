extern crate bigbro;

use bigbro::BigBro;
pub fn main() {
    println!("running cargo...");
    let status = bigbro::Command::new("cargo")
        .args(&["--version"])
        .status().unwrap();
    for f in status.read_from_files() {
        println!("r: {}", f.to_string_lossy());
    }
    for f in status.written_to_files() {
        println!("w: {}", f.to_string_lossy());
    }
    for f in status.read_from_directories() {
        println!("l: {}", f.to_string_lossy());
    }
    for f in status.mkdir_directories() {
        println!("m: {}", f.to_string_lossy());
    }
}
