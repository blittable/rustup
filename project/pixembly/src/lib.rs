use std::f64;
use std::fs;
use std::io;
use std::io::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use base64::{encode, decode};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub struct Bitmap<'a> {
    raw: Vec<u8>,
    content: Vec<Vec<(u8, u8, u8)>>,
    seek: usize,
    name: &'a str
}

impl<'a> Bitmap<'a> {
    const CONTENT_OFFSET: usize = 10;
    const WEIGHT_OFFSET: usize = 18;
    const HEIGHT_OFFSET: usize = 22;

    fn new(name: &'a str, content: Vec<u8>) -> Self {
        let weight: i16 = bincode::deserialize(&content[Bitmap::WEIGHT_OFFSET..Bitmap::WEIGHT_OFFSET+3]).unwrap();
        let height: i16 = bincode::deserialize(&content[Bitmap::HEIGHT_OFFSET..Bitmap::HEIGHT_OFFSET+3]).unwrap();

        let offset: i16 = bincode::deserialize(&content[Bitmap::CONTENT_OFFSET..Bitmap::CONTENT_OFFSET+3]).unwrap();
        let seek: usize = offset as usize;
        let data = content[seek..].to_vec();

        let mut rows = Vec::new();
        let mut columns = Vec::new();
        for i in 0..data.len()/3 {
            let start = i * 3;
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

pub trait Filter {
    fn apply(&self, data: Vec<Vec<(u8, u8, u8)>>) -> Vec<(u8, u8, u8)>;
}

pub struct InverseColorFilter {}
impl Filter for InverseColorFilter {
    fn apply(&self, data: Vec<Vec<(u8, u8, u8)>>) -> Vec<(u8, u8, u8)> {
        data
            .iter()
            .flat_map(|array| array.iter())
            .map(|&(b, g, r)| (!b, !g, !r))
            .collect::<Vec<_>>()
    }
}

pub enum Flip {
    Holizontal,
    Vertical
}
pub struct FlipFilter {
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

#[macro_use]
extern crate serde_derive;

#[wasm_bindgen]
pub fn load_image(id: &str, base64: String, filter: &str) {
    let data: Vec<u8> = decode(&base64).unwrap();
    let mut bmp = Bitmap::new("003", data);
    
    bmp = match filter {
        "InverseColorFilter" => bmp.apply(Box::new(InverseColorFilter{})),
        "FlipFilterHolizontal" => bmp.apply(Box::new(FlipFilter{
            flip: Flip::Holizontal
        })),
        "FlipFilterVertical" => bmp.apply(Box::new(FlipFilter{
            flip: Flip::Vertical
        })),
        _ => bmp
    };

    draw(id, bmp);
}

pub fn draw(id: &str, bmp: Bitmap) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(id).unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    
    let pixels = bmp.content;
    log!("pixels len: {:?}", pixels);

    let height = pixels.len();
    let weight = pixels[0].len();
    context.clear_rect(0.0, 0.0, weight as f64, height as f64);
    for y in 0..height {
        for x in 0..weight {
            let pixel = pixels[y][x];
            context.set_fill_style(&JsValue::from_str(&format!("rgb({}, {}, {})", pixel.2, pixel.1, pixel.0)));
            context.fill_rect(x as f64, (height - y - 1) as f64, 1.0, 1.0);
        }
    }
}