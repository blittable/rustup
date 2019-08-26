pub mod squarelib;
pub mod randomveclib;

fn main() {
    println!("Hello, world!");

    // let a = [1, 2, 3];

    // a.iter().map(|&x| x * 2);

    // assert_eq!(vec![2, 4, 6], doubled);

    let (h, w, d) = squarelib::square::generatedimention();
    println!("[Height:{}; Width:{}; Deep:{}]", h, w, d);

    let mut myvec = randomveclib::ramdomvec::generatevec();
    while let Some(top) = myvec.pop() {
        println!("one of myvec is {}", top);
    }
}
