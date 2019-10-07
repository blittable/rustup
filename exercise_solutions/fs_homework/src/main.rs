#![allow(dead_code)]
#![allow(warnings)]
use std::fs;
use std::io;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};
use array2d::Array2D;

#[derive(Debug)]
struct Bitmap<'a> {
    raw: Vec<u8>,
    content: Vec<Vec<(u8, u8, u8)>>,
    seek: usize,
    name: &'a str
}

impl<'a> Bitmap<'a> {
    const content_offset: usize = 10;
    const weight_offset: usize = 18;
    const height_offset: usize = 22;

    fn new(name: &'a str, content: Vec<u8>) -> Self {
        let weight: i16 = bincode::deserialize(&content[Bitmap::weight_offset..Bitmap::weight_offset+3]).unwrap();
        let height: i16 = bincode::deserialize(&content[Bitmap::height_offset..Bitmap::height_offset+3]).unwrap();

        let offset: i16 = bincode::deserialize(&content[Bitmap::content_offset..Bitmap::content_offset+3]).unwrap();
        let seek: usize = offset as usize;
        let mut data = content[seek..].to_vec();

        let mut rows = Vec::new();
        let mut columns = Vec::new();
        for i in 0..data.len()/3 {
            let start = (i * 3);
            let len = columns.len() as i16;
            if len == weight
            {
                rows.push(columns);
                columns = Vec::new();
            }
            columns.push((data[start], data[start + 1], data[start + 2]));
        }
        rows.push(columns);

        Bitmap {
            raw: content,
            content: rows,
            seek: seek,
            name: name
        }
    }

    fn apply(self, filter: Box<dyn Filter>) -> Self {
        let mut meta = self.raw[..self.seek].to_vec();
        let mut translated = filter.apply(self.content)
                                    .iter()
                                    .flat_map(|&(b, g, r)| vec![b, g, r])
                                    .collect::<Vec<_>>();
        let mut result = Vec::new();
        
        result.append(&mut meta);
        result.append(&mut translated);

        Self::new(self.name, result)
    }
}

trait Filter {
    fn apply(&self, data: Vec<Vec<(u8, u8, u8)>>) -> Vec<(u8, u8, u8)>;
}

struct InverseColorFilter {}
impl Filter for InverseColorFilter {
    fn apply(&self, data: Vec<Vec<(u8, u8, u8)>>) -> Vec<(u8, u8, u8)> {
        data
            .iter()
            .flat_map(|array| array.iter())
            .map(|&(b, g, r)| (!b, !g, !r))
            .collect::<Vec<_>>()
    }
}

enum Flip {
    Holizontal,
    Vertical
}
struct FlipFilter {
    flip: Flip
}
impl Filter for FlipFilter {
    fn apply(&self, data: Vec<Vec<(u8, u8, u8)>>) -> Vec<(u8, u8, u8)> {
        match self.flip {
            Flip::Vertical => data
                .iter()
                .rev()
                .flat_map(|array| array.iter())
                .map(|&t| t)
                .collect::<Vec<_>>(),
            _ => data
                .iter()
                .flat_map(|array| array.iter().rev())
                .map(|&t| t)
                .collect::<Vec<_>>()
        }
    }
}

fn main() {
    let fs = fs::OpenOptions::new();
    let content = &fs::read("bird.bmp").unwrap();
    let bmp = Bitmap::new("002", content.to_vec())
            .apply(Box::new(InverseColorFilter{}))
            .apply(Box::new(FlipFilter{
                flip: Flip::Holizontal
            }))
            .apply(Box::new(FlipFilter{
                flip: Flip::Vertical
            }));

    let mut fp = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(format!("{}.bmp", bmp.name))
        .unwrap();

    fp.write_all(&bmp.raw);
}