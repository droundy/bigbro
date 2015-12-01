use std::path;
use std::collections::HashSet;

use std::io;
use std::process;

pub fn shell(command_line: &str) -> io::Result<Accesses> {
    let r = try!(try!(process::Command::new("sh").arg("-c")
                      .arg(command_line).spawn()).wait());
    Ok(Accesses {
        status: ExitStatus { exit_code: r.code() },
        read_files: HashSet::new(),
        wrote_files: HashSet::new(),
    })
}

