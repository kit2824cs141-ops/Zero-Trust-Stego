use wasm_bindgen::prelude::*;
use image::{load_from_memory, ImageOutputFormat, RgbaImage};
use std::io::Cursor;

#[wasm_bindgen]
pub fn encode(image_data: &[u8], secret_message: &str) -> Result<Vec<u8>, JsValue> {
    let mut img: RgbaImage = load_from_memory(image_data)
        .map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?
        .to_rgba8();

    let mut message_bytes = secret_message.as_bytes().to_vec();
    message_bytes.push(0); 

    let mut bits = message_bytes.into_iter().flat_map(|b| {
        (0..8).map(move |i| (b >> (7 - i)) & 1)
    });

    for pixel in img.pixels_mut() {
        for channel in 0..3 { 
            if let Some(bit) = bits.next() {
                pixel[channel] = (pixel[channel] & 0xFE) | bit;
            }
        }
    }

    let mut output = Cursor::new(Vec::new());
    img.write_to(&mut output, ImageOutputFormat::Png)
        .map_err(|e| JsValue::from_str(&format!("Failed to encode to PNG: {}", e)))?;

    Ok(output.into_inner())
}

#[wasm_bindgen]
pub fn decode(image_data: &[u8]) -> Result<String, JsValue> {
    let img: RgbaImage = load_from_memory(image_data)
        .map_err(|e| JsValue::from_str(&format!("Failed to load image: {}", e)))?
        .to_rgba8();

    let mut current_byte = 0u8;
    let mut bit_count = 0;
    let mut message_bytes = Vec::new();

    'outer: for pixel in img.pixels() {
        for channel in 0..3 {
            let bit = pixel[channel] & 1;
            current_byte = (current_byte << 1) | bit;
            bit_count += 1;

            if bit_count == 8 {
                if current_byte == 0 {
                    break 'outer;
                }
                
                message_bytes.push(current_byte);
                current_byte = 0;
                bit_count = 0;
            }
        }
    }

    String::from_utf8(message_bytes)
        .map_err(|e| JsValue::from_str(&format!("Decoded bytes are not valid UTF-8: {}", e)))
}
