// use crate::graphics::gl_wrapper::{BufferObject, Vao};
use crate::graphics::texture::Texture;
use crate::math::vec3::Vec3;
use crate::parsers::mtl_parser::Material;

pub type Normal = Vec3;
pub type TexCoord = (f32, f32);

#[derive(Debug)]
#[allow(unused)]
pub struct Face {
    element: Vec<FaceElem>,
}

#[derive(Debug)]
pub struct FaceElem {
    pub position: Vec3,
    pub tex_coord: TexCoord,
    pub normals: Vec3,
}

#[derive(Debug)]
pub struct Mesh {
    pub name: String,
    pub texture: Vec<Texture>,
    pub material: Material,
    pub indice: Vec<u32>,
    pub vertices: Vec<f32>,
    pub smoothing: u8,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            name: "Undefined".to_string(),
            indice: vec![],
            texture: vec![],
            smoothing: 0,
            material: Material::default(),
        }
    }
}

#[allow(dead_code)]
impl Mesh {
    pub fn new(name: String, texture: Vec<Texture>, smoothing: u8) -> Self {
        Self {
            name,
            indice: vec![],
            texture,
            smoothing,
            material: Material::default(),
        }
    }

    // pub fn is_smoothing(&self) -> bool {
    //     self.smoothing != 0
    // }
}

// pub fn parse_line(&mut self, line: &str) {
//     let Some((token, data)) = line.split_once(" ") else {
//         return;
//     };
//     if token.is_empty() || data.is_empty() {
//         return;
//     }
//     match token {
//         "v" => {
//             if let Some(value) = data_to_vec3(data) {
//                 self.vertices.push(value);
//             }
//         }
//         "vn" => {
//             if let Some(value) = data_to_vec3(data) {
//                 self.normal.push(value);
//             }
//         }
//         "vt" => {
//             if let Some(value) = data_to_tex_coord(data) {
//                 self.tex_coord.push(value);
//             }
//         }
//         "usemtl" => {
//             let value = data.to_string();
//             self.material.name = value;
//         }
//         "f" => {
//             if let Some(face) = self.data_to_face(data) {
//                 self.face.push(face);
//             }
//         }
//         _ => {}
//     }
// }

// fn data_to_face(&mut self, data: &str) -> Option<Face> {
//     let splited_data = data.split_whitespace();
//     if splited_data.clone().count() < 3 {
//         eprintln!("face format invalid : {}", data);
//         return None;
//     }
//     let mut face: Face = Face { element: vec![] };
//     for elem in splited_data {
//         let mut face_elem: FaceElem = FaceElem {
//             position: Vec3 {
//                 x: 0.0,
//                 y: 0.0,
//                 z: 0.0,
//             },
//             tex_coord: TexCoord::default(),
//             normals: Vec3 {
//                 x: 0.0,
//                 y: 0.0,
//                 z: 0.0,
//             },
//         };
//         let mut elem_splited = elem.split("/");
//         let vertex_index = elem_splited
//             .next()
//             .unwrap_or("-1")
//             .parse::<i32>()
//             .unwrap_or(-1);
//         if vertex_index > 0 {
//             self.indice.push((vertex_index - 1) as u32);
//             // dbg!(vertex_index);
//             // dbg!(
//             //     self.vertices
//             //         .iter()
//             //         .nth((vertex_index - 1) as usize)
//             //         .unwrap()
//             // );
//         }
//         if let Some(v) = self.vertex_from_index(vertex_index) {
//             face_elem.position = v;
//         }
//         // } else {
//         //     continue;
//         // }
//         if let Some(vt) = self.tex_coord_from_index(
//             elem_splited
//                 .next()
//                 .unwrap_or("-1")
//                 .parse::<i32>()
//                 .unwrap_or(-1),
//         ) {
//             face_elem.tex_coord = vt;
//         }
//         if let Some(vn) = self.normal_from_index(
//             elem_splited
//                 .next()
//                 .unwrap_or("-1")
//                 .parse::<i32>()
//                 .unwrap_or(-1),
//         ) {
//             face_elem.normals = vn;
//         }
//         face.element.push(face_elem);
//     }
//     // dbg!(data);
//     // dbg!(face.element.len());
//     // if face.element.len() == 2 {
//     //     dbg!(&face);
//     // }
//     Some(face)
// }

//     fn vertex_from_index(&mut self, index: i32) -> Option<Vec3> {
//         let len = self.vertices.len();
//         // dbg!(len);
//         if len >= index as usize && index - 1 >= 0 {
//             let v = self.vertices[(index - 1) as usize].clone();
//             // if index == 42 {
//             //     dbg!(index);
//             // }
//             Some(v)
//         } else {
//             None
//         }
//     }

//     fn normal_from_index(&mut self, index: i32) -> Option<Vec3> {
//         let len = self.normal.len();

//         if len >= index as usize && index - 1 >= 0 {
//             let vn = self.normal[(index - 1) as usize].clone();
//             Some(vn)
//         } else {
//             None
//         }
//     }

//     fn tex_coord_from_index(&mut self, index: i32) -> Option<TexCoord> {
//         let len = self.tex_coord.len();
//         if len >= index as usize && index - 1 >= 0 {
//             let vt = self.tex_coord[(index - 1) as usize].clone();
//             Some(vt)
//         } else {
//             None
//         }
//     }

//     fn data_to_mtl(&mut self, data: &str) {}

//     pub fn mesh_to_slice(&self) -> Vec<f32> {
//         let mut sliced: Vec<f32> = vec![];
//         // for face in &self.face {
//         //     for face_elem in &face.element {
//         //         dbg!(face_elem);
//         //         sliced.push(face_elem.position.x);
//         //         sliced.push(face_elem.position.y);
//         //         sliced.push(face_elem.position.z);
//         //         // sliced.push(face_elem.tex_coord.0);
//         //         // sliced.push(face_elem.tex_coord.1);
//         //         // sliced.push(face_elem.normals.x);
//         //         // sliced.push(face_elem.normals.y);
//         //         // sliced.push(face_elem.normals.z);
//         //     }
//         // }
//         for v in &self.vertices {
//             sliced.push(v.x);
//             sliced.push(v.y);
//             sliced.push(v.z);
//             sliced.push((v.x));
//             sliced.push((v.y));
//             sliced.push((v.z));
//         }
//         sliced
//     }
// }

// fn data_to_tex_coord(data: &str) -> Option<TexCoord> {
//     let mut splited_data = data.split_whitespace();
//     if splited_data.clone().count() != 2 {
//         return None;
//     }
//     let value = (
//         splited_data
//             .next()
//             .unwrap_or_default()
//             .parse::<f32>()
//             .unwrap_or(0.0),
//         splited_data
//             .next()
//             .unwrap_or_default()
//             .parse::<f32>()
//             .unwrap_or(0.0),
//     );
//     Some(value)
// }

// fn data_to_vec3(data: &str) -> Option<Vec3> {
//     let mut splited_data = data.split_whitespace();
//     if splited_data.clone().count() != 3 {
//         return None;
//     }
//     let value = Vec3::new(
//         splited_data
//             .next()
//             .unwrap_or_default()
//             .parse::<f32>()
//             .unwrap_or(0.0),
//         splited_data
//             .next()
//             .unwrap_or_default()
//             .parse::<f32>()
//             .unwrap_or(0.0),
//         splited_data
//             .next()
//             .unwrap_or_default()
//             .parse::<f32>()
//             .unwrap_or(0.0),
//     );
//     Some(value)
// }
