fn main() {
    println!("Hello, world!");

    let a = [1, 2, 3];

    let doubled: Vec<i32> = a.iter().map(|&x| x * 2).collect();

    assert_eq!(vec![2, 4, 6], doubled);
}
