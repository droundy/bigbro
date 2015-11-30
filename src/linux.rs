
use super::{Accesses};

use std::io;
use nix;
use std::ffi::{CString};
use std::collections::HashSet;
use std::env;
use std::os::unix;

fn mkstemp() -> io::Result<unix::io::RawFd> {
    let r = try!(nix::fcntl::open(&env::temp_dir(),
                                  nix::fcntl::O_TMPFILE
                                  | nix::fcntl::O_RDWR,
                                  nix::sys::stat::Mode::empty()));
    Ok(r)
}

pub fn shell(command_line: &str) -> io::Result<Accesses> {
    match try!(nix::unistd::fork()) {
        nix::unistd::Fork::Parent(pid) => {
            use nix::sys::wait::WaitStatus::*;
            use super::ExitStatus;
            println!("I am parent of {}", pid);
            match try!(nix::sys::wait::waitpid(pid, None)) {
                Exited(_,ii) => {
                    Ok(Accesses {
                        status: ExitStatus { exit_code: Some(ii as i32) },
                        read_files: HashSet::new(),
                        wrote_files: HashSet::new(),
                    })
                },
                Signaled(_,_,_) => {
                    Ok(Accesses {
                        status: ExitStatus { exit_code: None },
                        read_files: HashSet::new(),
                        wrote_files: HashSet::new(),
                    })
                },
                Stopped(_,_) => unreachable!(),
                Continued(_) => unreachable!(),
                StillAlive => unreachable!(),
            }
        },
        nix::unistd::Fork::Child => {
            try!(nix::unistd::close(0));
            try!(nix::unistd::close(1));
            try!(nix::unistd::close(2));
            let stdouterrfd = try!(mkstemp());
            nix::unistd::dup2(stdouterrfd, 1);
            nix::unistd::dup2(stdouterrfd, 2);
            nix::unistd::execvp(&CString::new("sh").unwrap(),
                                &[CString::new("sh").unwrap(),
                                  CString::new("-c").unwrap(),
                                  CString::new(command_line).unwrap()]);
            unreachable!()
        },
    }
}

#[test]
fn test_into_io_error() {
    use nix::errno::Errno;
    use std;
    std::io::Error::from(nix::Error::InvalidPath);
}
