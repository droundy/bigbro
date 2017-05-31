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
use std::io::Write;

#[derive(Debug)]
pub struct Child {
    inner: Option<std::process::Child>,
    want_stdouterr: bool,
    log_stdouterr: Option<PathBuf>,
}

impl Child {
    pub fn kill(&mut self) -> std::io::Result<()> {
        if let Some(mut c) = self.inner.take() {
            c.kill()
        } else {
            Ok(())
        }
    }
    /// Ask the child process to exit
    pub fn terminate(&mut self) -> std::io::Result<()> {
        self.kill()
    }
    /// Wait for child to finish
    pub fn wait(&mut self) -> std::io::Result<Status> {
        if let Some(mut child) = self.inner.take() {
            if self.want_stdouterr {
                let s = child.wait_with_output()?;
                if let Some(ref p) = self.log_stdouterr {
                    let mut f = std::fs::File::open(p)?;
                    f.write(&s.stdout)?;
                }
                Ok(Status {
                    status: s.status,
                    read_from_directories: std::collections::HashSet::new(),
                    read_from_files: std::collections::HashSet::new(),
                    written_to_files: std::collections::HashSet::new(),
                    mkdir_directories: std::collections::HashSet::new(),
                    stdout_fd: Some(s.stdout),
                })
            } else {
                let s = child.wait()?;
                Ok(Status {
                    status: s,
                    read_from_directories: std::collections::HashSet::new(),
                    read_from_files: std::collections::HashSet::new(),
                    written_to_files: std::collections::HashSet::new(),
                    mkdir_directories: std::collections::HashSet::new(),
                    stdout_fd: None,
                })
            }
        } else {
            Err(io::Error::new(io::ErrorKind::Other,"already used up child"))
        }
    }
    /// Check if the child has finished
    pub fn try_wait(&mut self) -> std::io::Result<Option<Status>> {
        unimplemented!()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Killer {
}

impl Killer {
    pub fn kill(&mut self) -> std::io::Result<()> {
        Err(io::Error::new(io::ErrorKind::Other,"Killer not implemented generically"))
    }
    /// Ask the child process to exit
    pub fn terminate(&mut self) -> std::io::Result<()> {
        self.kill()
    }
}

/// The result of running a command using bigbro.
///
/// It contains the
/// ExitStatus as well as the information about files and directories
/// accessed by the command.
#[derive(Debug)]
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
    log_stdouterr: Option<std::path::PathBuf>,
}

impl Command {
    pub fn new<S: AsRef<std::ffi::OsStr>>(program: S) -> Command {
        Command {
            cmd: std::process::Command::new(program),
            want_stdouterr: false,
            log_stdouterr: None,
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

    pub fn log_stdouterr(&mut self, path: &std::path::Path) {
        self.stdout(Stdio::piped());
        self.want_stdouterr = true;
        self.log_stdouterr = Some(PathBuf::from(path));
    }

    /// Run the Command blind, wait for it to complete, and return its results.
    pub fn blind(&mut self, envs_cleared: bool,
                 envs_removed: &std::collections::HashSet<OsString>,
                 envs_set: &std::collections::HashMap<OsString,OsString>) -> io::Result<Status> {
        self.status(envs_cleared, &envs_removed, &envs_set)
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
            if let Some(ref p) = self.log_stdouterr {
                let mut f = std::fs::File::open(p)?;
                f.write(&s.stdout)?;
            }
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
    pub fn spawn(mut self, envs_cleared: bool,
                 envs_removed: std::collections::HashSet<OsString>,
                 envs_set: std::collections::HashMap<OsString,OsString>)
                 -> io::Result<Child>
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
        self.cmd.spawn().map(|c| {
            Child {
                inner: Some(c),
                want_stdouterr: self.want_stdouterr,
                log_stdouterr: self.log_stdouterr.clone(),
            }
        })
    }
    pub fn spawn_to_chans<F>(mut self, envs_cleared: bool,
                             envs_removed: std::collections::HashSet<OsString>,
                             envs_set: std::collections::HashMap<OsString,OsString>,
                             pid_sender: std::sync::mpsc::Sender<Option<::Killer>>,
                             status_hook: F,)
                             -> io::Result<()>
        where F: FnOnce(std::io::Result<::Status>) + Send + 'static
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
        let c = self.cmd.spawn()?;
        let mut myc = Child {
            inner: Some(c),
            want_stdouterr: self.want_stdouterr,
            log_stdouterr: self.log_stdouterr.clone(),
        };
        pid_sender.send(Some(::Killer { inner: Killer {}})).ok();
        std::thread::spawn(move || {
            status_hook(myc.wait().map(|c| ::Status { inner: c }));
        });
        Ok(())
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
