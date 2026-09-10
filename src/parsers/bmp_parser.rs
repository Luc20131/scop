use std::{
    fs::File,
    io::{Error, Read, Seek, SeekFrom},
    path::Path,
};

use gl::TEXTURE_ALPHA_TYPE;

#[derive(Debug)]
pub enum BmpError {
    Io(Error),
    InvalidSignature([u8; 2]),
    UnsupportedDibHeaderSize(u32),
    UnsupportedBpp(u16),
    InvalidColorIndex(),
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
    pub height: i32,
    plane: u16,
    bits_per_pixel: u16,
    compression: u32,
    pub img_size: u32,
    x_px_per_meter: u32,
    y_px_per_meter: u32,
    colors_in_color_table: u32,
    important_color_count: u32,
    red_channel_bitmask: u32,
    green_channel_bitmask: u32,
    blue_channel_bitmask: u32,
    alpha_channel_bitmask: u32,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(unused)]
#[repr(C)]
pub struct BGRA {
    blue: u8,
    green: u8,
    red: u8,
    alpha: u8,
}

pub type Pixel = BGRA;

impl Default for Pixel {
    fn default() -> Self {
        Self {
            blue: 0,
            green: 0,
            red: 0,
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

    pub fn height(&mut self) -> i32 {
        self.dib_header.height
    }
}

pub fn image_loader(path: &Path, flag: u32) -> Result<BmpImage, BmpError> {
    if !path.exists() {
        eprintln!("Error: {} not found", path.to_str().unwrap_or_default());
    }
    println!("Loading {}", path.display());
    let file = File::open(path)?;
    let header = parse_header(&file)?;

    let dib_header: DibHeader = parse_dib(&file)?;
    dbg!(&dib_header);
    let mut colors_list: Vec<Pixel> = vec![];
    let mut img_pixels: Vec<BGRA> = vec![];
    if dib_header.bits_per_pixel < 16 {
        if dib_header.colors_in_color_table != 0 {
            colors_list = parse_color_table(&file, &dib_header, flag)?;
        }
        img_pixels =
            parse_px_from_color_table(&file, colors_list, &dib_header, header.data_offset, flag)?;
    } else {
        img_pixels = parse_pixel(&file, header.data_offset, &dib_header)?;
    }
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
    let mut buffer = [0u8; 124];

    file.read_exact(&mut buffer)?;

    let dib_header_size = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
    if dib_header_size != 124 {
        return Err(BmpError::UnsupportedDibHeaderSize(dib_header_size));
    }
    let dib = DibHeader {
        dib_head_size: dib_header_size,
        width: u32::from_le_bytes(buffer[4..8].try_into().unwrap()),
        height: i32::from_le_bytes(buffer[8..12].try_into().unwrap()),
        plane: u16::from_le_bytes(buffer[12..14].try_into().unwrap()),
        bits_per_pixel: u16::from_le_bytes(buffer[14..16].try_into().unwrap()),
        compression: u32::from_le_bytes(buffer[16..20].try_into().unwrap()),
        img_size: u32::from_le_bytes(buffer[20..24].try_into().unwrap()),
        x_px_per_meter: u32::from_le_bytes(buffer[24..28].try_into().unwrap()),
        y_px_per_meter: u32::from_le_bytes(buffer[28..32].try_into().unwrap()),
        colors_in_color_table: u32::from_le_bytes(buffer[32..36].try_into().unwrap()),
        important_color_count: u32::from_le_bytes(buffer[36..40].try_into().unwrap()),
        red_channel_bitmask: u32::from_le_bytes(buffer[40..44].try_into().unwrap()),
        green_channel_bitmask: u32::from_le_bytes(buffer[44..48].try_into().unwrap()),
        blue_channel_bitmask: u32::from_le_bytes(buffer[48..52].try_into().unwrap()),
        alpha_channel_bitmask: u32::from_le_bytes(buffer[56..60].try_into().unwrap()),
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
    let height: i32 = (dib_header.height as i32).abs();
    let bytes_per_px: usize = dib_header.bits_per_pixel as usize / 8;
    let row_size: usize = (width as usize * dib_header.bits_per_pixel as usize + 31) / 32 * 4;

    let is_mirrored = dib_header.height > 0;

    let mut row_buf = vec![0u8; row_size];
    let mut pixels: Vec<Pixel> = vec![Pixel::default(); (width as usize) * (height as usize)];
    for row in 0..height as usize {
        file.read_exact(&mut row_buf)?;
        let dst_row = if is_mirrored {
            height as usize - 1 - row
        } else {
            row
        };
        for col in 0..width as usize {
            let px_offset: usize = col * bytes_per_px;
            let alpha;
            if bytes_per_px <= 3 {
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
            pixels[dst_row * (width as usize) + col] = px;
        }
    }
    Ok(pixels)
}

fn parse_color_table(
    mut file: &File,
    dib_header: &DibHeader,
    flag: u32,
) -> Result<Vec<BGRA>, BmpError> {
    let mut buf: Vec<u8> = vec![0u8; (dib_header.colors_in_color_table * 4) as usize];
    file.seek(SeekFrom::Start(14u64 + dib_header.dib_head_size as u64))?;
    file.read_exact(&mut buf)?;
    let mut color_table: Vec<BGRA> = vec![];

    for index in 0..((dib_header.colors_in_color_table) as usize) {
        let mut color = u32_to_bgra(u32::from_le_bytes(
            buf[index * 4..(index * 4 + 4)].try_into().unwrap(),
        ));
        if flag == TEXTURE_ALPHA_TYPE {
            color.alpha = color.red;
        }
        color_table.push(color);
    }
    Ok(color_table)
}

fn parse_px_from_color_table(
    mut file: &File,
    color_table: Vec<BGRA>,
    dib_header: &DibHeader,
    offset: u32,
    flag: u32,
) -> Result<Vec<BGRA>, BmpError> {
    file.seek(SeekFrom::Start(offset as u64))?;
    let width = dib_header.width;
    let height: i32 = (dib_header.height as i32).abs();
    let row_size: usize = (width as usize * dib_header.bits_per_pixel as usize + 31) / 32 * 4;
    let is_mirrored = dib_header.height > 0;
    let mut row_buf = vec![0u8; row_size];
    let mut pixels: Vec<BGRA> = vec![BGRA::default(); (width as usize) * (height as usize)];

    for row in 0..height as usize {
        file.read_exact(&mut row_buf)?;
        let dst_row = if is_mirrored {
            height as usize - 1 - row
        } else {
            row
        };

        for col in 0..width as usize {
            let color_index: usize = (row_buf[col]) as usize;
            let pixel_index = dst_row * (width as usize) + col;
            pixels[pixel_index] = color_table
                .get(color_index)
                .ok_or(BmpError::InvalidColorIndex())?
                .clone();
            if flag == TEXTURE_ALPHA_TYPE {
                pixels[pixel_index].alpha = pixels[pixel_index].red;
            }
        }
    }

    Ok(pixels)
}

fn u32_to_bgra(value: u32) -> BGRA {
    let oui = value.to_le_bytes();
    let color = BGRA {
        blue: oui[3],
        green: oui[2],
        red: oui[1],
        alpha: 255,
    };

    color
}
