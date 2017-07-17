fn main() {
    let mut it = std::env::args();
    it.next();
    for arg in it {
        println!("touching {:?}", &arg);
        std::fs::OpenOptions::new().create(true).append(true).open(arg)
            .expect("trouble touching file");
    }
}
