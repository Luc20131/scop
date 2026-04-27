use gl::types::GLfloat;
use std::{str::SplitWhitespace, u32};

pub struct Obj {
    pub name: String,
    pub v: Vec<GLfloat>,
    vt: Vec<Point>,
    vn: Vec<Point>,
    // vp: Vec<Point>,
    pub f: Vec<Face>,
    // l: Vec<String>,
}

#[derive(Debug)]
pub struct Face {
    pub v: [f32; 3],
    vn: Point,
    vt: [i32; 3],
}

// #[derive(Debug)]
// struct FaceData {
//     v: u32,
//     vt: u32,
//     vn: u32,
// }

#[derive(Debug)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Obj {
    pub fn new() -> Self {
        Self {
            name: "Undefined".to_string(),
            v: Vec::<GLfloat>::new(),
            vt: Vec::<Point>::new(),
            vn: Vec::<Point>::new(),
            // vp: Vec::<Point>::new(),
            f: Vec::<Face>::new(),
            // l: Vec::<String>::new(),
        }
    }

    pub fn print_data(&mut self) {
        println!("Name : {}", self.name);
        println!("Number of Vertices: {}", self.v.iter().size_hint().0 / 3);
        println!(
            "Number of Vertices Textures: {}",
            self.vt.iter().size_hint().0
        );
        println!(
            "Number of Vertices Normal: {}",
            self.vn.iter().size_hint().0
        );
        println!("Number of Faces: {}", self.f.iter().size_hint().0);
    }

    pub fn add_data(&mut self, line: &str) {
        let mut words: SplitWhitespace<'_> = line.split_whitespace();
        match words.next() {
            Some("o") => self.name = extract_name(line),
            Some("v") => self.v = extract_vertices(line, self.v.clone()),
            Some("vt") => self.vt.push(extract_texture(line)),
            Some("vn") => self.vn.push(extract_point(line)),
            Some("f") => extract_face(&mut self.f, line),
            Some(_) => {}
            None => {}
        }
    }

    // pub fn get_vertices_as_array(&self) {
    //     unsafe {
    //         let a: usize = { self.v.len() };
    //         let arr: [f32; 32];
    //         let mut buffer: [f32];
    //         arr.join();
    //     }
    // }
}

// pub fn vec_to_array(v: Vec<f32>) -> [T; N] {
//     v.try_into()
//         .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
// }

impl Clone for Obj {
    fn clone(&self) -> Self {
        todo!()
    }
}

pub fn extract_name(line: &str) -> String {
    let mut words: SplitWhitespace<'_> = line.split_whitespace();
    let ln_count: usize = words.clone().count();
    if ln_count >= 2 {
        return words.nth(1).expect("Name undefined").to_string();
    }
    "Undefined".to_string()
}

pub fn extract_point(line: &str) -> Point {
    let mut words: SplitWhitespace<'_> = line.split_whitespace();
    let mut point: Point = Point {
        x: (0.0),
        y: (0.0),
        z: (0.0),
    };
    let ln_count: usize = words.clone().count();
    if ln_count >= 4 {
        words.next();
        point.x = words
            .next()
            .expect("X coord Undefined")
            .parse::<f32>()
            .expect("X can't parse");
        point.y = words
            .next()
            .expect("Y coord Undefined")
            .parse::<f32>()
            .expect("Y can't parse");
        point.z = words
            .next()
            .expect("Z coord Undefined")
            .parse::<f32>()
            .expect("Z can't parse");
    }
    point
}

pub fn extract_vertices(line: &str, mut v: Vec<GLfloat>) -> Vec<GLfloat> {
    let words: SplitWhitespace<'_> = line.split_whitespace();
    for elem in words {
        if elem == "v" {
            continue;
        }
        v.push(elem.parse::<f32>().expect("Vertice can't be parsed"));
    }
    v
}

pub fn extract_texture(line: &str) -> Point {
    let mut words: SplitWhitespace<'_> = line.split_whitespace();
    let mut point: Point = Point {
        x: (0.0),
        y: (0.0),
        z: (0.0),
    };
    let ln_count: usize = words.clone().count();
    if ln_count >= 3 {
        words.next();
        point.x = words
            .next()
            .expect("X coord Undefined")
            .parse::<f32>()
            .expect("X can't parse");
        point.y = words
            .next()
            .expect("Y coord Undefined")
            .parse::<f32>()
            .expect("Y can't parse");
    }
    point
}

pub fn extract_face(f: &mut Vec<Face>, line: &str) {
    let mut words: SplitWhitespace<'_> = line.split_whitespace();
    let ln_count: usize = words.clone().count();
    let mut face: Face = Face {
        v: [0.0, 0.0, 0.0],
        vn: Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        vt: [0, 0, 0],
    };
    if ln_count >= 2 {
        words.next();
        let tmp_str = String::from(words.next().expect("oui").to_string());
        face.v = [1.0, 2.0, 3.0];
    }
    f.push(face);
}

// fn create_face_data(data: &str) -> FaceData {
//     let mut tmp = data.split_terminator("/");
//     let mut face_data: FaceData = FaceData { v: 0, vn: 0, vt: 0 };
//     face_data.v = tmp.next().expect("Face v").parse::<u32>().expect("Face vn");
//     if tmp.next().is_some() {
//         face_data.vn = tmp
//             .next()
//             .expect("Face vn")
//             .parse::<u32>()
//             .expect("Face vn");
//     }
//     if tmp.next().is_some() {
//         face_data.vt = tmp
//             .next()
//             .expect("Face vt")
//             .parse::<u32>()
//             .expect("Face vn");
//     }
//     face_data
// }
