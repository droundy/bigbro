fn main() {
    let mut it = std::env::args();
    it.next();
    let args: Vec<_> = it.collect();
    if args.len() == 0 {
        let d = std::env::current_dir().unwrap();
        for x in std::fs::read_dir(d).unwrap() {
            println!("{}", x.unwrap().path().display());
        }
    } else {
        for ref d in args.iter() {
            for x in std::fs::read_dir(d).unwrap() {
                println!("{}", x.unwrap().path().display());
            }
        }
    }
}
