use super::{Accesses, ExitStatus};

use std::process;
use std::collections::HashSet;
use std::io;


pub fn shell(command_line: &str) -> io::Result<Accesses> {
    let r = try!(try!(process::Command::new("sh").arg("-c")
                      .arg(command_line).spawn()).wait());
    Ok(Accesses {
        status: ExitStatus { exit_code: r.code() },
        read_files: HashSet::new(),
        wrote_files: HashSet::new(),
    })
}
