// use crate::graphics::gl_wrapper::{BufferObject, Vao};
use crate::graphics::texture::Texture;
use crate::math::vec3::Vec3;
use crate::parsers::mtl_parser::Material;

pub type Normal = Vec3;
pub type TexCoord = (f32, f32);
pub type Face = Vec<FaceElem>;

#[derive(Debug, Clone)]
pub struct FaceElem {
    pub vertex: u32,
    pub tex_coord: Option<u32>,
    pub normals: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub name: String,
    pub texture: Vec<Texture>,
    pub material: Material,
    pub indice: Vec<u32>,
    pub vertices: Vec<f32>,
    pub faces: Vec<Face>,
    pub smoothing: u8,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            name: "Undefined".to_string(),
            indice: vec![],
            vertices: vec![],
            texture: vec![],
            faces: vec![],
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
            vertices: vec![],
            texture: texture,
            faces: vec![],
            smoothing,
            material: Material::default(),
        }
    }

}

    // fn vertex_from_index(&mut self, index: i32) -> Option<Vec3> {
    //     let len = self.vertices.len();
    //     // dbg!(len);
    //     if len >= index as usize && index - 1 >= 0 {
    //         let v = self.vertices[(index - 1) as usize].clone();
    //         // if index == 42 {
    //         //     dbg!(index);
    //         // }
    //         Some(v)
    //     } else {
    //         None
    //     }
    // }

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
