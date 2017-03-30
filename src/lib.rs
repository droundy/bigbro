extern crate libc;

use libc::c_char;
use libc::c_int;

mod private {
    use libc::c_char;
    use libc::c_int;

    #[link(name="fac")]
    extern "C" {
        fn bigbro(workingdir: *const c_char, child_ptr: *mut c_int,
                  stdoutfd: c_int, stderrfd: c_int,
                  envp: *const *const c_char,
                  commandline: *const *const c_char,
                  read_from_directories: *mut *mut *mut c_char,
                  mkdir_directories: *mut *mut *mut c_char,
                  read_from_files: *mut *mut *mut c_char,
                  written_to_files: *mut *mut *mut c_char) -> c_int;
    }
}

pub struct Command {
}

pub fn bigbro(workingdir: &std::path::Path,
              ) {
    unsafe {
    }
}
