extern crate gcc;

use std::env;

fn main() {

    let target = env::var("TARGET").unwrap();

    if target.contains("windows") {
        gcc::Config::new()
            .flag("-std=c99")
            .file("bigbro-windows.c")
            .include(".")
            .compile("libbigbro.a");
    } else if target.contains("linux") {
        gcc::Config::new()
            .flag("-std=c99")
            .file("bigbro-linux.c")
            .include(".")
            .compile("libbigbro.a");
    } else {
        // It would be lovely to have a completely portable version
        // that did no access tracking...
    }
}
