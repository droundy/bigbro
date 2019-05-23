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
extern crate seccomp_droundy;

#[cfg(feature="noprofile")]
extern crate cpuprofiler;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(not(target_os = "linux"))]
mod generic;

#[cfg(target_os = "linux")]
use crate::linux as imp;

#[cfg(not(target_os = "linux"))]
use generic as imp;

pub use crate::imp::{Stdio};

use std::ffi::{OsString, OsStr};
use std::path::PathBuf;

/// A boolean to identify whether bigbro is actually able to track
/// filesystem accesses.  On platforms where it cannot track changes,
/// it will still enable you to use its API to execute programs.
pub const TRACKS_CHANGES: bool = imp::WORKS;

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
/// # use std::env;
/// # if let Some(path) = env::var_os("PATH") {
/// #     let mut paths = env::split_paths(&path).collect::<Vec<_>>();
/// #     let mut debug = env::current_dir().unwrap();
/// #     debug.push("target/debug");
/// #     paths.push(debug);
/// #     let new_path = env::join_paths(paths).unwrap();
/// #     env::set_var("PATH", &new_path);
/// # }
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
    am_blind: bool,
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
            am_blind: false,
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
    /// # use std::env;
    /// # if let Some(path) = env::var_os("PATH") {
    /// #     let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    /// #     let mut debug = env::current_dir().unwrap();
    /// #     debug.push("target/debug");
    /// #     paths.push(debug);
    /// #     let new_path = env::join_paths(paths).unwrap();
    /// #     env::set_var("PATH", &new_path);
    /// # }
    /// # if cfg!(target_os = "linux") {
    /// use bigbro::Command;
    ///
    /// let mut status = Command::new("sh")
    ///                          .arg("-c")
    ///                          .arg("echo $CFLAGS")
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
    /// assert_eq!(contents, "--coverage\n");
    /// # }
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
    /// // for some reason windows doesn't allow to clear HOME or TERM?
    /// if ! cfg!(target_os = "windows") {
    ///   assert_eq!(contents, "FOO=foo\n");
    /// }
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
    /// assert_eq!(contents, "hello\n");
    /// ```
    pub fn save_stdouterr(&mut self) -> &mut Command {
        self.inner.save_stdouterr();
        self
    }

    /// Set the stderr and stdout of the command to go to a file,
    /// from which they can be read after the command is run.
    ///
    /// # Examples
    ///
    /// ```
    /// use bigbro::Command;
    ///
    /// let mut logfile = std::env::temp_dir();
    /// logfile.push("test-file");
    /// let mut status = Command::new("echo")
    ///                          .arg("hello")
    ///                          .arg("world")
    ///                          .log_stdouterr(&logfile)
    ///                          .status()
    ///                          .expect("failed to execute echo");
    ///
    /// assert!(status.status().success() );
    /// let f = status.stdout().unwrap();
    /// assert!(f.is_some());
    /// let mut contents = String::new();
    /// f.unwrap().read_to_string(&mut contents).unwrap();
    /// assert_eq!(contents, "hello world\n");
    /// ```
    pub fn log_stdouterr(&mut self, path: &std::path::Path) -> &mut Command {
        self.inner.log_stdouterr(path);
        self
    }

    /// Run the Command, wait for it to complete, and return its results.
    pub fn status(&mut self) -> std::io::Result<Status> {
        if self.am_blind {
            self.inner.blind(self.envs_cleared, &self.envs_removed, &self.envs_set)
                .map(|s| Status { inner: s })
        } else {
            self.inner.status(self.envs_cleared, &self.envs_removed, &self.envs_set)
                .map(|s| Status { inner: s })
        }
    }

    /// Do not actually track accesses.
    ///
    /// # Examples
    ///
    /// ```
    /// use bigbro::Command;
    ///
    /// let (tx,rx) = std::sync::mpsc::channel();
    /// let mut cmd = Command::new("echo");
    /// cmd.arg("hello").arg("world").save_stdouterr().blind(true);
    /// let _killer = cmd.spawn_and_hook(move |s| { tx.send(s).ok(); })
    ///                  .expect("failed to execute echo");
    /// let mut status = rx.recv().unwrap().unwrap();
    /// assert!(status.status().success() );
    /// let f = status.stdout().unwrap();
    /// assert!(f.is_some());
    /// let mut f = f.unwrap();
    /// let mut contents = String::new();
    /// f.read_to_string(&mut contents).unwrap();
    /// assert_eq!(contents, "hello world\n");
    /// assert_eq!(status.read_from_files().len(), 0); // not tracking means no files listed
    /// ```

    pub fn blind(&mut self, am_blind: bool) -> &mut Command {
        self.am_blind = am_blind;
        self
    }

    /// Start running the Command and return without waiting for it to complete.
    pub fn spawn(self) -> std::io::Result<Child> {
        if self.am_blind {
            unimplemented!()
        } else {
            self.inner.spawn(self.envs_cleared, self.envs_removed, self.envs_set)
                .map(|s| Child { inner: s })
        }
    }

    /// Start running the Command and return without waiting for it to
    /// complete.  Return the final status via a callback, but return the
    /// pid information with which to kill the child synchronously.
    ///
    /// # Examples
    ///
    /// ```
    /// use bigbro::Command;
    ///
    /// let mut logfile = std::env::temp_dir();
    /// logfile.push("test-file");
    /// println!("saving output in file {:?}", &logfile);
    /// let (tx,rx) = std::sync::mpsc::channel();
    /// let mut cmd = Command::new("echo");
    /// cmd.arg("hello").arg("world").log_stdouterr(&logfile);
    /// let _killer = cmd.spawn_and_hook(move |s| { tx.send(s).ok(); })
    ///                  .expect("failed to execute echo");
    /// let mut status = rx.recv().expect("should have gotten *something*!")
    ///                    .expect("the echo command failed to run?");
    /// assert!(status.status().success() );
    /// let f = status.stdout().expect("unable to look at stdout?");
    /// assert!(f.is_some());
    /// let mut contents = String::new();
    /// f.unwrap().read_to_string(&mut contents).expect("unable to read from logfile");
    /// assert_eq!(contents, "hello world\n");
    /// ```
    pub fn spawn_and_hook<F>(self, status_hook: F) -> std::io::Result<Killer>
        where F: FnOnce(std::io::Result<Status>) + Send + 'static
    {
        if self.am_blind {
            self.inner.spawn_hook_blind(self.envs_cleared, self.envs_removed,
                                        self.envs_set, status_hook)
        } else {
            self.inner.spawn_hook(self.envs_cleared, self.envs_removed, self.envs_set,
                                  status_hook)
        }
    }
}

