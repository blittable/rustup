use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    let k = [1, 2, 3];
    thread::spawn(|| {
        let n = k[0];
    });
}
