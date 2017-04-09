#![cfg_attr(feature = "strict", deny(warnings))]
#![cfg_attr(feature = "strict", deny(missing_docs))]

//! bigbro is a crate that enables running external commands and
//! tracking their use of the filesystem.  It currently only works
//! under linux.
//!
//! # Example
//!
//! ```
//! let status = bigbro::Command::new("cargo")
//!                             .args(&["--version"])
//!                             .status().unwrap();
//! for f in status.read_from_files() {
//!    println!("read file: {}", f.to_string_lossy());
//! }
//! ```
extern crate libc;

use std;
use std::io::Seek;

use std::ffi::{OsStr, OsString, CString};
use std::io;

/// The result of running a command using bigbro.
///
/// It contains the
/// ExitStatus as well as the information about files and directories
/// accessed by the command.
pub struct Status {
    status: std::process::ExitStatus,
    read_from_directories: std::collections::HashSet<OsString>,
    read_from_files: std::collections::HashSet<OsString>,
    written_to_files: std::collections::HashSet<OsString>,
    mkdir_directories: std::collections::HashSet<OsString>,
    stdout_fd: Option<std::fs::File>,
}

impl Status {
    pub fn status(&self) -> std::process::ExitStatus {
        self.status
    }
    pub fn read_from_directories(&self) -> std::collections::HashSet<OsString> {
       self.read_from_directories.clone()
    }
    pub fn read_from_files(&self) -> std::collections::HashSet<OsString> {
        self.read_from_files.clone()
    }
    pub fn written_to_files(&self) -> std::collections::HashSet<OsString> {
        self.written_to_files.clone()
    }
    pub fn mkdir_directories(&self) -> std::collections::HashSet<OsString> {
        self.mkdir_directories.clone()
    }
    pub fn stdout(&mut self) -> std::io::Result<Option<Box<std::io::Read>>> {
        if let Some(mut f) = self.stdout_fd.take() {
            f.seek(std::io::SeekFrom::Start(0))?;
            return Ok(Some(Box::new(f)));
        }
        Ok(None)
    }
}

pub struct Command {
    argv: Vec<OsString>,
    envs: Option<std::collections::HashMap<OsString, OsString>>,
    workingdir: Option<std::path::PathBuf>,
    stdin: Std,
    stdout: Std,
    stderr: Std,
}

impl Command {
    pub fn new<S: AsRef<std::ffi::OsStr>>(program: S) -> Command {
        Command {
            argv: vec![program.as_ref().to_os_string()],
            envs: None,
            workingdir: None,
            stdin: Std::Inherit,
            stdout: Std::Inherit,
            stderr: Std::Inherit,
        }
    }

    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command {
        self.argv.push(arg.as_ref().to_os_string());
        self
    }
    pub fn current_dir<P: AsRef<std::path::Path>>(&mut self, dir: P) -> &mut Command {
        self.workingdir = Some(std::path::PathBuf::from(dir.as_ref()));
        self
    }

    pub fn stdin(&mut self, cfg: Stdio) -> &mut Command {
        self.stdin = cfg.0;
        self
    }

    pub fn stdout(&mut self, cfg: Stdio) -> &mut Command {
        self.stdout = cfg.0;
        self
    }

    pub fn stderr(&mut self, cfg: Stdio) -> &mut Command {
        self.stderr = cfg.0;
        self
    }

    pub fn save_stdouterr(&mut self) -> &mut Command {
        self
    }

    pub fn status(&mut self) -> io::Result<Status> {
        unimplemented!()
    }
}

enum Std {
    Inherit,
    MakePipe,
    Null,
}

pub struct Stdio(Std);

impl Stdio {
    pub fn piped() -> Stdio { Stdio(Std::MakePipe) }

    pub fn inherit() -> Stdio { Stdio(Std::Inherit) }

    pub fn null() -> Stdio { Stdio(Std::Null) }
}
