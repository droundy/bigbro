//! The bigbro crate.
//!
//! This allows you to track file accesses by child processes.

use std::process;
use std::path;
use std::collections::HashSet;
use std::io;

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

pub fn shell(command_line: &str) -> io::Result<Accesses> {
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
