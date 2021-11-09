fn main() {
    println!("Hello, world!");

    println!("{:?}", Kind::default())
}


#[derive(Debug)]
#[allow(dead_code)]
// #[derive(Default)]
enum Kind {
    A,
    B,
    C,
}

impl Default for Kind {
    fn default() -> Self { Kind::A }
}