use std::{thread, time};

pub fn start() {
    let ten_millis = time::Duration::from_millis(10);

    loop {
        thread::sleep(ten_millis);
    }
}

fn main() {
    start()
}
