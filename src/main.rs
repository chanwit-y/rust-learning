#[repr(u8)]
enum Bar {
    A,
    B,
    C = 10,
    D,
}

fn main() {
    let x = Bar::C as u8;
    let x2 = format!("{:?}", x);
    println!("{}", x2);
}
