extern crate bigbro;

use std::sync::mpsc::channel;

/// This code is only intended for testing the bigbro library, and
/// hence does very poor argument handling.

pub fn main() {
    let args: Vec<_> = std::env::args().collect();
    let count: usize = args[1].parse().unwrap();
    let (tx,rx) = channel();
    for _ in 0..count {
        let mut cmd = bigbro::Command::new(&args[2]);
        cmd.blind(true)
            .save_stdouterr()
            .stdin(bigbro::Stdio::null());
        for a in args.iter().skip(2) {
            cmd.arg(a);
        }
        let tx = tx.clone();
        let _killer = cmd.spawn_and_hook(move |s| { tx.send(s).ok(); }).unwrap();
        rx.recv().unwrap().unwrap();
    }
}