/// A currently running (or possibly already completed) child process.
#[derive(Debug)]
pub struct Child {
    inner: imp::Child,
}

impl Child {
    /// Force the child process to exit
    pub fn kill(&mut self) -> std::io::Result<()> {
        self.inner.kill()
    }
    /// Ask the child process to exit
    pub fn terminate(&mut self) -> std::io::Result<()> {
        self.inner.terminate()
    }
    /// Wait for child to finish
    pub fn wait(&mut self) -> std::io::Result<Status> {
        self.inner.wait().map(|s| Status { inner: s })
    }
    /// Check if the child has finished
    pub fn try_wait(&mut self) -> std::io::Result<Option<Status>> {
        self.inner.try_wait().map(|s| s.map(|s| Status { inner: s}))
    }
}

/// A currently running (or possibly already completed) child process.
#[derive(Debug, Copy, Clone)]
pub struct Killer {
    inner: imp::Killer,
}

impl Killer {
    /// Force the child process to exit
    pub fn kill(&mut self) -> std::io::Result<()> {
        self.inner.kill()
    }
    /// Ask the child process to exit
    pub fn terminate(&mut self) -> std::io::Result<()> {
        self.inner.terminate()
    }
}

/// The result of running a command using bigbro.
///
/// It contains the
/// ExitStatus as well as the information about files and directories
/// accessed by the command.
#[derive(Debug)]
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
    /// # use std::env;
    /// # if let Some(path) = env::var_os("PATH") {
    /// #     let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    /// #     let mut debug = env::current_dir().unwrap();
    /// #     debug.push("target/debug");
    /// #     paths.push(debug);
    /// #     let new_path = env::join_paths(paths).unwrap();
    /// #     env::set_var("PATH", &new_path);
    /// # }
    /// use bigbro::Command;
    ///
    /// let status = Command::new("false")
    ///                      .status()
    ///                      .expect("failed to execute false");
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
    /// # use std::env;
    /// # if let Some(path) = env::var_os("PATH") {
    /// #     let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    /// #     let mut debug = env::current_dir().unwrap();
    /// #     debug.push("target/debug");
    /// #     paths.push(debug);
    /// #     let new_path = env::join_paths(paths).unwrap();
    /// #     env::set_var("PATH", &new_path);
    /// # }
    /// use bigbro::Command;
    ///
    /// let dir = std::env::temp_dir();
    /// let status = Command::new("ls")
    ///                      .arg(&dir)
    ///                      .status()
    ///                      .expect("failed to execute ls");
    ///
    /// assert!(status.status().success() );
    /// if bigbro::TRACKS_CHANGES {
    ///   assert!(status.read_from_directories().contains(&dir) );
    /// }
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
    /// # use std::env;
    /// # if let Some(path) = env::var_os("PATH") {
    /// #     let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    /// #     let mut debug = env::current_dir().unwrap();
    /// #     debug.push("target/debug");
    /// #     paths.push(debug);
    /// #     let new_path = env::join_paths(paths).unwrap();
    /// #     env::set_var("PATH", &new_path);
    /// # }
    /// use bigbro::Command;
    ///
    /// let mut p = env::current_dir().unwrap();
    /// p.push("Cargo.toml");
    /// let e = std::ffi::OsString::from(&p);
    /// let status = Command::new("cat")
    ///                      .arg(&e)
    ///                      .status()
    ///                      .expect("failed to execute cat");
    ///
    /// assert!(status.status().success() );
    /// for f in status.read_from_files() {
    ///    println!("read file {:#?}", f);
    /// }
    /// if bigbro::TRACKS_CHANGES {
    ///   println!("The above list should have {:?}", p);
    ///   assert!(status.read_from_files().contains(&p) );
    /// }
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
    /// # use std::env;
    /// # if let Some(path) = env::var_os("PATH") {
    /// #     let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    /// #     let mut debug = env::current_dir().unwrap();
    /// #     debug.push("target/debug");
    /// #     paths.push(debug);
    /// #     let new_path = env::join_paths(paths).unwrap();
    /// #     env::set_var("PATH", &new_path);
    /// # }
    /// use bigbro::Command;
    ///
    /// let mut p = std::env::temp_dir();
    /// p.push("hello");
    /// let status = Command::new("touch")
    ///                      .arg(&p)
    ///                      .status()
    ///                      .expect("failed to execute sh");
    ///
    /// assert!(status.status().success() );
    /// for f in status.written_to_files() {
    ///    println!("wrote file {:#?}", f);
    /// }
    /// if bigbro::TRACKS_CHANGES {
    ///   assert!(status.written_to_files().contains(&p) );
    ///   assert!(status.written_to_files().len() == 1 );
    /// }
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
    /// # use std::env;
    /// # if let Some(path) = env::var_os("PATH") {
    /// #     let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    /// #     let mut debug = env::current_dir().unwrap();
    /// #     debug.push("target/debug");
    /// #     paths.push(debug);
    /// #     let new_path = env::join_paths(paths).unwrap();
    /// #     env::set_var("PATH", &new_path);
    /// # }
    /// use bigbro::Command;
    ///
    /// let mut status = Command::new("ls")
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

