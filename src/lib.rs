//! The bigbro crate.
//!
//! This allows you to track file accesses by child processes.

extern crate nix;

use std::path;
use std::collections::HashSet;

#[cfg(not(target_os = "linux"))]
use std::io;
#[cfg(not(target_os = "linux"))]
use std::process;

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
pub fn shell(command_line: &str) -> Result<Accesses, Box<Error>> {
    let r = try!(try!(process::Command::new("sh").arg("-c")
                      .arg(command_line).spawn()).wait());
    Ok(Accesses {
        status: ExitStatus { exit_code: r.code() },
        read_files: HashSet::new(),
        wrote_files: HashSet::new(),
    })
}

#[test]
fn test_true() {
    let a = shell("true").unwrap();
    assert!(a.status.code() == Some(0));
}

#[test]
fn test_mktempdir() {
    let a = shell("mkdir -p tmp").unwrap();
    assert!(a.status.code() == Some(0));
}

#[test]
fn test_echo_to_file() {
    test_mktempdir();
    let a = shell("echo foo > tmp/foo").unwrap();
    assert!(a.status.code() == Some(0));
    // if cfg!(target_os = "linux") {
    //     assert!(a.read_files.contains(&path::PathBuf::from("tmp/foo")));
    // }
}
