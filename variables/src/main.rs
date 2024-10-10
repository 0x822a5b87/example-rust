fn main() {
    tup()
}

fn tup() {
    let t: (i32, f64, u8) = (500, 6.4, 1);

    println!("{}", t.0);
    println!("{}", t.1);
    println!("{}", t.2);

    let (x, y, z) = t;

    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
}
