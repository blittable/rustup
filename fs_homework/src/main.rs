use std::fs;
use std::io::*;

fn main() {
    //READ
    let mut fp = fs::OpenOptions::new()
        .read(true)
        .open("sample/bird.bmp")
        .unwrap();
    let mut buffer = Vec::new();
    let len = fp.read_to_end(&mut buffer).unwrap();

    //INVERT
    let address = ((buffer[10] as u32) << 0) 
                + ((buffer[11] as u32) << 8) 
                + ((buffer[12] as u32) << 16) 
                + ((buffer[13] as u32) << 24);
    let start = address as usize;
    for x in start..len {
        buffer[x] = !buffer[x]
    }

    //WRITE
    let mut fp = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open("sample/output.bmp")
        .unwrap();

    fp.write_all(&buffer).unwrap();
}