use std::{ffi::c_char, io::Read, os::unix::fs::FileExt};

const SIZE_FILE_OFFSET: u64 = 2;
const RESERVED1_OFFSET: u64 = 6;
const RESERVED2_OFFSET: u64 = 8;
const FILE_OFFSET_OFFSET: u64 = 10;

#[repr(C)]
#[derive(Debug)]
struct BmpHeader {
    sign: [c_char; 2],
    file_size: u32,
    reserved1: u16,
    reserved2: u16,
    file_offset: u32,
    // dib_head_size: u32,
    // img_w: u32,
    // img_h: u32,
}

impl BmpHeader {
    pub fn new() -> Self {
        BmpHeader {
            sign: [0i8; 2],
            file_size: 0,
            reserved1: 0,
            reserved2: 0,
            file_offset: 0,
        }
    }
}

pub fn image_loader(path: &str) {
    let mut buff: [u8; 2] = [0u8; 2];
    let mut file = std::fs::File::open(path).expect("Failed to open image");
    let mut header: BmpHeader = BmpHeader::new();
    // file.read_exact(&mut buff);
    file.read_exact(&mut buff)
        .expect("Error: Failed to read image");
    header.sign = unsafe { std::mem::transmute(buff) };
    // println!("{:?}", buff);
    let mut buff: [u8; 4] = [0u8; 4];
    file.read_exact_at(&mut buff, SIZE_FILE_OFFSET)
        .expect("Error: Failed to read image");
    header.file_size = u32::from_ne_bytes(buff);
    let mut buff: [u8; 2] = [0u8; 2];
    file.read_exact_at(&mut buff, RESERVED1_OFFSET)
        .expect("Error: Failed to read image");
    header.reserved1 = u16::from_ne_bytes(buff);
    file.read_exact_at(&mut buff, RESERVED2_OFFSET)
        .expect("Error: Failed to read image");
    header.reserved2 = u16::from_ne_bytes(buff);
    let mut buff: [u8; 4] = [0u8; 4];
    file.read_exact_at(&mut buff, FILE_OFFSET_OFFSET)
        .expect("Error: Failed to read image");
    header.file_offset = u32::from_ne_bytes(buff);
    println!("{:?}", header);
    // header = extract_header(buff);
    // let img = read(path).expect("Failed to open image");
}

fn extract_header(content: [u8; 54]) -> BmpHeader {
    let mut buff: [u8; 16] = [0u8; 16];
    for i in 0..14 {
        buff[i] = content[i];
    }
    // let mut header: BmpHeader = BmpHeader {
    // u16::from_le_bytes(content[0..2]);
    //     file_size: u32::from_le_bytes(content[1..6].try_into().unwrap()),
    //     reserved1: u16::from_le_bytes(content[6..8].try_into().unwrap()),
    //     reserved2: u16::from_le_bytes(content[8..10].try_into().unwrap()),
    //     file_offset: u32::from_le_bytes(content[10..14].try_into().unwrap()),
    // };
    let header = unsafe { std::mem::transmute(buff) };
    println!("{:?}", header);
    // file.read_exact(&mut buff).expect("Failed to read header");
    header
}
