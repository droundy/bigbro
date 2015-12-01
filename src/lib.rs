//! The bigbro crate.
//!
//! This allows you to track file accesses by child processes.

extern crate nix;

use std::path;
use std::collections::HashSet;

pub struct ExitStatus {
    exit_code: Option<i32>,
}
impl ExitStatus {
    pub fn code(&self) -> Option<i32> {
        self.exit_code
    }
}

pub struct Accesses {
    pub status: ExitStatus,
    pub read_files: HashSet<path::PathBuf>,
    pub wrote_files: HashSet<path::PathBuf>,
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::shell;

#[cfg(not(target_os = "linux"))]
mod generic;
#[cfg(not(target_os = "linux"))]
pub use generic::shell;

#[cfg(test)]
mod generic;

#[test]
fn test_true() {
    let a = shell("true").unwrap();
    assert!(a.status.code() == Some(0));
}

#[test]
fn test_mktempdir() {
    std::fs::create_dir_all("tmp").unwrap();
    let a = shell("mkdir -p tmp/dir").unwrap();
    assert!(a.status.code() == Some(0));
}

#[test]
fn test_echo_to_file() {
    std::fs::create_dir_all("tmp").unwrap();
    let a = shell("echo foo > tmp/foo").unwrap();
    assert!(a.status.code() == Some(0));
    // if cfg!(target_os = "linux") {
    //     assert!(a.read_files.contains(&path::PathBuf::from("tmp/foo")));
    // }
}

#[test]
fn test_generic_echo_to_file() {
    std::fs::create_dir_all("tmp").unwrap();
    let a = generic::shell("echo foo > tmp/foo").unwrap();
    assert!(a.status.code() == Some(0));
    // if cfg!(target_os = "linux") {
    //     assert!(a.read_files.contains(&path::PathBuf::from("tmp/foo")));
    // }
}
