fn main() {
    let mut it = std::env::args();
    it.next();
    for arg in it {
        std::fs::OpenOptions::new().append(true).open(arg).unwrap();
    }
}
