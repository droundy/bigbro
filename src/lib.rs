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

#[cfg(target_os = "linux")]
mod linux;
#[cfg(not(target_os = "linux"))]
mod generic;

#[cfg(target_os = "linux")]
use linux as imp;

#[cfg(not(target_os = "linux"))]
use generic as imp;

pub use imp::{Stdio};

use std::ffi::{OsString, OsStr};
use std::path::PathBuf;

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
    envs_set: std::collections::HashMap<OsString,OsString>,
    envs_removed: std::collections::HashSet<OsString>,
    envs_cleared: bool,
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
        Command {
            envs_set: std::collections::HashMap::new(),
            envs_removed: std::collections::HashSet::new(),
            envs_cleared: false,
            inner: imp::Command::new(program),
        }
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

    /// Update an environment variable mapping
    ///
    /// # Examples
    ///
    /// ```
    /// use bigbro::Command;
    ///
    /// let mut status = Command::new("sh")
    ///                          .arg("-c")
    ///                          .arg("echo -n $CFLAGS")
    ///                          .env("CFLAGS", "--coverage")
    ///                          .save_stdouterr()
    ///                          .status()
    ///                          .expect("failed to execute sh");
    ///
    /// println!("status: {:?}", status.status());
    /// assert!(status.status().success() );
    /// let f = status.stdout().unwrap();
    /// assert!(f.is_some());
    /// let mut contents = String::new();
    /// f.unwrap().read_to_string(&mut contents).unwrap();
    /// assert_eq!(contents, "--coverage");
    /// ```
    ///
    /// ```
    /// use bigbro::Command;
    ///
    /// let mut status = Command::new("env")
    ///                          .env_clear()
    ///                          .env("FOO", "foo")
    ///                          .save_stdouterr()
    ///                          .status()
    ///                          .expect("failed to execute env");
    ///
    /// println!("status: {:?}", status.status());
    /// assert!(status.status().success() );
    /// let f = status.stdout().unwrap();
    /// assert!(f.is_some());
    /// let mut contents = String::new();
    /// f.unwrap().read_to_string(&mut contents).unwrap();
    /// assert_eq!(contents, "FOO=foo\n");
    /// ```
    pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Command
        where K: AsRef<OsStr>, V: AsRef<OsStr>
    {
        self.envs_removed.remove(key.as_ref());
        self.envs_set.insert(key.as_ref().to_os_string(), val.as_ref().to_os_string());
        self
    }

    /// Add or update multiple environment variable mappings.
    pub fn envs<I, K, V>(&mut self, vars: I) -> &mut Command
        where I: IntoIterator<Item=(K, V)>, K: AsRef<OsStr>, V: AsRef<OsStr>
    {
        for (k,v) in vars {
            self.env(k,v);
        }
        self
    }

    /// Remove an environment variable mapping
    pub fn env_remove<K>(&mut self, key: K) -> &mut Command
        where K: AsRef<OsStr>
    {
        self.envs_set.remove(key.as_ref());
        self.envs_removed.insert(key.as_ref().to_os_string());
        self
    }

    /// Clear the environment for the child
    pub fn env_clear(&mut self) -> &mut Command
    {
        self.envs_cleared = true;
        self.envs_set.clear();
        self.envs_removed.clear();
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
    /// let f = status.stdout().unwrap();
    /// assert!(f.is_some());
    /// let mut contents = String::new();
    /// f.unwrap().read_to_string(&mut contents);
    /// assert_eq!(contents, "hello");
    /// ```
    pub fn save_stdouterr(&mut self) -> &mut Command {
        self.inner.save_stdouterr();
        self
    }

    /// Save stderr and stdout to a file.
    pub fn log_stdouterr(&mut self, path: &std::path::Path) -> &mut Command {
        self.inner.log_stdouterr(path);
        self
    }

    /// Run the Command, wait for it to complete, and return its results.
    pub fn status(&mut self) -> std::io::Result<Status> {
        self.inner.status(self.envs_cleared, &self.envs_removed, &self.envs_set)
            .map(|s| Status { inner: s })
    }

    /// Run the Command without tracking accesses, wait for it to
    /// complete, and return its results.
    pub fn blind(&mut self) -> std::io::Result<Status> {
        self.inner.blind(self.envs_cleared, &self.envs_removed, &self.envs_set)
            .map(|s| Status { inner: s })
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
    /// let dir = std::path::PathBuf::from("/tmp");
    /// let status = Command::new("ls")
    ///                      .arg(&dir)
    ///                      .status()
    ///                      .expect("failed to execute ls");
    ///
    /// assert!(status.status().success() );
    /// assert!(status.read_from_directories().contains(&dir) );
    /// ```
    pub fn read_from_directories(&self) -> std::collections::HashSet<PathBuf> {
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
    /// let e = std::ffi::OsString::from("/usr/bin/python");
    /// let p = std::path::PathBuf::from("/usr/bin/python");
    /// let status = Command::new("sha1sum")
    ///                      .arg(&e)
    ///                      .status()
    ///                      .expect("failed to execute sha1sum");
    ///
    /// assert!(status.status().success() );
    /// for f in status.read_from_files() {
    ///    println!("read file {:#?}", f);
    /// }
    /// assert!(status.read_from_files().contains(&p) );
    pub fn read_from_files(&self) -> std::collections::HashSet<PathBuf> {
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
    /// let p = std::path::PathBuf::from("/tmp/hello");
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
    pub fn written_to_files(&self) -> std::collections::HashSet<PathBuf> {
        self.inner.written_to_files()
    }
    /// This retuns the set of directories that the process created.
    /// For details of what is meant by a process having "read from a
    /// directory", see [semantics](semantics.html).
    pub fn mkdir_directories(&self) -> std::collections::HashSet<PathBuf> {
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
    /// let f = status.stdout().unwrap();
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