#[cfg(target_os = "linux")]
#[cfg(test)]
fn count_file_descriptors() -> usize {
    let mut count = 0;
    for _ in std::fs::read_dir("/proc/self/fd").unwrap() {
        count += 1;
    }
    println!("open file descriptors: {}", count);
    count
}

#[cfg(target_os = "linux")]
#[test]
fn test_have_closed_fds() {
    let fds = count_file_descriptors();
    {
        let status = Command::new("echo")
            .arg("-n")
            .arg("hello")
            .save_stdouterr()
            .status()
            .expect("failed to execute echo");
        assert!(count_file_descriptors() > fds,
                "save_stdouterr should open a file descriptor?");
        println!("status: {:?}", status);
    }
    assert_eq!(count_file_descriptors(), fds);
    {
        Command::new("ls")
            .status()
            .expect("failed to execute ls");
        assert_eq!(count_file_descriptors(), fds);
    }
    assert_eq!(count_file_descriptors(), fds);
    {
        Command::new("ls")
            .stdin(Stdio::null())
            .status()
            .expect("failed to execute ls");
        assert_eq!(count_file_descriptors(), fds);
    }
    assert_eq!(count_file_descriptors(), fds);
    {
        let status = Command::new("echo")
            .arg("-n")
            .arg("hello")
            .stdin(Stdio::null())
            .save_stdouterr()
            .status()
            .expect("failed to execute echo");
        assert!(count_file_descriptors() > fds);
        println!("status: {:?}", status);
    }
    assert_eq!(count_file_descriptors(), fds);
    {
        let mut logfile = std::env::temp_dir();
        logfile.push("test-file");
        let status = Command::new("echo")
            .arg("-n")
            .arg("hello")
            .stdin(Stdio::null())
            .log_stdouterr(&logfile)
            .status()
            .expect("failed to execute echo");
        assert!(count_file_descriptors() > fds);
        println!("status: {:?}", status);
    }
    assert_eq!(count_file_descriptors(), fds);
}
