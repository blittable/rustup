use std::ops::Add;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::console::log;
use web_sys::{console, CanvasRenderingContext2d, ImageData};


#[wasm_bindgen]
pub fn transform_bird(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
) -> Result<(), JsValue> {

    let mut data = render(width, height);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn render(width: u32, height: u32) -> Vec<u8> {
    let mut data = Vec::new();

    data
}


