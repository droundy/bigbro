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

#[cfg(unix)]
mod linux;

#[cfg(unix)]
pub use linux::{Stdio};

#[cfg(unix)]
use linux as imp;

#[cfg(windows)]
mod winbro;

#[cfg(windows)]
pub use winbro::{Stdio};

#[cfg(windows)]
use winbro as imp;

use std::ffi::{OsString, OsStr};

/// A process builder, providing fine-grained control over how a new
/// process should be spawned.
///
/// Strongly modelled after `std::process::Command`.  A default
/// configuration is generated using `Command::new(program)`, where
/// `program` gives a path to the program to be executed. Additional
/// builder methods allow the configuration to be changed (for
/// example, by adding arguments) prior to running:
///
/// ```
/// use bigbro::Command;
///
/// let status = if cfg!(target_os = "windows") {
///     Command::new("cmd")
///             .args(&["/C", "echo hello"])
///             .status()
///             .expect("failed to execute process")
/// } else {
///     Command::new("sh")
///             .arg("-c")
///             .arg("echo hello")
///             .status()
///             .expect("failed to execute process")
/// };
///
/// assert!(status.status().success());
/// ```
pub struct Command {
    inner: imp::Command,
}

impl Command {
    /// Constructs a new `Command` for launching the program at
    /// path `program`, with the following default configuration:
    ///
    /// * No arguments to the program
    /// * Inherit the current process's environment
    /// * Inherit the current process's working directory
    /// * Inherit stdin/stdout/stderr for `spawn` or `status`, but create pipes for `output`
    ///
    /// Builder methods are provided to change these defaults and
    /// otherwise configure the process.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```no_run
    /// use bigbro::Command;
    ///
    /// Command::new("sh")
    ///         .status()
    ///         .expect("sh command failed to run");
    /// ```
    pub fn new<S: AsRef<OsStr>>(program: S) -> Command {
        Command { inner: imp::Command::new(program) }
    }

    /// Add a single argument to the command.
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command {
        self.inner.arg(arg);
        self
    }

    /// Add arguments to the command.
    pub fn args<I, S>(&mut self, args: I) -> &mut Command
        where I: IntoIterator<Item=S>, S: AsRef<OsStr>
    {
        for arg in args {
            self.inner.arg(arg.as_ref());
        }
        self
    }

    /// Set the working directory for the command.
    pub fn current_dir<P: AsRef<std::path::Path>>(&mut self, dir: P) -> &mut Command {
        self.inner.current_dir(dir);
        self
    }

    /// Set the stdin of the command.
    pub fn stdin(&mut self, cfg: Stdio) -> &mut Command {
        self.inner.stdin(cfg);
        self
    }

    /// Set the stdout of the command.
    pub fn stdout(&mut self, cfg: Stdio) -> &mut Command {
        self.inner.stdout(cfg);
        self
    }

    /// Set the stderr of the command.
    pub fn stderr(&mut self, cfg: Stdio) -> &mut Command {
        self.inner.stderr(cfg);
        self
    }

    /// Set the stderr and stdout of the command to go to a temp file,
    /// from which they can be read after the command is run.
    ///
    /// # Examples
    ///
    /// ```
    /// use bigbro::Command;
    ///
    /// let mut status = Command::new("echo")
    ///                          .arg("-n")
    ///                          .arg("hello")
    ///                          .save_stdouterr()
    ///                          .status()
    ///                          .expect("failed to execute echo");
    ///
    /// assert!(status.status().success() );
    /// let mut f = status.stdout().unwrap();
    /// assert!(f.is_some());
    /// let mut contents = String::new();
    /// f.unwrap().read_to_string(&mut contents);
    /// assert_eq!(contents, "hello");
    /// ```
    pub fn save_stdouterr(&mut self) -> &mut Command {
        self.inner.save_stdouterr();
        self
    }

    /// Run the Command, wait for it to complete, and return its results.
    pub fn status(&mut self) -> std::io::Result<Status> {
        self.inner.status().map(|s| Status { inner: s })
    }
}

/// The result of running a command using bigbro.
///
/// It contains the
/// ExitStatus as well as the information about files and directories
/// accessed by the command.
pub struct Status {
    inner: imp::Status,
}


