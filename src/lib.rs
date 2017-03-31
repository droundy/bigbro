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

use std::ffi::{OsString};
use std::io;
use libc::{c_int, c_char};

use std::os::unix::process::{CommandExt, ExitStatusExt};

use std::os::unix::ffi::{ OsStrExt };

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
