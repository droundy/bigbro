extern crate gcc;

use std::os::unix::io::{FromRawFd, IntoRawFd};

fn main() {

    if ! std::path::Path::new("syscalls/linux.h").exists() {
        let linux_h_fd = std::fs::File::create("syscalls/linux.h")
            .unwrap().into_raw_fd();
        std::process::Command::new("python3")
            .args(&["syscalls/linux.py"])
            .stdout(unsafe {std::process::Stdio::from_raw_fd(linux_h_fd)})
            .status().unwrap();
    }

    gcc::Config::new()
                .flag("-std=c99")
                .file("bigbro-linux.c")
                .include(".")
                .compile("libbigbro.a");
}
