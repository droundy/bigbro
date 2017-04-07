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

use std::ffi::{OsStr, OsString, CString};
use std::io;
use libc::{c_int, c_char};

use std::os::unix::process::{ExitStatusExt};

use std::os::unix::ffi::{ OsStrExt };

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

        pub fn setpgid(pid: c_int, pgid: c_int) -> c_int;
        pub fn execvpe(path: *const c_char, argv: *const *const c_char,
                       envp: *const *const c_char) -> c_int;
        pub static environ: *const *const c_char;
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
        self.status
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
       self.read_from_directories.clone()
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
    argv: Vec<CString>,
    envs: Option<std::collections::HashMap<CString, CString>>,
    workingdir: Option<std::path::PathBuf>,
    stdin: Std,
    stdout: Std,
    stderr: Std,
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
    pub fn new<S: AsRef<std::ffi::OsStr>>(program: S) -> Command {
        Command {
            argv: vec![cstr(program.as_ref())],
            envs: None,
            workingdir: None,
            stdin: Std::Inherit,
            stdout: Std::Inherit,
            stderr: Std::Inherit,
        }
    }

    /// Add a single argument to the command.
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command {
        self.argv.push(cstr(arg.as_ref()));
        self
    }

    /// Add arguments to the command.
    pub fn args<I, S>(&mut self, args: I) -> &mut Command
        where I: IntoIterator<Item=S>, S: AsRef<OsStr>
    {
        for arg in args {
            self.arg(arg.as_ref());
        }
        self
    }

    /// Set the stdin of the command.
    pub fn stdin(&mut self, cfg: Stdio) -> &mut Command {
        self.stdin = cfg.0;
        self
    }

    /// Set the stdout of the command.
    pub fn stdout(&mut self, cfg: Stdio) -> &mut Command {
        self.stdout = cfg.0;
        self
    }

    /// Set the stderr of the command.
    pub fn stderr(&mut self, cfg: Stdio) -> &mut Command {
        self.stderr = cfg.0;
        self
    }

    /// Run the Command, wait for it to complete, and return its results.
    pub fn status(&mut self) -> io::Result<Status> {
        let mut args_raw: Vec<*const c_char> =
            self.argv.iter().map(|arg| arg.as_ptr()).collect();
        args_raw.push(std::ptr::null());
        let stdin = self.stdin.to_child_fd()?;
        let stdout = self.stdout.to_child_fd()?;
        let stderr = self.stderr.to_child_fd()?;

        let mut envptr = unsafe { private::environ };
        // the following needs to live beyond execvpe to keep string data alive
        let mut env: Vec<CString>;
        // the following needs to live beyond execvpe to keep string pointers alive
        let envps: Vec<*const c_char>;
        if let Some(ref e) = self.envs {
            env = vec![];
            for (k,v) in e {
                let mut newv: Vec<u8> = vec![];
                newv.extend(k.as_bytes());
                newv.extend(b"=");
                newv.extend(v.as_bytes());
                env.push(unsafe { CString::from_vec_unchecked(newv) });
            }
            envps = env.iter().map(|arg| arg.as_ptr() as *const c_char).collect();
            envptr = envps.as_ptr();
        }

        let mut rd = std::ptr::null_mut();
        let mut rf = std::ptr::null_mut();
        let mut wf = std::ptr::null_mut();
        let mut md = std::ptr::null_mut();
        let pid = unsafe {
            let pid = cvt(libc::fork())?;
            private::setpgid(pid, pid);
            if pid == 0 {
                if let Some(ref p) = self.workingdir {
                    std::env::set_current_dir(p)?;
                }
                if let Some(fd) = stdin {
                        libc::dup2(fd, libc::STDIN_FILENO);
                }
                if let Some(fd) = stdout {
                        libc::dup2(fd, libc::STDOUT_FILENO);
                }
                if let Some(fd) = stderr {
                        libc::dup2(fd, libc::STDERR_FILENO);
                }
                private::bigbro_before_exec();
                private::execvpe(args_raw[0], args_raw.as_ptr(), envptr);
                libc::exit(137)
            }
            pid
        };
        println!("running bigbro_process {}!", pid);
        let exitcode = unsafe {
            private::bigbro_process(pid, &mut rd, &mut md, &mut rf, &mut wf)
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

enum Std {
    Inherit,
    MakePipe,
    Null,
    Fd(std::os::unix::io::RawFd),
}

fn fd_cloexec(fd: std::os::unix::io::RawFd) -> io::Result<()> {
    unsafe {
        if libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

fn dup_cloexec(src: std::os::unix::io::RawFd)
               -> io::Result<std::os::unix::io::RawFd> {
    let fd = unsafe {
        let fd = libc::dup(src);
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        fd
    };
    fd_cloexec(fd)?;
    Ok(fd)
}

impl Std {
    fn to_child_fd(&self) -> io::Result<Option<std::os::unix::io::RawFd>> {
        match *self {
            Std::MakePipe => unimplemented!(),
            Std::Null =>
                Ok(Some(cvt(unsafe {
                    libc::open("/dev/null\0".as_ptr() as *const c_char,
                               libc::O_RDWR)
                })?)),
            Std::Inherit => Ok(None),
            Std::Fd(fd) => if fd >= 0 && fd <= libc::STDERR_FILENO {
                Ok(Some(dup_cloexec(fd)?))
            } else {
                Ok(Some(fd))
            },
        }
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

impl std::os::unix::io::FromRawFd for Stdio {
    unsafe fn from_raw_fd(fd: std::os::unix::io::RawFd) -> Stdio {
        Stdio(Std::Fd(fd))
    }
}

fn cvt(t: c_int) -> io::Result<c_int> {
    if t == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(t)
    }
}
