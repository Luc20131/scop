use crate::{
    graphics::{
        mesh::{Mesh, Normal, TexCoord},
        texture::Texture,
    },
    math::vec3::Vec3,
    parsers::mtl_parser::MtlFile,
};

#[allow(dead_code)]
pub struct Model {
    pub vertices: Vec<u32>,
    pub meshes: Vec<Mesh>,
    pub textures: Vec<Texture>,
    pub mtl_file: Vec<MtlFile>,
}

impl Model {
    pub fn parse(&mut self, content: String) {
        let lined_content = content.lines();
        let mut mesh = Mesh::default();
        for elem in lined_content {
            if elem.starts_with("o ") || elem.starts_with("g ") {
                if mesh.indice.len() > 0 {
                    self.meshes.push(mesh);
                }
                mesh = Mesh::default();
                mesh.name = elem
                    .split_once(" ")
                    .unwrap()
                    .1
                    .to_string()
                    .trim()
                    .to_string();
                continue;
            }
            if !elem.starts_with("f ") {
                continue;
            }

            self.parse_line(elem);
        }
        self.meshes.push(mesh);
    }

    pub fn parse_line(&mut self, line: &str) {
        let Some((token, data)) = line.split_once(" ") else {
            return;
        };
        if token.is_empty() || data.is_empty() {
            return;
        }
        match token {
            "f" => {}
            _ => {}
        }
    }

    fn data_to_face(&mut self, data: &str) {
        let splited_data = data.split_whitespace();
        if splited_data.clone().count() < 3 {
            eprintln!("face format invalid : {}", data);
            return None;
        }
        for elem in splited_data {
            let mut elem_splited = elem.split("/");
            let vertex_index = elem_splited
                .next()
                .unwrap_or("0")
                .parse::<i32>()
                .unwrap_or(0);
            if vertex_index > 0 {
                self.indice.push((vertex_index - 1) as u32);
                // dbg!(vertex_index);
                // dbg!(
                //     self.vertices
                //         .iter()
                //         .nth((vertex_index - 1) as usize)
                //         .unwrap()
                // );
            }
            if let Some(v) = self.vertex_from_index(vertex_index) {
                self.vertices.push(v.as);
                self.face_elem.position = v;
            }
            // } else {
            //     continue;
            // }
            if let Some(vt) = self.tex_coord_from_index(
                elem_splited
                    .next()
                    .unwrap_or("0")
                    .parse::<i32>()
                    .unwrap_or(0),
            ) {
                face_elem.tex_coord = vt;
            }
            if let Some(vn) = self.normal_from_index(
                elem_splited
                    .next()
                    .unwrap_or("0")
                    .parse::<i32>()
                    .unwrap_or(0),
            ) {
                face_elem.normals = vn;
            }
            face.element.push(face_elem);
        }
        // dbg!(data);
        // dbg!(face.element.len());
        // if face.element.len() == 2 {
        //     dbg!(&face);
        // }
        Some(face)
    }
}

fn data_to_tex_coord(data: &str) -> Option<TexCoord> {
    let mut splited_data = data.split_whitespace();
    if splited_data.clone().count() != 2 {
        return None;
    }
    let value = (
        splited_data
            .next()
            .unwrap_or_default()
            .parse::<f32>()
            .unwrap_or(0.0),
        splited_data
            .next()
            .unwrap_or_default()
            .parse::<f32>()
            .unwrap_or(0.0),
    );
    Some(value)
}

fn data_to_vec3(data: &str) -> Option<Vec3> {
    let mut splited_data = data.split_whitespace();
    if splited_data.clone().count() != 3 {
        return None;
    }
    let value = Vec3::new(
        splited_data
            .next()
            .unwrap_or_default()
            .parse::<f32>()
            .unwrap_or(0.0),
        splited_data
            .next()
            .unwrap_or_default()
            .parse::<f32>()
            .unwrap_or(0.0),
        splited_data
            .next()
            .unwrap_or_default()
            .parse::<f32>()
            .unwrap_or(0.0),
    );
    Some(value)
}
