// use std::str::SplitWhitespace;

// pub struct Obj {
//     pub name: String,
//     pub v: Vec<f32>,
//     pub vt: Vec<[f32; 3]>,
//     pub vn: Vec<[f32; 3]>,
//     // vp: Vec<Point>,
//     pub f: Vec<Face>,
//     // l: Vec<String>,
//     //
//     pub indices: Vec<u32>,
//     pub vn_built: Vec<f32>,
// }

// #[derive(Debug, Clone)]
// pub struct Face {
//     pub v_index: u32,
//     pub vt_index: u32,
//     pub vn_index: u32,
// }

// impl Obj {
//     pub fn new() -> Self {
//         Self {
//             name: "Undefined".to_string(),
//             v: Vec::<f32>::new(),
//             vt: Vec::<[f32; 3]>::new(),
//             vn: Vec::<[f32; 3]>::new(),
//             // vp: Vec::<Point>::new(),
//             f: Vec::<Face>::new(),
//             // l: Vec::<String>::new(),
//             indices: Vec::<u32>::new(),
//             vn_built: Vec::<f32>::new(),
//         }
//     }

// pub fn print_data(&mut self) {
//     println!("Name : {}", self.name);
//     println!("Number of Vertices: {}", self.v.iter().size_hint().0 / 3);
//     println!(
//         "Number of Vertices Textures: {}",
//         self.vt.iter().size_hint().0
//     );
//     println!(
//         "Number of Vertices Normal: {}",
//         self.vn.iter().size_hint().0
//     );
//     println!("Number of Faces: {}", self.f.iter().size_hint().0);
// }

// pub fn read_data(&mut self, line: &str) {
//     let mut words: SplitWhitespace<'_> = line.split_whitespace();
//     match words.next() {
//         Some("o") => self.name = extract_name(line),
//         Some("v") => self.extract_vertices(line),
//         Some("vt") => self.extract_texture(line),
//         // Some("vn") => self.extract_normal(line),
//         Some("f") => self.extract_face(line),
//         Some(_) => {}
//         None => {}
//     }
// }

//     fn extract_vertices(&mut self, line: &str) {
//         let mut words: SplitWhitespace<'_> = line.split_whitespace();
//         let tmp: [f32; 3];
//         words.next();
//         tmp = [
//             words
//                 .next()
//                 .expect("invalid x value")
//                 .parse::<f32>()
//                 .expect("failed parse x value"),
//             words
//                 .next()
//                 .expect("invalid y value")
//                 .parse::<f32>()
//                 .expect("failed parse y value"),
//             words
//                 .next()
//                 .expect("invalid z value")
//                 .parse::<f32>()
//                 .expect("failed parse z value"),
//         ];
//         let w = words
//             .next()
//             .map(|s| s.parse::<f32>().expect("invalid w value"))
//             .unwrap_or(1.0);
//         self.v.push(tmp[0] * w);
//         self.v.push(tmp[1] * w);
//         self.v.push(tmp[2] * w);
//     }

//     fn extract_face(&mut self, line: &str) {
//         let mut words: SplitWhitespace<'_> = line.split_whitespace();
//         words.next();
//         let tmp = create_face_data(words.next().unwrap());
//         let face: Face = Face {
//             v_index: tmp.0,
//             vt_index: tmp.1,
//             vn_index: tmp.2,
//         };
//         self.f.push(face);
//         let tmp = create_face_data(words.next().unwrap());
//         let face: Face = Face {
//             v_index: tmp.0,
//             vt_index: tmp.1,
//             vn_index: tmp.2,
//         };
//         self.f.push(face);
//         let tmp = create_face_data(words.next().unwrap());
//         let face: Face = Face {
//             v_index: tmp.0,
//             vt_index: tmp.1,
//             vn_index: tmp.2,
//         };
//         self.f.push(face);
//     }

//     pub fn extract_texture(&mut self, line: &str) {
//         let mut words: SplitWhitespace<'_> = line.split_whitespace();
//         let tmp: [f32; 3];
//         words.next();
//         let u = words
//             .next()
//             .expect("no text")
//             .parse::<f32>()
//             .expect("parse failed on text");
//         let v = words
//             .next()
//             .map(|s| s.parse::<f32>().expect("invalid v value"))
//             .unwrap_or(0.0);
//         let w = words
//             .next()
//             .map(|s| s.parse::<f32>().expect("invalid w value"))
//             .unwrap_or(0.0);
//         tmp = [u, v, w];
//         self.vt.push(tmp);
//     }

//     pub fn build_indices(&mut self) {
//         let mut indices: Vec<u32> = Vec::<u32>::new();
//         let mut normals: Vec<f32> = Vec::<f32>::new();
//         let tmp_f = self.f.clone();
//         for elem in tmp_f {
//             indices.push(elem.v_index - 1);
//             if self.vn.len() > elem.vn_index as usize {
//                 normals = self.unwrap_normal((elem.vn_index) as usize);
//             }
//         }
//         self.indices = indices;
//         self.vn_built = normals;
//     }

//     fn unwrap_normal(&mut self, i: usize) -> Vec<f32> {
//         let mut norm: Vec<f32> = Vec::<f32>::new();
//         norm.push(self.vn[i][0]);
//         norm.push(self.vn[i][1]);
//         norm.push(self.vn[i][2]);
//         norm
//     }
// }

// pub fn extract_name(line: &str) -> String {
//     let mut words: SplitWhitespace<'_> = line.split_whitespace();
//     let ln_count: usize = words.clone().count();
//     if ln_count >= 2 {
//         return words.nth(1).expect("Name undefined").to_string();
//     }
//     "Undefined".to_string()
// }

// fn create_face_data(data: &str) -> (u32, u32, u32) {
//     let mut parts = data.split("/");
//     let v = parts.next().expect("Face v").parse::<u32>().unwrap();
//     let vt = parts
//         .next()
//         .and_then(|s| {
//             if s.is_empty() {
//                 None
//             } else {
//                 Some(s.parse::<u32>().expect("invalid vt value"))
//             }
//         })
//         .unwrap_or(0);
//     let vn = parts
//         .next()
//         .and_then(|s| {
//             if s.is_empty() {
//                 None
//             } else {
//                 Some(s.parse::<u32>().expect("invalid vn value"))
//             }
//         })
//         .unwrap_or(0);
//     (v, vt, vn)
// }
