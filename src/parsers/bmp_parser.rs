use std::{
    fs::File,
    io::{Error, Read, Seek, SeekFrom},
    path::Path,
};

use crate::parsers::mtl_parser::RGB;

pub enum BmpError {
    Io(Error),
    InvalidSignature([u8; 2]),
    UnsupportedDibHeaderSize(u32),
    UnsupportedBpp(u16),
}

impl From<Error> for BmpError {
    fn from(err: Error) -> Self {
        BmpError::Io(err)
    }
}

#[repr(C)]
#[derive(Debug)]
struct BmpHeader {
    sign: [u8; 2],
    file_size: u32,
    reserved: u32,
    data_offset: u32,
}

#[repr(C)]
#[derive(Debug)]
struct DibHeader {
    dib_head_size: u32,
    pub width: u32,
    pub height: u32,
    plane: u16,
    bits_per_pixel: u16,
    compression: u32,
    pub img_size: u32,
    x_px_per_meter: u32,
    y_px_per_meter: u32,
    colors_in_color_table: u32,
    important_color_count: u32,
}

type Pixel = RGB;

#[repr(C)]
#[derive(Debug)]
pub struct BmpImage {
    name: String,
    header: BmpHeader,
    dib_header: DibHeader,
    pub img_pixels: Vec<Pixel>,
}

pub fn image_loader(path: &Path) -> Result<BmpImage, BmpError> {
    if !path.exists() {
        eprintln!("Error: {} not found", path.to_str().unwrap_or_default());
    }
    println!("Loading {}", path.display());
    let file = File::open(path)?;
    let header = parse_header(&file)?;

    let dib_header: DibHeader = parse_dib(&file)?;
    if dib_header.bits_per_pixel != 24 {
        return Err(BmpError::UnsupportedBpp(dib_header.bits_per_pixel));
    }
    // let img = read(path.to_str().unwrap_or_default()).expect("Failed to open image");
    // let img_name = path.file_name().unwrap_or_default();
    let img_pixels: Vec<Pixel> = parse_pixel(
        &file,
        header.data_offset,
        dib_header.width,
        dib_header.height,
    )?;
    let image = BmpImage {
        name: path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("Undefined")
            .to_string(),
        header,
        dib_header,
        img_pixels,
    };
    Ok(image)
    // dbg!(img_data.name);
}

fn parse_header(mut file: &File) -> Result<BmpHeader, BmpError> {
    let mut buffer = [0u8; 14];
    file.read_exact(&mut buffer)?;

    let signature = [buffer[0], buffer[1]];
    if signature != [b'B', b'M'] {
        return Err(BmpError::InvalidSignature(signature));
    }
    let header = BmpHeader {
        sign: signature,
        file_size: u32::from_le_bytes(buffer[2..6].try_into().unwrap()),
        reserved: u32::from_le_bytes(buffer[6..10].try_into().unwrap()),
        data_offset: u32::from_le_bytes(buffer[10..14].try_into().unwrap()),
    };
    Ok(header)
}

fn parse_dib(mut file: &File) -> Result<DibHeader, BmpError> {
    let mut buffer = [0u8; 40];
    file.read_exact(&mut buffer)?;

    let dib_header_size = u32::from_ne_bytes(buffer[0..4].try_into().unwrap());
    if dib_header_size != 40 {
        return Err(BmpError::UnsupportedDibHeaderSize(dib_header_size));
    }
    let dib = DibHeader {
        dib_head_size: dib_header_size,
        width: u32::from_ne_bytes(buffer[4..8].try_into().unwrap()),
        height: u32::from_ne_bytes(buffer[8..12].try_into().unwrap()),
        plane: u16::from_ne_bytes(buffer[12..14].try_into().unwrap()),
        bits_per_pixel: u16::from_ne_bytes(buffer[14..16].try_into().unwrap()),
        compression: u32::from_ne_bytes(buffer[16..20].try_into().unwrap()),
        img_size: u32::from_ne_bytes(buffer[20..24].try_into().unwrap()),
        x_px_per_meter: u32::from_ne_bytes(buffer[24..28].try_into().unwrap()),
        y_px_per_meter: u32::from_ne_bytes(buffer[28..32].try_into().unwrap()),
        colors_in_color_table: u32::from_ne_bytes(buffer[32..36].try_into().unwrap()),
        important_color_count: u32::from_ne_bytes(buffer[36..40].try_into().unwrap()),
    };
    Ok(dib)
}

fn parse_pixel(
    mut file: &File,
    offset: u32,
    width: u32,
    height: u32,
) -> Result<Vec<Pixel>, BmpError> {
    file.seek(SeekFrom::Start(offset as u64))?;

    // let height =
    let row_size = ((width * 3 + 3) / 4) * 4;
    let pixels: Vec<Pixel> = vec![];
    Ok(pixels)
}

// img_pixels: content[(file_offset as usize)..((file_offset + img_size) as usize)].to_vec(),
