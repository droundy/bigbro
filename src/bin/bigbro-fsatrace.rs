extern crate bigbro;

use std::io::Write;

/// This code is only intended for testing the bigbro library, and
/// hence does very poor argument handling.

pub fn main() {
    let mut args = std::env::args();
    args.next(); // throw away argv[0]
    let _flags = args.next().unwrap();
    let output_file = args.next().unwrap();
    assert_eq!(Some("--".to_string()), args.next());
    let mut cmd = bigbro::Command::new(args.next().unwrap());
    for a in args {
        cmd.arg(a);
    }
    let status = cmd.status().unwrap();
    let mut file = std::fs::File::create(&output_file)
        .expect(&format!("Trouble creating {:?}", output_file));
    for f in status.read_from_files() {
        writeln!(&mut file, "r|{}", f.to_string_lossy()).unwrap();
    }
    for f in status.written_to_files() {
        writeln!(&mut file, "w|{}", f.to_string_lossy()).unwrap();
    }
    for f in status.read_from_directories() {
        writeln!(&mut file, "l|{}", f.to_string_lossy()).unwrap();
    }
    for f in status.mkdir_directories() {
        writeln!(&mut file, "d|{}", f.to_string_lossy()).unwrap();
    }
}
