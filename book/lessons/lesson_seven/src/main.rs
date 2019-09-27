fn main() {
    println!("Hello, world!");

    let a = [1, 2, 3];

    a.iter().map(|&x| x * 2);

    assert_eq!(vec![2, 4, 6], doubled);
}
