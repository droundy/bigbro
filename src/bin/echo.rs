fn main() {
    let mut it = std::env::args();
    it.next();
    if let Some(arg) = it.next() {
        print!("{}", arg);
    }
    for arg in it {
        print!(" {}", arg);
    }
    println!();
}
