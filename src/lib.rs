//! bigbro is a crate that enables running external commands and
//! tracking their use of the filesystem.  It currently only works
//! under linux.
//!
//! # Example
//!
//! ```
//! use std::process::Command;
//! use bigbro::BigBro;
//!
//! let status = Command::new("cargo")
//!                      .args(&["--version"])
//!                      .bigbro().unwrap();
//! for f in status.read_from_files() {
//!    println!("read file: {}", f.to_string_lossy());
//! }
//! ```
extern crate libc;

use std::ffi::{OsStr, OsString, CString};
use std::io;
use libc::{c_int, c_char};

use std::os::unix::process::{CommandExt, ExitStatusExt};

use std::os::unix::ffi::{ OsStrExt, OsStringExt };

fn cstr(x: &OsStr) -> CString {
    CString::new(x.as_bytes()).unwrap()
}

mod private {
    use libc::c_char;
    use libc::c_int;

    #[link(name="bigbro")]
    extern "C" {
        // fn bigbro(workingdir: *const c_char, child_ptr: *mut c_int,
        //           stdoutfd: c_int, stderrfd: c_int,
        //           envp: *const *const c_char,
        //           commandline: *const *const c_char,
        //           read_from_directories: *mut *mut *mut c_char,
        //           mkdir_directories: *mut *mut *mut c_char,
        //           read_from_files: *mut *mut *mut c_char,
        //           written_to_files: *mut *mut *mut c_char) -> c_int;
        pub fn bigbro_before_exec();
        pub fn bigbro_process(child: c_int,
                          read_from_directories: *mut *mut *mut c_char,
                          mkdir_directories: *mut *mut *mut c_char,
                          read_from_files: *mut *mut *mut c_char,
                          written_to_files: *mut *mut *mut c_char) -> c_int;
    }
}

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
}

impl Status {
    pub fn status(&self) -> std::process::ExitStatus {
        self.status
    }
    /// This retuns the set of directories that the process read from.
    /// For details of what is meant by a process having "read from a
    /// directory", see [semantics](semantics.html).
    pub fn read_from_directories(&self) -> std::collections::HashSet<OsString> {
       self.read_from_directories.clone()
    }
    /// This retuns the set of files that the process read.  For
    /// details of what is meant by a process having "read from a
    /// directory", see [semantics](semantics.html).
    pub fn read_from_files(&self) -> std::collections::HashSet<OsString> {
        self.read_from_files.clone()
    }
    /// This retuns the set of files that the process wrote to.  For
    /// details of what is meant by a process having "read from a
    /// directory", see [semantics](semantics.html).
    pub fn written_to_files(&self) -> std::collections::HashSet<OsString> {
        self.written_to_files.clone()
    }
    /// This retuns the set of directories that the process created.
    /// For details of what is meant by a process having "read from a
    /// directory", see [semantics](semantics.html).
    pub fn mkdir_directories(&self) -> std::collections::HashSet<OsString> {
        self.mkdir_directories.clone()
    }
}

/// This trait adds a single method that enables running bigbro.
/// Perhaps I should have created a function rather than adding a
/// method?
pub trait BigBro {
    /// Run the command while tracking all reads and writes to the
    /// filesystem.
    fn bigbro(&mut self) -> io::Result<Status>;
}

fn bb_before() -> std::io::Result<()> {
    unsafe { private::bigbro_before_exec(); }
    Ok(())
}

fn null_c_array_to_osstr(a: *const *const c_char) -> std::collections::HashSet<OsString> {
    if a == std::ptr::null() {
        return vec![].into_iter().collect(); // surely there is a nicer way to get empty set?
    }
    let mut count = 0;
    unsafe {
        while *a.offset(count as isize) != std::ptr::null() {
            count += 1;
        }
    }
    let sl = unsafe { std::slice::from_raw_parts(a, count) };
    let mut v = vec![];
    for s in sl {
        let mut strlen = 0;
        unsafe {
            while *s.offset(strlen as isize) != 0 {
                strlen += 1;
            }
        }
        let osstr = std::ffi::OsStr::from_bytes(unsafe {
            std::slice::from_raw_parts(*s as *const u8, strlen) });
        v.push(osstr.to_owned());
    }
    v.into_iter().collect()
}

