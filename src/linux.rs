#![cfg_attr(feature = "strict", deny(warnings))]
#![cfg_attr(feature = "strict", deny(missing_docs))]

extern crate libc;

use std;
use std::ffi::{OsStr, OsString, CString};
use std::path::PathBuf;
use std::io;
use libc::{c_int, c_char};

use std::os::unix::process::{ExitStatusExt};

use std::io::{Seek};
use std::os::unix::ffi::{ OsStrExt };
use std::os::unix::io::{FromRawFd};

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
    }
}

pub struct Status {
    status: std::process::ExitStatus,
    read_from_directories: std::collections::HashSet<PathBuf>,
    read_from_files: std::collections::HashSet<PathBuf>,
    written_to_files: std::collections::HashSet<PathBuf>,
    mkdir_directories: std::collections::HashSet<PathBuf>,
    stdout_fd: Option<std::fs::File>,
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
        if let Some(mut f) = self.stdout_fd.take() {
            f.seek(std::io::SeekFrom::Start(0))?;
            return Ok(Some(Box::new(f)));
        }
        Ok(None)
    }
}

fn null_c_array_to_pathbuf(a: *const *const c_char) -> std::collections::HashSet<PathBuf> {
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
        v.push(PathBuf::from(osstr));
    }
    v.into_iter().collect()
}

pub struct Command {
    argv: Vec<CString>,
    workingdir: Option<std::path::PathBuf>,
    stdin: Std,
    stdout: Std,
    stderr: Std,
    can_read_stdout: bool,
    have_error: Option<std::io::Error>,
}

impl Command {
    pub fn new<S: AsRef<std::ffi::OsStr>>(program: S) -> Command {
        Command {
            argv: vec![cstr(program.as_ref())],
            workingdir: None,
            stdin: Std::Inherit,
            stdout: Std::Inherit,
            stderr: Std::Inherit,
            can_read_stdout: false,
            have_error: None,
        }
    }

    fn assert_no_error(&mut self) -> std::io::Result<()> {
        if let Some(e) = self.have_error.take() {
            return Err(e)
        }
        Ok(())
    }
    fn errored(&self) -> bool {
        self.have_error.is_some()
    }

    /// Add a single argument to the command.
    pub fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Command {
        self.argv.push(cstr(arg.as_ref()));
        self
    }
    pub fn current_dir<P: AsRef<std::path::Path>>(&mut self, dir: P) {
        self.workingdir = Some(std::path::PathBuf::from(dir.as_ref()));
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

    pub fn save_stdouterr(&mut self) -> &mut Command {
        if ! self.errored() {
            let namebuf = CString::new("/tmp/bigbro-XXXXXX").unwrap();
            let fd = unsafe {
                libc::mkstemp(namebuf.as_ptr() as *mut c_char)
            };
            if fd == -1 {
                self.have_error = Some(io::Error::last_os_error());
                return self;
            }
            unsafe {
                libc::unlink(namebuf.as_ptr() as *const c_char);
                // Ignore error on unlink, since it doesn't precisely hurt
                // to leave the file around, and it's not clear that
                // aborting is a solution?
            }

            self.stderr = Std::Fd(fd);
            self.stdout = Std::Fd(fd);
            self.can_read_stdout = true;
        }
        self
    }

    /// Run the Command, wait for it to complete, and return its results.
    pub fn status(&mut self, envs_cleared: bool,
                  envs_removed: &std::collections::HashSet<OsString>,
                  envs_set: &std::collections::HashMap<OsString,OsString>) -> io::Result<Status> {
        self.assert_no_error()?;
        let mut args_raw: Vec<*const c_char> =
            self.argv.iter().map(|arg| arg.as_ptr()).collect();
        args_raw.push(std::ptr::null());
        let stdin = self.stdin.to_child_fd()?;
        let stdout = self.stdout.to_child_fd()?;
        let stderr = self.stderr.to_child_fd()?;

        let mut rd = std::ptr::null_mut();
        let mut rf = std::ptr::null_mut();
        let mut wf = std::ptr::null_mut();
        let mut md = std::ptr::null_mut();
        let pid = unsafe {
            let pid = cvt(libc::fork())?;
            private::setpgid(pid, pid);
            if pid == 0 {
                if envs_cleared {
                    for (k, _) in std::env::vars_os() {
                        std::env::remove_var(k)
                    }
                }
                for k in envs_removed {
                    std::env::remove_var(k);
                }
                for (k,v) in envs_set {
                    std::env::set_var(k, v);
                }

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
                libc::execvp(args_raw[0], args_raw.as_ptr());
                libc::exit(137)
            }
            pid
        };
        let exitcode = unsafe {
            private::bigbro_process(pid, &mut rd, &mut md, &mut rf, &mut wf)
        };
        let status = Status {
            status: std::process::ExitStatus::from_raw(exitcode),
            read_from_directories: null_c_array_to_pathbuf(rd as *const *const i8),
            read_from_files: null_c_array_to_pathbuf(rf as *const *const i8),
            written_to_files: null_c_array_to_pathbuf(wf as *const *const i8),
            mkdir_directories: null_c_array_to_pathbuf(md as *const *const i8),
            stdout_fd: if self.can_read_stdout {
                if let Some(ref fd) = stdout {
                    Some (unsafe { std::fs::File::from_raw_fd(*fd) })
                } else { None }
            } else { None },
        };
        unsafe {
            libc::free(rd as *mut libc::c_void);
            libc::free(md as *mut libc::c_void);
            libc::free(rf as *mut libc::c_void);
            libc::free(wf as *mut libc::c_void);
        }
        Ok(status)
    }


    /// Run the Command blind, wait for it to complete, and return its results.
    pub fn blind(&mut self, envs_cleared: bool,
                 envs_removed: &std::collections::HashSet<OsString>,
                 envs_set: &std::collections::HashMap<OsString,OsString>) -> io::Result<Status> {
        self.assert_no_error()?;
        let mut args_raw: Vec<*const c_char> =
            self.argv.iter().map(|arg| arg.as_ptr()).collect();
        args_raw.push(std::ptr::null());
        let stdin = self.stdin.to_child_fd()?;
        let stdout = self.stdout.to_child_fd()?;
        let stderr = self.stderr.to_child_fd()?;

        let pid = unsafe {
            let pid = cvt(libc::fork())?;
            private::setpgid(pid, pid);
            if pid == 0 {
                if envs_cleared {
                    for (k, _) in std::env::vars_os() {
                        std::env::remove_var(k)
                    }
                }
                for k in envs_removed {
                    std::env::remove_var(k);
                }
                for (k,v) in envs_set {
                    std::env::set_var(k, v);
                }

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
                libc::execvp(args_raw[0], args_raw.as_ptr());
                libc::exit(137)
            }
            pid
        };
        let exitcode = unsafe {
            let mut st: c_int = 0;
            libc::waitpid(pid, &mut st, 0);
            if libc::WIFEXITED(st) {
                libc::WEXITSTATUS(st)
            } else {
                -1
            }
        };
        let status = Status {
            status: std::process::ExitStatus::from_raw(exitcode),
            read_from_directories: std::collections::HashSet::new(),
            read_from_files: std::collections::HashSet::new(),
            written_to_files: std::collections::HashSet::new(),
            mkdir_directories: std::collections::HashSet::new(),
            stdout_fd: if self.can_read_stdout {
                if let Some(ref fd) = stdout {
                    Some (unsafe { std::fs::File::from_raw_fd(*fd) })
                } else { None }
            } else { None },
        };
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
