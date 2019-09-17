#![feature(seek_convenience)]
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{IoSliceMut, SeekFrom};

#[derive(Serialize, Copy, Clone, Deserialize)]
struct BmpHeader {
    MagicNumber: u16,
    FileSize: u32,
    Reserved: u32,
    StartAddress: u32,
}

struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}


fn debug_bmp_header(header: BmpHeader) {

    println!("");
    println!("Header int: {:?} ", header.MagicNumber);
    println!("FileSize int: {:?} ", header.FileSize);
    println!("Reserved int: {:?} ", header.Reserved);
    println!("StartAddress int : {:?} ", header.StartAddress);
    println!("");
    println!("Header hex: {:x?} ", header.MagicNumber);
    println!("FileSize hex: {:x?} ", header.FileSize);
    println!("Reserved hex: {:x?} ", header.Reserved);
    println!("StartAddress hex: {:x?} ", header.StartAddress);
    println!("");

}


fn main() -> io::Result<()> {
    
    const HEAD_LENGTH: usize = 14;

    let mut f = &File::open("sample/bird.bmp")?;
    let mut header_buffer = [0; HEAD_LENGTH];
    f.read(&mut header_buffer)?;

    let header: BmpHeader = bincode::deserialize(&header_buffer[..]).unwrap();
    debug_bmp_header(header);

    //There are 40 bytes between the header and the start of the r,g,b data
    //Offset before mutation to avoid another buffer and to make it easy 
    //to piece the two pieces back together.  Possibly handled with a split. 

    let mut data_buffer: Vec<u8> = Vec::new();
    println!("File POS: {:?} ", f.stream_position()?);
    f.read_to_end(&mut data_buffer)?;

    let mut io_slice = IoSliceMut::new(&mut data_buffer);
    let slice_length = io_slice.len();

    let mut counter = 0;

    //Chunk based iteration wrapper for future chaos
    for chunk in io_slice.chunks_mut(1) {
        for elem in chunk.iter_mut() {   
            if counter >= header.StartAddress as i64 && counter < slice_length as i64 {
                *elem = *elem << 2;
            }
            counter = counter + 1;
        }
    }

    let mut transformed_file = File::create("filtered_bitmap.bmp")?;

    match transformed_file.write_all(&bincode::serialize(&header).unwrap()) {
        Ok(o) => println!("File Written"),
        Err(e) => println!("Error writing file {:?}", e),
    };

    transformed_file.write_all(&data_buffer).unwrap();

    Ok(())
}
