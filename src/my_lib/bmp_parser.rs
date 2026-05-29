use std::{
    fs::read,
    path::Path,
    process::{ExitStatus, exit},
};

use crate::graphics::texture;

#[repr(C)]
#[derive(Debug)]
struct BmpImage<'a> {
    name: &'a str,
    sign: [char; 2],
    file_size: u32,
    reserved1: u16,
    reserved2: u16,
    file_offset: u32,
    dib_head_size: u32,
    img_w: u32,
    img_h: u32,
    plane: u16,
    bits_per_pixel: u16,
    compression: u32,
    img_size: u32,
    x_px_per_meter: u32,
    y_px_per_meter: u32,
    colors_in_color_table: u32,
    important_color_count: u32,
    img_pixels: &'a [u8],
}

pub fn image_loader(path: &Path) {
    if !path.exists() {
        eprintln!("Error: {} not found", path.to_str().unwrap_or_default());
    }
    dbg!("Loading {}", path.to_str());
    let img = read(path.to_str().unwrap_or_default()).expect("Failed to open image");
    let img_name = path.file_name().unwrap_or_default();
    let img_data = format_data(&img, img_name.to_str().unwrap_or_default());

    dbg!(img_data.name);
}

fn format_data<'a>(content: &'a Vec<u8>, img_name: &'a str) -> BmpImage<'a> {
    let file_offset = u32::from_ne_bytes(content[10..14].try_into().unwrap());
    let img_size = u32::from_ne_bytes(content[34..38].try_into().unwrap());
    let header: BmpImage<'a> = BmpImage::<'a> {
        name: img_name,
        sign: [content[0] as char, content[1] as char],
        file_size: u32::from_ne_bytes(content[2..6].try_into().unwrap()),
        reserved1: u16::from_ne_bytes(content[6..8].try_into().unwrap()),
        reserved2: u16::from_ne_bytes(content[8..10].try_into().unwrap()),
        file_offset: file_offset,
        dib_head_size: u32::from_ne_bytes(content[14..18].try_into().unwrap()),
        img_w: u32::from_ne_bytes(content[18..22].try_into().unwrap()),
        img_h: u32::from_ne_bytes(content[22..26].try_into().unwrap()),
        plane: u16::from_ne_bytes(content[26..28].try_into().unwrap()),
        bits_per_pixel: u16::from_ne_bytes(content[28..30].try_into().unwrap()),
        compression: u32::from_ne_bytes(content[30..34].try_into().unwrap()),
        img_size: img_size,
        x_px_per_meter: u32::from_ne_bytes(content[38..42].try_into().unwrap()),
        y_px_per_meter: u32::from_ne_bytes(content[42..46].try_into().unwrap()),
        colors_in_color_table: u32::from_ne_bytes(content[46..50].try_into().unwrap()),
        important_color_count: u32::from_ne_bytes(content[50..54].try_into().unwrap()),
        img_pixels: content[(file_offset as usize)..((file_offset + img_size) as usize)]
            .iter()
            .as_slice(),
    };
    header
}
