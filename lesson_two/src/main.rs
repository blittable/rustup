//Playing with immutable and borrowing

fn main() {
    let mut x = 10;
    x = 12;

    println!("{}", x);

    let a = "String A";
    let b = "String B";
    let c = &a;

    println!("{}, {}, {}", a, b, c);

    let d = "String D";
    let e = d.clone();

    println!("{}, {}", d, e);
}

