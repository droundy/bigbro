extern crate bigbro;

use std::io::Write;
use std::sync::mpsc::channel;

/// This code is only intended for testing the bigbro library, and
/// hence does very poor argument handling.

pub fn main() {
    let mut args = std::env::args();
    args.next(); // throw away argv[0]
    let mut cmd = bigbro::Command::new(args.next().unwrap());
    for a in args {
        cmd.arg(a);
    }
    let (tx,rx) = channel();
    let _killer = cmd.spawn_to_chan(tx).unwrap();
    println!("I am about to wait..");
    let status = rx.recv().unwrap().unwrap();
    println!("status is {:?}", &status);
    for f in status.read_from_files() {
        writeln!(&mut std::io::stderr(), "r: {}", f.to_string_lossy()).unwrap();
    }
    for f in status.written_to_files() {
        writeln!(&mut std::io::stderr(), "w: {}", f.to_string_lossy()).unwrap();
    }
    for f in status.read_from_directories() {
        writeln!(&mut std::io::stderr(), "l: {}", f.to_string_lossy()).unwrap();
    }
    for f in status.mkdir_directories() {
        writeln!(&mut std::io::stderr(), "m: {}", f.to_string_lossy()).unwrap();
    }
}
