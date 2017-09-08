#![cfg_attr(feature = "strict", deny(warnings))]
#![cfg_attr(feature = "strict", deny(missing_docs))]

extern crate libc;
extern crate seccomp;

use std;
use std::ffi::{OsStr, OsString, CString};
use std::path::PathBuf;
use std::io;
use libc::{c_int, c_char};

use std::os::unix::process::{ExitStatusExt};
use std::os::unix::ffi::{OsStringExt};
use std::collections::HashSet;

use std::io::{Seek};
use std::os::unix::ffi::{ OsStrExt };
use std::os::unix::io::{FromRawFd};

#[cfg(feature="noprofile")]
use cpuprofiler::PROFILER;

#[cfg(feature="noprofile")]
fn stop_profiling() {
    PROFILER.lock().unwrap().stop().unwrap();
}
#[cfg(not(feature="noprofile"))]
fn stop_profiling() {
}

pub const WORKS: bool = true;

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

#[derive(Debug)]
pub struct Child {
    pid: c_int,
    have_completed: std::sync::Arc<std::sync::atomic::AtomicBool>,
    status_thread: Option<std::thread::JoinHandle<std::io::Result<Status>>>,
}

impl Child {
    /// Force the child process to exit
    pub fn kill(&self) -> std::io::Result<()> {
        let code = unsafe { libc::kill(self.pid, libc::SIGKILL) };
        if code < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    /// Ask the child process to exit
    pub fn terminate(&self) -> std::io::Result<()> {
        let code = unsafe { libc::kill(self.pid, libc::SIGTERM) };
        if code < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    /// Wait for child to finish
    pub fn wait(&mut self) -> std::io::Result<Status> {
        let x = self.status_thread.take();
        if let Some(jh) = x {
            match jh.join() {
                Ok(v) => v,
                Err(_) => Err(io::Error::new(io::ErrorKind::Other,"error joining")),
            }
        } else {
            Err(io::Error::new(io::ErrorKind::Other,"already used up child"))
        }
    }
    /// Check if the child has finished
    pub fn try_wait(&mut self) -> std::io::Result<Option<Status>> {
        if self.have_completed.load(std::sync::atomic::Ordering::Relaxed) {
            self.wait().map(|s| Some(s))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Killer {
    pid: c_int,
}

impl Killer {
    /// Force the child process to exit
    pub fn kill(&self) -> std::io::Result<()> {
        let code = unsafe { libc::kill(self.pid, libc::SIGKILL) };
        if code < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    /// Ask the child process to exit
    pub fn terminate(&self) -> std::io::Result<()> {
        let code = unsafe { libc::kill(self.pid, libc::SIGTERM) };
        if code < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

#[derive(Debug)]
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

    fn realpath(&mut self, path: PathBuf, lasth: LastSymlink) -> PathBuf {
        // FIXME allocating lots of tiny OsStrings is definitely not
        // optimal here.  It seems worth trying to change "elements"
        // to a PathBuf, so our temporaries will always be stored
        // continguously with minimum heap allocation.  However,
        // PathBuf is not ideal for storing a path "in reverse".
        let mut result = PathBuf::from("/");
        let mut elements = Vec::new();
        for c in path.components().rev().filter(|c| *c != std::path::Component::RootDir) {
            elements.push(std::ffi::OsString::from(c.as_os_str()));
        }
        while let Some(next) = elements.pop() {
            if next == std::ffi::OsStr::new("..") {
                result.pop();
            } else {
                result.push(next.as_os_str());
                if elements.len() > 0 || lasth == LastSymlink::Followed {
                    if let Ok(linkval) = result.read_link() {
                        self.read_from_files.insert(result.clone());
                        result.pop();
                        for c in linkval.components().rev() {
                            elements.push(std::ffi::OsString::from(c.as_os_str()));
                        }
                    }
                } else {
                    return result;
                }
            }
        }
        result
    }
    fn realpath_at(&mut self, pid: i32, dirfd: i32, path: PathBuf,
                   lasth: LastSymlink) -> PathBuf {
        if path == std::path::Path::new("") {
            return path;
        }

        if let Ok(procstuff) = path.strip_prefix("/proc/self") {
            return self.realpath(PathBuf::from(format!("/proc/{}", pid)).join(procstuff), lasth);
        }
        if path.is_absolute() {
            return self.realpath(path, lasth);
        }

        let proc_fd = if dirfd == libc::AT_FDCWD {
            PathBuf::from(format!("/proc/{}/cwd", pid))
        } else {
            PathBuf::from(format!("/proc/{}/fd/{}", pid, dirfd))
        };
        if let Ok(cwd) = proc_fd.read_link() {
            self.realpath(cwd.join(path), lasth)
        } else {
            println!("Unable to determine cwd from {:?}", proc_fd);
            PathBuf::from("")
        }
    }

    fn bigbro_process(&mut self, pid: i32) {
        let mut status = 0;
        unsafe {
            libc::waitpid(pid, &mut status, 0);
            if libc::WIFEXITED(status) {
                // This probably means that tracing with PTRACE_TRACEME didn't
                // work, since the child should have stopped before exiting.  At
                // this point there isn't much to do other than return the exit
                // code.  Presumably we are running under seccomp?
                self.status = std::process::ExitStatus::from_raw(libc::WEXITSTATUS(status));
                return;
            }
            assert!(libc::WIFSTOPPED(status));
            //println!("signal is {}", libc::WSTOPSIG(status));
            assert_eq!(libc::WSTOPSIG(status), libc::SIGSTOP);
            let extra_ptrace_flags = libc::PTRACE_O_TRACEFORK |
                                     libc::PTRACE_O_TRACEVFORK |
                                     libc::PTRACE_O_TRACEVFORKDONE |
                                     libc::PTRACE_O_TRACECLONE |
                                     libc::PTRACE_O_TRACEEXEC;

            if libc::ptrace(libc::PTRACE_SETOPTIONS, pid, 0,
                            libc::PTRACE_O_TRACESECCOMP | extra_ptrace_flags) != 0 {
                println!("error tracing with seccomp?!");
                if libc::ptrace(libc::PTRACE_SETOPTIONS, pid, 0,
                                libc::PTRACE_O_TRACESYSGOOD | extra_ptrace_flags) != 0 {
                    println!("error tracing with tracesysgood?!");
                }
            }
        }
        // ptrace(PTRACE_SETOPTIONS, child, 0, my_ptrace_options);
        // if (ptrace(PTRACE_SYSCALL, child, 0, 0)) {
        //   // I'm not sure what this error is, but if we can't resume the
        //   // process probably we should exit.
        //   return -1;
        // }
        unsafe {
            libc::ptrace(libc::PTRACE_CONT, pid, 0, 0);
        }
        while self.wait_for_syscall(pid) {
        }
    }
    fn wait_for_syscall(&mut self, pid: i32) -> bool {
        let mut status = 0;
        let mut signal_to_send_back = 0;
        let child = unsafe { libc::waitpid(-pid, &mut status, 0) };
        let keep_going: bool = unsafe {
            const PTRACE_EVENT_FORK: i32 = 1;
            const PTRACE_EVENT_VFORK: i32 = 2;
            const PTRACE_EVENT_CLONE: i32 = 3;
            const PTRACE_EVENT_EXEC: i32 = 4;
            const PTRACE_EVENT_SECCOMP: i32 = 7;
            if status>>8 == (libc::SIGTRAP | (PTRACE_EVENT_SECCOMP<<8)) {
                // it is a seccomp stop
                let mut syscall_num = 0;
                libc::ptrace(libc::PTRACE_GETEVENTMSG, child, 0, &mut syscall_num);
                match SYSCALLS[syscall_num] {
                    Syscall::Open | Syscall::OpenAt => {
                        let args = get_args(child);
                        let retval = wait_for_return(child);
                        let flag;
                        let dirfd;
                        let path;
                        if SYSCALLS[syscall_num] == Syscall::Open {
                            path = read_a_string(child, args[0]);
                            dirfd = libc::AT_FDCWD;
                            flag = args[1] as i32;
                        } else {
                            path = read_a_string(child, args[1]);
                            dirfd = args[0] as i32;
                            flag = args[2] as i32;
                        }
                        let path = self.realpath_at(child, dirfd, path,
                                                    LastSymlink::Followed);
                        println!("{}({:?},{}) -> {}", SYSCALLS[syscall_num].tostr(),
                                 path, flag, retval);
                        if path.is_file() {
                            if retval >= 0 {
                                if flag & libc::O_WRONLY != 0 || flag & libc::O_RDWR != 0 {
                                    self.read_from_files.remove(&path);
                                    self.written_to_files.insert(path);
                                } else {
                                    self.read_from_files.insert(path);
                                }
                            }
                        }
                    },
                    Syscall::Mkdir | Syscall::Mkdirat => {
                        let args = get_args(child);
                        let retval = wait_for_return(child);
                        if retval == 0 {
                            let dirfd;
                            let path;
                            if SYSCALLS[syscall_num] == Syscall::Mkdir {
                                path = read_a_string(child, args[0]);
                                dirfd = libc::AT_FDCWD;
                            } else {
                                path = read_a_string(child, args[1]);
                                dirfd = args[0] as i32;
                            }
                            let path = self.realpath_at(child, dirfd, path,
                                                        LastSymlink::Followed);
                            println!("{}({:?}) -> 0", SYSCALLS[syscall_num].tostr(),
                                     path);
                            self.mkdir_directories.insert(path);
                        } else {
                            println!("{}(?) -> {}", SYSCALLS[syscall_num].tostr(), retval);
                        }
                    },
                    Syscall::Futimesat | Syscall::Utimensat => {
                        let args = get_args(child);
                        let retval = wait_for_return(child);
                        // I don't understand why the args[1] != 0
                        // check below is needed.  The function should
                        // not succeed with a null pointer for the
                        // path, but somehow it seems to sometimes do
                        // so.  :(
                        if retval == 0 && args[1] != 0 {
                            let dirfd = args[0] as i32;
                            let path = read_a_string(child, args[1]);
                            let follow = if SYSCALLS[syscall_num] == Syscall::Futimesat {
                                LastSymlink::Followed
                            } else {
                                if args[3] as i32 & libc::AT_SYMLINK_FOLLOW != 0 {
                                    LastSymlink::Followed
                                } else {
                                    LastSymlink::Returned
                                }
                            };
                            let path = self.realpath_at(child, dirfd, path, follow);
                            println!("{}({:?}) -> 0", SYSCALLS[syscall_num].tostr(),
                                     path);
                            self.written_to_files.insert(path);
                        }
                    },
                    Syscall::Link | Syscall::Linkat => {
                        let args = get_args(child);
                        let retval = wait_for_return(child);
                        if retval == 0 {
                            let tofd;
                            let fromfd;
                            let to;
                            let from;
                            let follow;
                            if SYSCALLS[syscall_num] == Syscall::Link {
                                from = read_a_string(child, args[0]);
                                to = read_a_string(child, args[1]);
                                tofd = libc::AT_FDCWD;
                                fromfd = libc::AT_FDCWD;
                                follow = LastSymlink::Returned;
                            } else {
                                fromfd = args[0] as i32;
                                from = read_a_string(child, args[1]);
                                tofd = args[2] as i32;
                                to = read_a_string(child, args[3]);
                                follow = if args[4] as i32 & libc::AT_SYMLINK_FOLLOW != 0 {
                                    LastSymlink::Followed
                                } else {
                                    LastSymlink::Returned
                                };
                            }
                            let to = self.realpath_at(child, tofd, to, follow);
                            let from = self.realpath_at(child, fromfd, from, follow);
                            println!("{}({:?} -> {:?}) -> 0", SYSCALLS[syscall_num].tostr(),
                                     &from, &to);
                            self.read_from_files.insert(from);
                            self.written_to_files.insert(to);
                        }
                    },
                    Syscall::Symlink | Syscall::Symlinkat => {
                        let args = get_args(child);
                        let retval = wait_for_return(child);
                        if retval == 0 {
                            let dirfd;
                            let path;
                            if SYSCALLS[syscall_num] == Syscall::Symlink {
                                path = read_a_string(child, args[1]);
                                dirfd = libc::AT_FDCWD;
                            } else {
                                path = read_a_string(child, args[2]);
                                dirfd = args[1] as i32;
                            }
                            let path = self.realpath_at(child, dirfd, path,
                                                        LastSymlink::Returned);
                            println!("{}({:?} -> ?) -> 0", SYSCALLS[syscall_num].tostr(),
                                     &path);
                            self.written_to_files.insert(path);
                        }
                    },
                    Syscall::Execve | Syscall::Execveat => {
                        let args = get_args(child);
                        let dirfd;
                        let path;
                        if SYSCALLS[syscall_num] == Syscall::Execve {
                            path = read_a_string(child, args[0]);
                            dirfd = libc::AT_FDCWD;
                        } else {
                            path = read_a_string(child, args[1]);
                            dirfd = args[0] as i32;
                        }
                        let path = self.realpath_at(child, dirfd, path,
                                                    LastSymlink::Followed);
                        if path != std::path::Path::new("") {
                            println!("{}({:?}) -> 0", SYSCALLS[syscall_num].tostr(), path);
                            self.read_from_files.insert(path);
                        }
                    },
                    Syscall::Unlink | Syscall::Unlinkat => {
                        let args = get_args(child);
                        let retval = wait_for_return(child);
                        if retval == 0 {
                            let dirfd;
                            let path;
                            if SYSCALLS[syscall_num] == Syscall::Unlink {
                                path = read_a_string(child, args[0]);
                                dirfd = libc::AT_FDCWD;
                            } else {
                                path = read_a_string(child, args[1]);
                                dirfd = args[0] as i32;
                            }
                            let path = self.realpath_at(child, dirfd, path,
                                                        LastSymlink::Followed);
                            println!("{}({:?}) -> 0", SYSCALLS[syscall_num].tostr(),
                                     path);
                            self.read_from_files.remove(&path);
                            self.written_to_files.remove(&path);
                        } else {
                            println!("{}(?) -> {}", SYSCALLS[syscall_num].tostr(), retval);
                        }
                    },
                    Syscall::Getdents => {
                        let args = get_args(child);
                        let dirfd = args[0] as i32;
                        let path = self.realpath_at(child, dirfd, PathBuf::from("."),
                                                    LastSymlink::Followed);
                        let retval = wait_for_return(child);
                        if retval == 0 {
                            println!("{}({}) -> {}", SYSCALLS[syscall_num].tostr(),
                                     dirfd, retval);
                            self.read_from_directories.insert(path);
                        }
                    },
                    Syscall::Chdir => {
                        let args = get_args(child);
                        let path = read_a_string(child, args[0]);
                        let path = self.realpath_at(child, libc::AT_FDCWD, path,
                                                    LastSymlink::Followed);
                        println!("{}({:?})", SYSCALLS[syscall_num].tostr(), path);
                    },
                    Syscall::Lstat => {
                        let args = get_args(child);
                        let path = read_a_string(child, args[0]);
                        let path = self.realpath(path, LastSymlink::Returned);
                        if let Ok(md) = path.symlink_metadata() {
                            if md.file_type().is_symlink() || md.file_type().is_file() {
                                println!("{}({:?})", SYSCALLS[syscall_num].tostr(), path);
                                self.read_from_files.insert(path);
                            }
                        }
                    },
                    Syscall::Readlinkat => {
                        let args = get_args(child);
                        let dirfd = args[0] as i32;
                        let path = read_a_string(child, args[1]);
                        let path = self.realpath_at(child, dirfd, path,
                                                    LastSymlink::Returned);
                        let retval = wait_for_return(child);
                        if retval == 0 {
                            println!("{}({:?}) -> {}", SYSCALLS[syscall_num].tostr(),
                                     path, retval);
                            println!("readdir path is {:?}", path);
                            self.read_from_files.insert(path);
                        }
                    },
                    Syscall::Stat => {
                        let args = get_args(child);
                        let path = read_a_string(child, args[0]);
                        let path = self.realpath_at(child, libc::AT_FDCWD,
                                                    path, LastSymlink::Followed);
                        if let Ok(md) = path.metadata() {
                            if md.file_type().is_symlink() || md.file_type().is_file() {
                                println!("{}({:?})", SYSCALLS[syscall_num].tostr(),
                                         path);
                                self.read_from_files.insert(path);
                            }
                        }
                    },
                }
                true
            } else if libc::WIFEXITED(status) {
                // This probably means that tracing with PTRACE_TRACEME didn't
                // work, since the child should have stopped before exiting.  At
                // this point there isn't much to do other than return the exit
                // code.  Presumably we are running under seccomp?
                if child == pid {
                    self.status = std::process::ExitStatus::from_raw(libc::WEXITSTATUS(status));
                    false
                } else {
                    true
                }
            } else if libc::WIFSIGNALED(status) {
                println!("process {} died of a signal!\n", child);
                if child == pid {
                    self.status = std::process::ExitStatus::from_raw(-libc::WTERMSIG(status));
                    println!("child died of signal");
                    false
                } else {
                    true  /* no need to do anything more for this guy */
                }
            } else if libc::WIFSTOPPED(status) && (status>>8) == (libc::SIGTRAP | PTRACE_EVENT_FORK << 8) {
                let mut newpid = 0;
                libc::ptrace(libc::PTRACE_GETEVENTMSG, child, 0, &mut newpid);
                //println!("{}: forked from {}\n", newpid, child);
                true
            } else if libc::WIFSTOPPED(status) && (status>>8) == (libc::SIGTRAP | PTRACE_EVENT_VFORK << 8) {
                let mut newpid = 0;
                libc::ptrace(libc::PTRACE_GETEVENTMSG, child, 0, &mut newpid);
                //println!("{}: vforked from {}\n", newpid, child);
                true
            } else if libc::WIFSTOPPED(status) && (status>>8) == (libc::SIGTRAP | PTRACE_EVENT_CLONE << 8) {
                let mut newpid = 0;
                libc::ptrace(libc::PTRACE_GETEVENTMSG, child, 0, &mut newpid);
                //println!("{}: cloned from {}\n", newpid, child);
                true
            } else if libc::WIFSTOPPED(status) && (status>>8) == (libc::SIGTRAP | PTRACE_EVENT_EXEC << 8) {
                let mut newpid = 0;
                libc::ptrace(libc::PTRACE_GETEVENTMSG, child, 0, &mut newpid);
                //println!("{}: execed from {}\n", newpid, child);
                true
            } else if libc::WIFSTOPPED(status) {
                // ensure that the signal we interrupted is actually delivered.
                match libc::WSTOPSIG(status) {
                    libc::SIGCHLD | libc::SIGTRAP | libc::SIGVTALRM => {
                        // I don't know why forwarding SIGCHLD along
                        // causes trouble.  :( SIGTRAP is what we get
                        // from ptrace.  For some reason SIGVTALRM
                        // causes trouble with ghc.
                        // println!("{}: ignoring signal {}\n", child, libc::WSTOPSIG(status));
                    },
                    _ => {
                        signal_to_send_back = libc::WSTOPSIG(status);
                        // println!("{}: sending signal {}\n", child, signal_to_send_back);
                    }
                };
                true
            } else {
                println!("Saw someting else?");
                true
            }
        };
        // tell the child to keep going!
        unsafe {
            if libc::ptrace(libc::PTRACE_CONT, child, 0, signal_to_send_back) == -1 {
                // Assume child died and that we will get a WIFEXITED
                // shortly.
            }
        }
        keep_going
    }
}


#[derive(Debug,Clone,Copy,Eq,PartialEq)]
enum LastSymlink {
    Followed,
    Returned,
}

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
enum Syscall {
    Open, OpenAt, Getdents, Lstat, Stat, Readlinkat, Mkdir, Mkdirat,
    Unlink, Unlinkat, Chdir, Link, Linkat, Symlink, Symlinkat,
    Execve, Execveat, Futimesat, Utimensat,
}
impl Syscall {
    fn seccomp(&self) -> Vec<seccomp::Syscall> {
        match *self {
            Syscall::Open => vec![seccomp::Syscall::open],
            Syscall::OpenAt => vec![seccomp::Syscall::openat],
            Syscall::Getdents => vec![seccomp::Syscall::getdents,
                                      seccomp::Syscall::getdents64],
            Syscall::Lstat => vec![seccomp::Syscall::lstat,
                                   seccomp::Syscall::lstat64,
                                   seccomp::Syscall::readlink,],
            Syscall::Stat => vec![seccomp::Syscall::stat,
                                  seccomp::Syscall::stat64],
            Syscall::Readlinkat => vec![seccomp::Syscall::readlinkat],
            Syscall::Mkdir => vec![seccomp::Syscall::mkdir],
            Syscall::Mkdirat => vec![seccomp::Syscall::mkdirat],
            Syscall::Link => vec![seccomp::Syscall::link],
            Syscall::Linkat => vec![seccomp::Syscall::linkat],
            Syscall::Symlink => vec![seccomp::Syscall::symlink],
            Syscall::Symlinkat => vec![seccomp::Syscall::symlinkat],
            Syscall::Unlink => vec![seccomp::Syscall::unlink],
            Syscall::Unlinkat => vec![seccomp::Syscall::unlinkat],
            Syscall::Execve => vec![seccomp::Syscall::execve],
            Syscall::Execveat => vec![seccomp::Syscall::execveat],
            Syscall::Chdir => vec![seccomp::Syscall::chdir],
            Syscall::Futimesat => vec![seccomp::Syscall::futimesat],
            Syscall::Utimensat => vec![seccomp::Syscall::utimensat],
        }
    }
    fn tostr(&self) -> &'static str {
        match *self {
            Syscall::Open => "open",
            Syscall::OpenAt => "openat",
            Syscall::Getdents => "getdents",
            Syscall::Lstat => "lstat/readlink",
            Syscall::Stat => "stat",
            Syscall::Readlinkat => "readlinkat",
            Syscall::Mkdir => "mkdir",
            Syscall::Mkdirat => "mkdirat",
            Syscall::Link => "link",
            Syscall::Linkat => "linkat",
            Syscall::Symlink => "symlink",
            Syscall::Symlinkat => "symlinkat",
            Syscall::Unlink => "unlink",
            Syscall::Unlinkat => "unlinkat",
            Syscall::Execve => "execve",
            Syscall::Execveat => "execveat",
            Syscall::Chdir => "chdir",
            Syscall::Futimesat => "futimesat",
            Syscall::Utimensat => "utimensat",
        }
    }
}

const SYSCALLS: &[Syscall] = &[
    Syscall::Open, Syscall::OpenAt, Syscall::Getdents, Syscall::Lstat, Syscall::Stat,
    Syscall::Readlinkat, Syscall::Mkdir, Syscall::Mkdirat,
    Syscall::Unlink, Syscall::Unlinkat, Syscall::Chdir, Syscall::Link, Syscall::Linkat,
    Syscall::Symlink, Syscall::Symlinkat, Syscall::Execve, Syscall::Execveat,
    Syscall::Futimesat, Syscall::Utimensat,
];

fn seccomp_context() -> std::io::Result<seccomp::Context> {
    let mut ctx = seccomp::Context::default(seccomp::Action::Allow).unwrap();
    ctx.add_arch(seccomp::ARCH_X86_64).ok();
    ctx.add_arch(seccomp::ARCH_X86).ok();
    ctx.add_arch(seccomp::ARCH_X32).ok();
    for (i,sc) in SYSCALLS.iter().cloned().enumerate() {
        for secc in sc.seccomp() {
            ctx.add_rule(seccomp::Rule::trace(secc, i as u32)).unwrap();
        }
    }
    Ok(ctx)
}

fn get_args(child: i32) -> [usize;6] {
    let mut regs: libc::user_regs_struct = unsafe { std::mem::zeroed() };
    if unsafe { libc::ptrace(libc::PTRACE_GETREGS, child, 0, &mut regs) } == -1 {
        println!("error getting registers for {}!\n", child);
        unsafe { std::mem::zeroed() }
    } else {
        if regs.cs == 0x23 {
            // child is actually x86 not x86_64..
            [regs.rbx as usize,
             regs.rcx as usize,
             regs.rdx as usize,
             regs.rsi as usize,
             regs.rdi as usize,
             regs.rbp as usize]
        } else {
            [regs.rdi as usize,
             regs.rsi as usize,
             regs.rdx as usize,
             regs.r10 as usize,
             regs.r8 as usize,
             regs.r9 as usize]
        }
    }
}

fn wait_for_return(child: i32) -> i32 {
    let mut status = 0;
    unsafe {
        libc::ptrace(libc::PTRACE_SYSCALL, child, 0, 0); // ignore return value
        libc::waitpid(child, &mut status, 0);
        let mut regs: libc::user_regs_struct = std::mem::zeroed();
        if libc::ptrace(libc::PTRACE_GETREGS, child, 0, &mut regs) == -1 {
            println!("error getting registers for {}!\n", child);
        }
        regs.rax as i32
    }
}

fn read_a_string(child: i32, addr: usize) -> std::path::PathBuf {
    if addr == 0 {
        return std::path::PathBuf::from("");
    };

    // There is a tradeoff here between allocating something too large
    // and wasting memory vs the cost of reallocing repeatedly.
    let mut val = Vec::with_capacity(1024);
    let mut read = 0;
    loop {
        let tmp = unsafe { libc::ptrace(libc::PTRACE_PEEKDATA, child, addr + read) };
        if std::io::Error::last_os_error().raw_os_error().is_some() && std::io::Error::last_os_error().raw_os_error() != Some(0) {
            break;
        }
        let ip: *const _ = &tmp;
        let bp: *const u8 = ip as *const _;
        let sz = std::mem::size_of_val(&tmp);
        let bs: &[u8] = unsafe { std::slice::from_raw_parts(bp, sz) };
        val.extend(bs.iter().cloned().take_while(|&c| c != 0));
        if bs.contains(&0) {
            break;
        }
        read += sz;
    }
    std::ffi::OsString::from_vec(val).into()
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

    pub fn log_stdouterr(&mut self, path: &std::path::Path) -> &mut Command {
        if ! self.errored() {
            let namebuf = cstr(path.as_os_str());
            let fd = unsafe { libc::open(namebuf.as_ptr(),
                                         libc::O_RDWR|libc::O_CREAT|libc::O_TRUNC,
                                         libc::S_IRWXU|libc::S_IRWXG|libc::S_IRWXO) };
            if fd == -1 {
                self.have_error = Some(io::Error::last_os_error());
                return self;
            }

            self.stderr = Std::Fd(fd);
            self.stdout = Std::Fd(fd);
            self.can_read_stdout = true;
        }
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

        let ctx = seccomp_context()?;
        let pid = unsafe {
            let pid = cvt(libc::fork())?;
            private::setpgid(pid, pid);
            if pid == 0 {
                // Avoid profiling the forked commands.  This
                // simplifies the profiling process for users of our
                // library.  Of course, it also means they can't
                // profile bigbro itself.
                stop_profiling();
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
                    if let Err(_) = std::env::set_current_dir(p) {
                        libc::_exit(137)
                    }
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
                match ctx.load() {
                    Ok(()) => Ok(()),
                    Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other,e)),
                }?;
                if libc::ptrace(libc::PTRACE_TRACEME,0,0,0) != 0 {
                    // UNABLE TO USE ptrace! This probably means
                    // seccomp is in use through docker or the like,
                    // and means bigbro won't work at all.  Currently,
                    // bigbro ignores this situation, but avoid
                    // stopping here, since if we stop we won't be
                    // able to restart using ptrace.  Perhaps we
                    // should return with an error?
                    eprint!("Unable to trace child, perhaps seccomp too strict?!\n");
                } else {
                    libc::kill(libc::getpid(), libc::SIGSTOP);
                }
                libc::execvp(args_raw[0], args_raw.as_ptr());
                libc::_exit(137)
            }
            pid
        };
        // Before we do anything else, let us clean up any file
        // descriptors we might have hanging around to avoid any
        // leaks:
        self.stdin.close_fd_if_appropriate(stdin);
        if !self.can_read_stdout {
            self.stdout.close_fd_if_appropriate(stdout);
        }
        if stderr != stdout {
            self.stderr.close_fd_if_appropriate(stderr);
        }
        let mut status = Status {
            status: std::process::ExitStatus::from_raw(137),
            read_from_directories: HashSet::new(),
            read_from_files: HashSet::new(),
            written_to_files: HashSet::new(),
            mkdir_directories: HashSet::new(),
            stdout_fd: if self.can_read_stdout {
                if let Some(ref fd) = stdout {
                    Some (unsafe { std::fs::File::from_raw_fd(*fd) })
                } else { None }
            } else { None },
        };
        status.bigbro_process(pid);
        Ok(status)
    }


    /// Start running the Command and return without waiting for it to complete.
    pub fn spawn(mut self, envs_cleared: bool,
                 envs_removed: std::collections::HashSet<OsString>,
                 envs_set: std::collections::HashMap<OsString,OsString>)
                 -> io::Result<Child>
    {
        self.assert_no_error()?;

        let have_completed = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let have_completed_two = have_completed.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        let status_thread = Some(std::thread::spawn(move || {
            // Avoid profiling the helper threads.  This simplifies
            // the profiling process for users of our library.  Of
            // course, it also means they can't profile bigbro itself.
            stop_profiling();
            let mut args_raw: Vec<*const c_char> =
                self.argv.iter().map(|arg| arg.as_ptr()).collect();
            args_raw.push(std::ptr::null());
            let stdinfd = self.stdin.to_child_fd()?;
            let stdoutfd = self.stdout.to_child_fd()?;
            let stderrfd = self.stderr.to_child_fd()?;
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
                    if let Some(fd) = stdinfd {
                        libc::dup2(fd, libc::STDIN_FILENO);
                    }
                    if let Some(fd) = stdoutfd {
                        libc::dup2(fd, libc::STDOUT_FILENO);
                    }
                    if let Some(fd) = stderrfd {
                        libc::dup2(fd, libc::STDERR_FILENO);
                    }
                    private::bigbro_before_exec();
                    libc::execvp(args_raw[0], args_raw.as_ptr());
                    libc::exit(137)
                }
                pid
            };
            tx.send(pid).expect("Error reporting pid");
            // Before we do anything else, let us clean up any file
            // descriptors we might have hanging around to avoid any
            // leaks:
            self.stdin.close_fd_if_appropriate(stdinfd);
            if !self.can_read_stdout {
                self.stdout.close_fd_if_appropriate(stdoutfd);
            }
            if stderrfd != stdoutfd {
                self.stderr.close_fd_if_appropriate(stderrfd);
            }
            let mut rd = std::ptr::null_mut();
            let mut rf = std::ptr::null_mut();
            let mut wf = std::ptr::null_mut();
            let mut md = std::ptr::null_mut();
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
                    if let Some(ref fd) = stdoutfd {
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
            have_completed_two.store(true, std::sync::atomic::Ordering::Relaxed);
            Ok(status)
        }));
        let pid = rx.recv().expect("Error learning pid");
        Ok(Child {
            pid: pid,
            have_completed: have_completed,
            status_thread: status_thread,
        })
    }


    /// Start running the Command and return without waiting for it to complete.
    pub fn spawn_hook<F>(mut self, envs_cleared: bool,
                         envs_removed: std::collections::HashSet<OsString>,
                         envs_set: std::collections::HashMap<OsString,OsString>,
                         status_hook: F,)
                         -> io::Result<::Killer>
        where F: FnOnce(std::io::Result<::Status>) + Send + 'static
    {
        self.assert_no_error()?;

        let stdinfd = self.stdin.to_child_fd()?;
        let stdoutfd = self.stdout.to_child_fd()?;
        let stderrfd = self.stderr.to_child_fd()?;
        let (tx,rx) = std::sync::mpsc::sync_channel(1);
        std::thread::spawn(move || -> () {
            let mut args_raw: Vec<*const c_char> =
                self.argv.iter().map(|arg| arg.as_ptr()).collect();
            args_raw.push(std::ptr::null());
            let pid = unsafe {
                let pid = match cvt(libc::fork()) {
                    Ok(pid) => pid,
                    Err(e) => {
                        status_hook(Err(e));
                        return;
                    },
                };
                private::setpgid(pid, pid);
                if pid == 0 {
                    // Avoid profiling the forked commands.  This
                    // simplifies the profiling process for users of
                    // our library.  Of course, it also means they
                    // can't profile bigbro itself.
                    stop_profiling();
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
                        if let Err(e) = std::env::set_current_dir(p) {
                            status_hook(Err(e));
                            return;
                        }
                    }
                    if let Some(fd) = stdinfd {
                        libc::dup2(fd, libc::STDIN_FILENO);
                    }
                    if let Some(fd) = stdoutfd {
                        libc::dup2(fd, libc::STDOUT_FILENO);
                    }
                    if let Some(fd) = stderrfd {
                        libc::dup2(fd, libc::STDERR_FILENO);
                    }
                    private::bigbro_before_exec();
                    libc::execvp(args_raw[0], args_raw.as_ptr());
                    libc::exit(137)
                }
                pid
            };
            tx.send(pid).ok();
            // Before we do anything else, let us clean up any file
            // descriptors we might have hanging around to avoid any
            // leaks:
            self.stdin.close_fd_if_appropriate(stdinfd);
            if !self.can_read_stdout {
                self.stdout.close_fd_if_appropriate(stdoutfd);
            }
            if stderrfd != stdoutfd {
                self.stderr.close_fd_if_appropriate(stderrfd);
            }
            let mut rd = std::ptr::null_mut();
            let mut rf = std::ptr::null_mut();
            let mut wf = std::ptr::null_mut();
            let mut md = std::ptr::null_mut();
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
                    if let Some(ref fd) = stdoutfd {
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
            status_hook(Ok(::Status { inner: status }));
        });
        match rx.recv() {
            Ok(pid) => Ok(::Killer { inner: Killer { pid: pid }}),
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other,e)),
        }
    }

    /// Start running the Command and return without waiting for it to complete.
    pub fn spawn_hook_blind<F>(mut self, envs_cleared: bool,
                               envs_removed: std::collections::HashSet<OsString>,
                               envs_set: std::collections::HashMap<OsString,OsString>,
                               status_hook: F,)
                               -> io::Result<::Killer>
        where F: FnOnce(std::io::Result<::Status>) + Send + 'static
    {
        self.assert_no_error()?;

        // Avoid profiling the helper threads.  This simplifies the
        // profiling process for users of our library.  Of course, it
        // also means they can't profile bigbro itself.
        stop_profiling();
        let mut args_raw: Vec<*const c_char> =
            self.argv.iter().map(|arg| arg.as_ptr()).collect();
        args_raw.push(std::ptr::null());
        let stdinfd = self.stdin.to_child_fd()?;
        let stdoutfd = self.stdout.to_child_fd()?;
        let stderrfd = self.stderr.to_child_fd()?;
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
                    if let Err(e) = std::env::set_current_dir(p) {
                        status_hook(Err(e));
                        libc::exit(137);
                    }
                }
                if let Some(fd) = stdinfd {
                    libc::dup2(fd, libc::STDIN_FILENO);
                }
                if let Some(fd) = stdoutfd {
                    libc::dup2(fd, libc::STDOUT_FILENO);
                }
                if let Some(fd) = stderrfd {
                    libc::dup2(fd, libc::STDERR_FILENO);
                }
                libc::execvp(args_raw[0], args_raw.as_ptr());
                libc::exit(137)
            }
            pid
        };
        std::thread::spawn(move || -> () {
            // Before we do anything else, let us clean up any file
            // descriptors we might have hanging around to avoid any
            // leaks:
            self.stdin.close_fd_if_appropriate(stdinfd);
            if !self.can_read_stdout {
                self.stdout.close_fd_if_appropriate(stdoutfd);
            }
            if stderrfd != stdoutfd {
                self.stderr.close_fd_if_appropriate(stderrfd);
            }
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
                    if let Some(ref fd) = stdoutfd {
                        Some (unsafe { std::fs::File::from_raw_fd(*fd) })
                    } else { None }
                } else { None },
            };
            status_hook(Ok(::Status { inner: status }));
        });
        Ok(::Killer { inner: Killer { pid: pid, }})
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
                // Avoid profiling the forked commands.  This
                // simplifies the profiling process for users of our
                // library.  Of course, it also means they can't
                // profile bigbro itself.
                stop_profiling();
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
        // Before we do anything else, let us clean up any file
        // descriptors we might have hanging around to avoid any
        // leaks:
        self.stdin.close_fd_if_appropriate(stdin);
        if !self.can_read_stdout {
            self.stdout.close_fd_if_appropriate(stdout);
        }
        if stderr != stdout {
            self.stderr.close_fd_if_appropriate(stderr);
        }
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

#[derive(Clone,Debug)]
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
    fn close_fd_if_appropriate(&self, fd: Option<std::os::unix::io::RawFd>) {
        if let Some(fd) = fd {
            match *self {
                Std::MakePipe => unimplemented!(),
                Std::Null => unsafe { libc::close(fd); },
                Std::Inherit => (),
                Std::Fd(_) => unsafe { libc::close(fd); },
            }
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
