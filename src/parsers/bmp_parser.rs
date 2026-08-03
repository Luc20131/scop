use std::{
    fs::File,
    io::{Error, Read, Seek, SeekFrom},
    path::Path,
};

use gl::PACK_IMAGE_HEIGHT;

#[derive(Debug)]
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

#[derive(Debug, Clone)]
pub struct RGBA {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

pub type Pixel = RGBA;

impl Default for Pixel {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct BmpImage {
    name: String,
    header: BmpHeader,
    dib_header: DibHeader,
    pub img_pixels: Vec<Pixel>,
}

impl BmpImage {
    pub fn width(&mut self) -> u32 {
        self.dib_header.width
    }

    pub fn height(&mut self) -> u32 {
        self.dib_header.height
    }
}

pub fn image_loader(path: &Path) -> Result<BmpImage, BmpError> {
    if !path.exists() {
        eprintln!("Error: {} not found", path.to_str().unwrap_or_default());
    }
    println!("Loading {}", path.display());
    let file = File::open(path)?;
    println!("Parse header");
    let header = parse_header(&file)?;
    println!("Parse dib header, data_offset: {}", header.data_offset);

    let dib_header: DibHeader = parse_dib(&file)?;
    if dib_header.bits_per_pixel != 24 {
        return Err(BmpError::UnsupportedBpp(dib_header.bits_per_pixel));
    }
    // let img = read(path.to_str().unwrap_or_default()).expect("Failed to open image");
    // let img_name = path.file_name().unwrap_or_default();
    println!(
        "Parse pixels \n\theight : {}\n\twidth : {}",
        dib_header.height, dib_header.width
    );

    let img_pixels: Vec<Pixel> = parse_pixel(&file, header.data_offset, &dib_header)?;
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
    dbg!(buffer);
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
    let mut buffer = [0u8; 124];

    file.read_exact(&mut buffer)?;

    let dib_header_size = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
    if dib_header_size != 124 {
        return Err(BmpError::UnsupportedDibHeaderSize(dib_header_size));
    }
    let dib = DibHeader {
        dib_head_size: dib_header_size,
        width: u32::from_le_bytes(buffer[4..8].try_into().unwrap()),
        height: u32::from_le_bytes(buffer[8..12].try_into().unwrap()),
        plane: u16::from_le_bytes(buffer[12..14].try_into().unwrap()),
        bits_per_pixel: u16::from_le_bytes(buffer[14..16].try_into().unwrap()),
        compression: u32::from_le_bytes(buffer[16..20].try_into().unwrap()),
        img_size: u32::from_le_bytes(buffer[20..24].try_into().unwrap()),
        x_px_per_meter: u32::from_le_bytes(buffer[24..28].try_into().unwrap()),
        y_px_per_meter: u32::from_le_bytes(buffer[28..32].try_into().unwrap()),
        colors_in_color_table: u32::from_le_bytes(buffer[32..36].try_into().unwrap()),
        important_color_count: u32::from_le_bytes(buffer[36..40].try_into().unwrap()),
    };
    Ok(dib)
}

fn parse_pixel(
    mut file: &File,
    offset: u32,
    dib_header: &DibHeader,
) -> Result<Vec<Pixel>, BmpError> {
    file.seek(SeekFrom::Start(offset as u64))?;
    let width = dib_header.width;
    let height = dib_header.height;
    let byte_per_px: usize = dib_header.bits_per_pixel as usize / 8;
    let row_size: usize = width as usize * byte_per_px + 4;

    dbg!(row_size);
    let mut row_buf = vec![0u8; row_size];
    let mut pixels: Vec<Pixel> = vec![Pixel::default(); (width as usize) * (height as usize)];
    dbg!(pixels.len());
    for row in 0..height as usize {
        dbg!(row);
        file.read_exact(&mut row_buf)?;
        for col in 0..width as usize {
            let px_offset: usize = col * byte_per_px;
            let alpha;
            if byte_per_px <= 3 {
                alpha = 255;
            } else {
                alpha = row_buf[px_offset + 3];
            }
            let px = Pixel {
                blue: row_buf[px_offset],
                green: row_buf[px_offset + 1],
                red: row_buf[px_offset + 2],
                alpha: alpha,
            };
            pixels[row * (width as usize) + col] = px;
        }
    }
    Ok(pixels)
}

// img_pixels: content[(file_offset as usize)..((file_offset + img_size) as usize)].to_vec(),
