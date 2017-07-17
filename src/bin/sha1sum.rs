fn main() {
    let mut it = std::env::args();
    it.next();
    for arg in it {
        std::fs::File::open(&arg).unwrap();
        println!("opened {}", arg);
    }
}
