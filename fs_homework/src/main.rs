use std::fs;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Bitmap<'a> {
    body: &'a [u8],
    header: &'a [u8],
    content: Vec<u8>,
    name: &'a str
}

impl<'a> Bitmap<'a> {
    const content_offset: usize = 10;

    fn new(name: &'a str, content: &'a Vec<u8>) -> Self {
        Bitmap {
            header: &content[..14],
            body: &content[14..],
            content: Vec::new(),
            name: name
        }
    }

    fn apply(self, filter: Box<dyn Filter>) -> Self {
        let mut offset = self.header[Bitmap::content_offset] as usize;
        offset = offset - 14;
        let mut meta = self.body[..offset].to_vec();
        let mut translated = filter.apply(&self.body[offset..]);
        let mut result = self.header.to_vec();
        result.append(&mut meta);
        result.append(&mut translated);
        
        Bitmap {
            header: self.header,
            body: self.body,
            content: result,
            name: self.name
        }
    }
}

struct InverseColorFilter {}

struct FlipFilter {}

trait Filter {
    fn apply(&self, data: &[u8]) -> Vec<u8>;
}

impl Filter for InverseColorFilter {
    fn apply(&self, data: &[u8]) -> Vec<u8> {
        data
            .iter()
            .map(|&x| !x)
            .collect::<Vec<_>>()
    }
}

impl Filter for FlipFilter {
    fn apply(&self, data: &[u8]) -> Vec<u8> {
        let mut result = Vec::new();
        for i in 0..data.len()/3 {
            let start = (i * 3);
            result.push((data[start], data[start + 1], data[start + 2]));
        }

        result
            .iter()
            .rev()
            .flat_map(|&(a, b, c)| vec![a, b, c])
            .collect::<Vec<_>>()
    }
}

fn main() {
    let fs = fs::OpenOptions::new();
    let content = &fs::read("bird.bmp").unwrap();
    let bmp = Bitmap::new("002", &content)
            .apply(Box::new(InverseColorFilter{}))
            .apply(Box::new(FlipFilter{}));

    let mut fp = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}.bmp", bmp.name))
        .unwrap();

    fp.write_all(&bmp.content);
}