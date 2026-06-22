use std::{fs::read, path::Path};

#[repr(C)]
#[derive(Debug)]
pub struct BmpImage {
    name: String,
    sign: [char; 2],
    file_size: u32,
    reserved1: u16,
    reserved2: u16,
    file_offset: u32,
    dib_head_size: u32,
    pub img_width: i32,
    pub img_height: i32,
    plane: u16,
    bits_per_pixel: u16,
    compression: u32,
    img_size: u32,
    x_px_per_meter: u32,
    y_px_per_meter: u32,
    colors_in_color_table: u32,
    important_color_count: u32,
    pub img_pixels: Vec<u8>,
}

pub fn image_loader(path: &Path) -> BmpImage {
    if !path.exists() {
        eprintln!("Error: {} not found", path.to_str().unwrap_or_default());
    }
    println!("Loading {}", path.display());
    let img = read(path.to_str().unwrap_or_default()).expect("Failed to open image");
    let img_name = path.file_name().unwrap_or_default();
    let img_data: BmpImage = format_data(&img, img_name.to_str().unwrap_or_default());
    img_data
    // dbg!(img_data.name);
}

fn format_data(content: &Vec<u8>, img_name: &str) -> BmpImage {
    let file_offset = u32::from_ne_bytes(content[10..14].try_into().unwrap());
    let img_size = u32::from_ne_bytes(content[34..38].try_into().unwrap());
    let header: BmpImage = BmpImage {
        name: img_name.to_string(),
        sign: [content[0] as char, content[1] as char],
        file_size: u32::from_ne_bytes(content[2..6].try_into().unwrap()),
        reserved1: u16::from_ne_bytes(content[6..8].try_into().unwrap()),
        reserved2: u16::from_ne_bytes(content[8..10].try_into().unwrap()),
        file_offset,
        dib_head_size: u32::from_ne_bytes(content[14..18].try_into().unwrap()),
        img_width: i32::from_ne_bytes(content[18..22].try_into().unwrap()),
        img_height: i32::from_ne_bytes(content[22..26].try_into().unwrap()),
        plane: u16::from_ne_bytes(content[26..28].try_into().unwrap()),
        bits_per_pixel: u16::from_ne_bytes(content[28..30].try_into().unwrap()),
        compression: u32::from_ne_bytes(content[30..34].try_into().unwrap()),
        img_size,
        x_px_per_meter: u32::from_ne_bytes(content[38..42].try_into().unwrap()),
        y_px_per_meter: u32::from_ne_bytes(content[42..46].try_into().unwrap()),
        colors_in_color_table: u32::from_ne_bytes(content[46..50].try_into().unwrap()),
        important_color_count: u32::from_ne_bytes(content[50..54].try_into().unwrap()),
        img_pixels: content[(file_offset as usize)..((file_offset + img_size) as usize)].to_vec(),
    };
    header
}
