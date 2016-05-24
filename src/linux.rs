
use super::{Accesses};

use std::io;
use nix;
use std::ffi::{CString};
use std::collections::HashSet;
use std::env;
use std::os::unix;
use std::path;

fn mkstemp() -> io::Result<unix::io::RawFd> {
    println!("making temp");
    let mode = nix::sys::stat::S_IRUSR | nix::sys::stat::S_IWUSR;
    println!("mode is {:?}", mode);
    let r = try!(nix::fcntl::open(&env::temp_dir(), // path::Path::new(&"."),
                                  nix::fcntl::O_TMPFILE
                                  | nix::fcntl::O_RDWR,
                                  mode));
    // let r = try!(nix::fcntl::open(path::Path::new(&"/tmp/test"),
    //                               nix::fcntl::O_CREAT |
    //                               nix::fcntl::O_RDWR,
    //                               mode));
    println!("done making temp");
    Ok(r)
}

pub fn shell(command_line: &str) -> io::Result<Accesses> {
    use nix::sys::ptrace;
    use std::ptr;
    // let e = mkstemp();
    // if e.is_err() {
    //     use std::process::exit;
    //     panic!("trouble with mkstemp {:?}", e);
    //     exit(7);
    // }
    match try!(nix::unistd::fork()) {
        nix::unistd::Fork::Parent(pid) => {
            use nix::sys::wait::WaitStatus::*;
            use super::ExitStatus;
            try!(nix::unistd::setpgid(pid,pid));  // causes grandchildren to be killed along with firstborn
            println!("my child is {}", pid);
            // Wait for the stopped process
            println!("first wait gives {:?}",
                     try!(nix::sys::wait::waitpid(pid,
                                                  Some(nix::sys::wait::__WALL))));
            println!("set options...");
            try!(ptrace::ptrace_setoptions(pid,
                                           ptrace::ptrace::PTRACE_O_TRACESYSGOOD |
                                           ptrace::ptrace::PTRACE_O_TRACEFORK |
                                           ptrace::ptrace::PTRACE_O_TRACEVFORK |
                                           ptrace::ptrace::PTRACE_O_TRACEVFORKDONE |
                                           ptrace::ptrace::PTRACE_O_TRACECLONE |
                                           ptrace::ptrace::PTRACE_O_TRACEEXEC));
            let null = ptr::null_mut();
            println!("cont...");
            try!(nix::sys::ptrace::ptrace(nix::sys::ptrace::ptrace::PTRACE_CONT,
                                          pid, null, null));
            println!("soon wait...");
            let status =
                try!(nix::sys::wait::waitpid(pid,
                                             Some(nix::sys::wait::__WALL)));
            println!("status is {:?}", &status);
            match status {
                Exited(_,ii) => {
                    unimplemented!();
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
                Stopped(_,signal) => {
                    println!("My child {} stopped with signal {}",
                             pid, signal);
                    unreachable!()
                },
                Continued(_) => unreachable!(),
                StillAlive => unreachable!(),
            }
        },
        nix::unistd::Fork::Child => {
            try!(nix::unistd::setpgid(0,0)); // causes grandchildren to be killed along with firstborn
            println!("I am the child");
            try!(nix::unistd::close(0));
            try!(nix::fcntl::open(path::Path::new(&"/dev/null"),
                                  nix::fcntl::O_RDONLY,
                                  nix::sys::stat::Mode::empty()));
            // try!(nix::unistd::close(1));
            // try!(nix::unistd::close(2));
            // try!(nix::unistd::dup2(stdouterrfd, 1));
            // try!(nix::unistd::dup2(stdouterrfd, 2));
            let null = ptr::null_mut();
            try!(nix::sys::ptrace::ptrace(nix::sys::ptrace::ptrace::PTRACE_TRACEME,
                                          0, null, null));
            let myself = nix::unistd::getpid();
            try!(nix::sys::signal::kill(myself,
                                        nix::sys::signal::signal::SIGSTOP));
            try!(nix::unistd::execvp(&CString::new("sh").unwrap(),
                                     &[CString::new("sh").unwrap(),
                                       CString::new("-c").unwrap(),
                                       CString::new(command_line).unwrap()]));
            unreachable!()
        },
    }
}

#[test]
fn test_into_io_error() {
    use std;
    std::io::Error::from(nix::Error::InvalidPath);
}
