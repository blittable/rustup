use serde::{Deserialize, Serialize};
use std::fs;
use std::io::prelude::*;
use std::io::SeekFrom;


fn main() {
    //Read image file and deserialize
    let mut fp = fs::OpenOptions::new()
                    .read(true)
                    .open("bird.bmp")
                    .unwrap();

    let mut buffer = Vec::new();
    let len = fp.read_to_end(&mut buffer).unwrap();
    let jump_position: u32 = bincode::deserialize(&buffer[10..14]).unwrap();
    let jump = jump_position as usize;

    //Get header of image and change color in content
    let mut header = &buffer[..jump];
    let mut content = &buffer[jump..].iter().map(|&x| !x).collect::<Vec<_>>();

    let mut new_image = Vec::new();
    new_image.append(&mut header.to_vec());
    new_image.append(&mut content.to_vec());

    //Write new image
    let mut fp = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open("aww.bmp")
            .unwrap();
    
    fp.write_all(&new_image);
}