pub struct Command {
    argv: Vec<CString>,
    envs: Option<std::collections::HashMap<CString, CString>>,
    workingdir: Option<std::path::PathBuf>,
    stdin: StdioInner,
    stdout: StdioInner,
    stderr: StdioInner,
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
    ///         .spawn()
    ///         .expect("sh command failed to start");
    /// ```
    pub fn new<S: AsRef<std::ffi::OsStr>>(program: S) -> Command {
        Command {
            argv: vec![cstr(program.as_ref())],
            envs: None,
            workingdir: None,
            stdin: StdioInner::Inherit,
            stdout: StdioInner::Inherit,
            stderr: StdioInner::Inherit,
        }
    }

    pub fn spawn(&mut self) -> io::Result<Child> {
        let mut args_raw: Vec<*const c_char> =
            self.argv.iter().map(|arg| arg.as_ptr()).collect();
        args_raw.push(std::ptr::null());
        let pid = unsafe {
            match cvt(libc::fork())? {
                0 => {
                    libc::execvp(args_raw[0], args_raw.as_ptr());
                    libc::exit(137)
                },
                n => n,
            }
        };
        Ok(Child {
            stdin: None,
            stdout: None,
            stderr: None,
            pid: pid,
        })
    }
}

enum StdioInner {
    Inherit,
    MakePipe,
    Null,
    Fd(std::os::unix::io::RawFd),
}

pub struct Stdio(StdioInner);

impl Stdio {
    /// A new pipe should be arranged to connect the parent and child processes.
    pub fn piped() -> Stdio { Stdio(StdioInner::MakePipe) }

    /// The child inherits from the corresponding parent descriptor.
    pub fn inherit() -> Stdio { Stdio(StdioInner::Inherit) }

    /// This stream will be ignored. This is the equivalent of attaching the
    /// stream to `/dev/null`
    pub fn null() -> Stdio { Stdio(StdioInner::Null) }
}

impl std::os::unix::io::FromRawFd for Stdio {
    unsafe fn from_raw_fd(fd: std::os::unix::io::RawFd) -> Stdio {
        Stdio(StdioInner::Fd(fd))
    }
}

impl BigBro for std::process::Command {
    fn bigbro(&mut self) -> io::Result<Status> {
        self.before_exec(bb_before);
        let mut rd = std::ptr::null_mut();
        let mut rf = std::ptr::null_mut();
        let mut wf = std::ptr::null_mut();
        let mut md = std::ptr::null_mut();
        let child = try!(self.spawn());
        let exitcode = unsafe {
            private::bigbro_process(child.id() as c_int, &mut rd, &mut md, &mut rf, &mut wf)
        };
        let status = Status {
            status: std::process::ExitStatus::from_raw(exitcode),
            read_from_directories: null_c_array_to_osstr(rd as *const *const i8),
            read_from_files: null_c_array_to_osstr(rf as *const *const i8),
            written_to_files: null_c_array_to_osstr(wf as *const *const i8),
            mkdir_directories: null_c_array_to_osstr(md as *const *const i8),
        };
        unsafe {
            libc::free(rd as *mut libc::c_void);
            libc::free(md as *mut libc::c_void);
            libc::free(rf as *mut libc::c_void);
            libc::free(wf as *mut libc::c_void);
        }
        Ok(status)
    }
}

pub struct ChildStdin {
    inner: std::os::unix::io::RawFd,
}
pub struct ChildStdout {
    inner: std::os::unix::io::RawFd,
}
pub struct ChildStderr {
    inner: std::os::unix::io::RawFd,
}

pub struct Child {
    pub stdin: Option<ChildStdin>,
    pub stdout: Option<ChildStdout>,
    pub stderr: Option<ChildStderr>,
    pid: c_int,
}

fn cvt(t: c_int) -> io::Result<c_int> {
    if t == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}
