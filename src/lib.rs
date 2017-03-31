extern crate libc;

use std::ffi::{OsString};
use std::io;
use libc::{c_int, c_char};

use std::os::unix::process::{CommandExt, ExitStatusExt};

use std::os::unix::ffi::{ OsStrExt };

mod private {
    use libc::c_char;
    use libc::c_int;

    #[link(name="fac")]
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

struct Status {
    status: std::process::ExitStatus,
    read_from_directories: std::collections::HashSet<OsString>,
    read_from_files: std::collections::HashSet<OsString>,
    written_to_files: std::collections::HashSet<OsString>,
    mkdir_directories: std::collections::HashSet<OsString>,
}

trait BigBro {
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
        let mut status: Status;
        status.status = std::process::ExitStatus::from_raw(exitcode);
        status.read_from_directories = null_c_array_to_osstr(rd as *const *const i8);
        status.read_from_files = null_c_array_to_osstr(rf as *const *const i8);
        status.written_to_files = null_c_array_to_osstr(wf as *const *const i8);
        status.mkdir_directories = null_c_array_to_osstr(md as *const *const i8);
        unsafe {
            libc::free(rd as *mut libc::c_void);
            libc::free(md as *mut libc::c_void);
            libc::free(rf as *mut libc::c_void);
            libc::free(wf as *mut libc::c_void);
        }
        unimplemented!()
    }
}
