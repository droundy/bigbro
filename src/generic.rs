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

use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use std::io;

/// The result of running a command using bigbro.
///
/// It contains the
/// ExitStatus as well as the information about files and directories
/// accessed by the command.
pub struct Status {
    status: std::process::ExitStatus,
    read_from_directories: std::collections::HashSet<PathBuf>,
    read_from_files: std::collections::HashSet<PathBuf>,
    written_to_files: std::collections::HashSet<PathBuf>,
    mkdir_directories: std::collections::HashSet<PathBuf>,
    stdout_fd: Option<Vec<u8>>,
}

impl Status {
    pub fn status(&self) -> std::process::ExitStatus {
        self.status
    }
    pub fn read_from_directories(&self) -> std::collections::HashSet<PathBuf> {
       self.read_from_directories.clone()
    }
    pub fn read_from_files(&self) -> std::collections::HashSet<PathBuf> {
        self.read_from_files.clone()
    }
    pub fn written_to_files(&self) -> std::collections::HashSet<PathBuf> {
        self.written_to_files.clone()
    }
    pub fn mkdir_directories(&self) -> std::collections::HashSet<PathBuf> {
        self.mkdir_directories.clone()
    }
    pub fn stdout(&mut self) -> std::io::Result<Option<Box<std::io::Read>>> {
        if let Some(f) = self.stdout_fd.take() {
            return Ok(Some(Box::new(std::io::Cursor::new(f))));
        }
        Ok(None)
    }
}

pub struct Command {
    cmd: std::process::Command,
    want_stdouterr: bool,
}

impl Command {
    pub fn new<S: AsRef<std::ffi::OsStr>>(program: S) -> Command {
        Command {
            cmd: std::process::Command::new(program),
            want_stdouterr: false,
        }
    }

    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) {
        self.cmd.arg(arg);
    }
    pub fn current_dir<P: AsRef<std::path::Path>>(&mut self, dir: P) {
        self.cmd.current_dir(dir);
    }

    pub fn stdin(&mut self, cfg: Stdio) {
        self.cmd.stdin(to_io(cfg));
    }
    pub fn stdout(&mut self, cfg: Stdio) {
        self.cmd.stdout(to_io(cfg));
    }
    pub fn stderr(&mut self, cfg: Stdio) {
        self.cmd.stderr(to_io(cfg));
    }

    pub fn save_stdouterr(&mut self) {
        self.stdout(Stdio::piped());
        self.want_stdouterr = true;
    }

    pub fn status(&mut self, envs_cleared: bool,
                  envs_removed: &std::collections::HashSet<OsString>,
                  envs_set: &std::collections::HashMap<OsString,OsString>)
                  -> io::Result<Status>
    {
        if envs_cleared {
            self.cmd.env_clear();
        }
        for e in envs_removed {
            self.cmd.env_remove(e);
        }
        for (k,v) in envs_set {
            self.cmd.env(k,v);
        }
        if self.want_stdouterr {
            let s = self.cmd.output()?;
            Ok(Status {
                status: s.status,
                read_from_directories: std::collections::HashSet::new(),
                read_from_files: std::collections::HashSet::new(),
                written_to_files: std::collections::HashSet::new(),
                mkdir_directories: std::collections::HashSet::new(),
                stdout_fd: Some(s.stdout),
            })
        } else {
            let s = self.cmd.status()?;
            Ok(Status {
                status: s,
                read_from_directories: std::collections::HashSet::new(),
                read_from_files: std::collections::HashSet::new(),
                written_to_files: std::collections::HashSet::new(),
                mkdir_directories: std::collections::HashSet::new(),
                stdout_fd: None,
            })
        }
    }
}

enum Std {
    Inherit,
    MakePipe,
    Null,
}

fn to_io(i: Stdio) -> std::process::Stdio {
    match i.0 {
        Std::Inherit => std::process::Stdio::inherit(),
        Std::MakePipe => std::process::Stdio::piped(),
        Std::Null => std::process::Stdio::null(),
    }
}

/// A description of what you want done with one of the standard streams.
pub struct Stdio(Std);

impl Stdio {
    /// A new pipe should be arranged to connect the parent and child processes.
    pub fn piped() -> Stdio { Stdio(Std::MakePipe) }

    /// The child inherits from the corresponding parent descriptor.
    pub fn inherit() -> Stdio { Stdio(Std::Inherit) }

    /// This stream will be ignored. This is the equivalent of attaching the
    /// stream to `/dev/null`
    pub fn null() -> Stdio { Stdio(Std::Null) }
}