impl Status {
    /// This returns the `std::process::ExitStatus` of the process.

    /// Was termination successful? Signal termination not considered a success,
    /// and success is defined as a zero exit status.
    ///
    /// # Examples
    ///
    /// ```
    /// use bigbro::Command;
    ///
    /// let status = Command::new("sh")
    ///                      .arg("-c")
    ///                      .arg("false")
    ///                      .status()
    ///                      .expect("failed to execute sh");
    ///
    /// assert!(! status.status().success() ); // should fail because "false" always fails
    /// ```
    pub fn status(&self) -> std::process::ExitStatus {
        self.inner.status()
    }
    /// This retuns the set of directories that the process read from.
    /// For details of what is meant by a process having "read from a
    /// directory", see [semantics](semantics.html).
    ///
    /// # Examples
    ///
    /// ```
    /// use bigbro::Command;
    ///
    /// let dir = std::ffi::OsString::from("/tmp");
    /// let status = Command::new("ls")
    ///                      .arg(&dir)
    ///                      .status()
    ///                      .expect("failed to execute ls");
    ///
    /// assert!(status.status().success() );
    /// assert!(status.read_from_directories().contains(&dir) );
    /// ```
    pub fn read_from_directories(&self) -> std::collections::HashSet<OsString> {
       self.inner.read_from_directories()
    }
    /// This retuns the set of files that the process read.  For
    /// details of what is meant by a process having "read from a
    /// directory", see [semantics](semantics.html).
    ///
    /// # Examples
    ///
    /// ```
    /// use bigbro::Command;
    ///
    /// let p = std::ffi::OsString::from("/usr/bin/python");
    /// let status = Command::new("sha1sum")
    ///                      .arg(&p)
    ///                      .status()
    ///                      .expect("failed to execute sha1sum");
    ///
    /// assert!(status.status().success() );
    /// for f in status.read_from_files() {
    ///    println!("read file {:#?}", f);
    /// }
    /// assert!(status.read_from_files().contains(&p) );
    pub fn read_from_files(&self) -> std::collections::HashSet<OsString> {
        self.inner.read_from_files()
    }
    /// This retuns the set of files that the process wrote to.  For
    /// details of what is meant by a process having "read from a
    /// directory", see [semantics](semantics.html).
    ///
    /// # Examples
    ///
    /// ```
    /// use bigbro::Command;
    ///
    /// let p = std::ffi::OsString::from("/tmp/hello");
    /// let status = Command::new("sh")
    ///                      .args(&["-c", "echo hello > /tmp/hello"])
    ///                      .status()
    ///                      .expect("failed to execute sh");
    ///
    /// assert!(status.status().success() );
    /// for f in status.written_to_files() {
    ///    println!("wrote file {:#?}", f);
    /// }
    /// assert!(status.written_to_files().contains(&p) );
    /// assert!(status.written_to_files().len() == 1 );
    pub fn written_to_files(&self) -> std::collections::HashSet<OsString> {
        self.inner.written_to_files()
    }
    /// This retuns the set of directories that the process created.
    /// For details of what is meant by a process having "read from a
    /// directory", see [semantics](semantics.html).
    pub fn mkdir_directories(&self) -> std::collections::HashSet<OsString> {
        self.inner.mkdir_directories()
    }

    /// This retuns the stdout, if it has been saved using `save_stdouterr`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bigbro::Command;
    ///
    /// let mut status = Command::new("ls")
    ///                          .arg("-l")
    ///                          .save_stdouterr()
    ///                          .status()
    ///                          .expect("failed to execute ls");
    ///
    /// assert!(status.status().success() );
    /// let mut f = status.stdout().unwrap();
    /// assert!(f.is_some());
    /// let mut contents = String::new();
    /// f.unwrap().read_to_string(&mut contents);
    /// println!("ls gives: {}", contents);
    /// assert!(contents.contains("Cargo.toml"));
    /// assert!(contents.contains("src"));
    pub fn stdout(&mut self) -> std::io::Result<Option<Box<std::io::Read>>> {
        self.inner.stdout()
    }
}
