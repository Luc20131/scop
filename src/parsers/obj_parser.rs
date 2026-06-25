use std::{
    collections::HashMap,
    fs::read_to_string,
    io::{self},
    path::{Path, PathBuf},
    str::SplitWhitespace,
    vec,
};

use crate::{
    graphics::{
        mesh::{Face, FaceElem, Mesh, TexCoord},
        model::Model,
    },
    math::vec3::Vec3,
};

use super::mtl_parser::MtlFile;

#[derive(Debug, Clone)]

pub struct ObjFile {
    pub name: String,
    path: PathBuf,
    pub vertex: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub tex_coord: Vec<TexCoord>,
    pub mtl_files: Vec<MtlFile>,
    pub model: Model,
}

impl ObjFile {
    pub fn new(path: &Path) -> Option<Self> {
        println!("Loading {}...", path.display());
        let content = match read_file(&path.to_path_buf()) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("{}", err);
                return None;
            }
        };
        let mut obj = ObjFile {
            name: path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or("Undefined")
                .to_string(),
            vertex: vec![],
            normals: vec![],
            tex_coord: vec![],
            path: path.to_path_buf(),
            mtl_files: vec![],
            model: Model::default(),
        };
        obj.parse(content);
        obj.modelise();
        Some(Self {
            name: obj.name,
            path: obj.path,
            vertex: obj.vertex,
            normals: obj.normals,
            tex_coord: obj.tex_coord,
            mtl_files: obj.mtl_files,
            model: obj.model,
        })
    }

    ///Parse .mtl file and add it to the mtl_files vector
    ///# Error:
    /// Print the error on stderr and resumes .obj parsing
    pub fn parse_mtl_file(&mut self, data: &str) {
        println!("Mtl file verification...");
        let Some((start, file_name)) = data.split_once("mtllib ") else {
            eprintln!(
                "Error: split failed on the mtlfile line when it tried to retrieve file name"
            );
            return;
        };
        if !start.trim().is_empty() {
            eprintln!("Error: the mtllib line is in the wrong format ");
            return;
        }
        let name = file_name.trim();
        if !name.ends_with(".mtl") || name.is_empty() {
            eprintln!("Error: the MTL file name in the OBJ file is invalid or is empty");
            return;
        }
        let mut path = PathBuf::from(name);
        path = self
            .path
            .parent()
            .expect("Incorrect obj file parent path")
            .join(&path);
        if !path.is_file() {
            eprintln!("Error: {} not found", path.display());
            return;
        }
        println!("Loading {}...", path.display());
        if let Ok(content) = read_file(&path) {
            let mut mtl_file: MtlFile = MtlFile {
                path: (path.clone()),
                content: content,
                materials: vec![],
            };
            if !mtl_file.content.is_empty() {
                mtl_file.parse();
                println!(
                    "Number of materials : {}",
                    mtl_file.materials.iter().count()
                );
            }
            self.mtl_files.push(mtl_file);
        } else {
            println!("Failed to read {}", path.display());
        }
    }

    fn parse(&mut self, content: String) {
        let lined_content = content.lines();
        for elem in lined_content {
            self.parse_line(elem.trim());
        }
    }

    pub fn parse_line(&mut self, line: &str) {
        let Some((token, data)) = line.split_once(" ") else {
            return;
        };
        if token.is_empty() || data.is_empty() {
            return;
        }
        match token {
            "v" => {
                if let Some(value) = data_to_vec3(data) {
                    self.vertex.push(value);
                }
            }
            "vn" => {
                if let Some(value) = data_to_vec3(data) {
                    self.normals.push(value);
                }
            }
            "vt" => {
                if let Some(value) = data_to_tex_coord(data) {
                    self.tex_coord.push(value);
                }
            }
            "mtllib" => {
                self.parse_mtl_file(line);
            }
            "o" => {
                self.model.meshes.push(Mesh::default());
                println!("obj name: {}", data);
                self.model.meshes.last_mut().unwrap().name = data.to_string();
            }
            "g " => {
                self.model.meshes.push(Mesh::default());
                println!("group name: {}", data);
                self.model.meshes.last_mut().unwrap().name = data.to_string();
            }
            "f" => {
                if self.model.meshes.len() == 0 {
                    self.model.meshes.push(Mesh::default());
                }
                let msh: &mut Mesh = self.model.meshes.last_mut().unwrap();
                if let Some(face) = data_to_face(data) {
                    msh.faces.push(face);
                } else if let Some(faces) = triangulate_face(data) {
                    for face in faces {
                        msh.faces.push(face);
                    }
                }
            }
            "s" => {}
            "usemtl" => {}
            "l" => {}
            _ => {}
        }

        fn triangulate_face(data: &str) -> Option<Vec<Face>> {
            let mut faces: Vec<Face> = vec![];
            let mut splited_data = data.split_whitespace();
            let origin: String = String::from(splited_data.next().unwrap_or_default());
            let mut buffer = splited_data.next().unwrap_or_default();
            let mut face_buffer: String;
            for elem in splited_data {
                face_buffer = origin.clone() + " " + buffer + " " + elem;
                buffer = elem;
                if let Some(face) = data_to_face(face_buffer.as_str()) {
                    faces.push(face);
                }
            }
            Some(faces)
        }
    }

    fn modelise(&mut self) {
        println!("nb_mesh : {}", self.model.meshes.len());
        for mesh in &mut self.model.meshes {
            let mut indice_map: HashMap<u32, u32> = HashMap::<u32, u32>::new();
            let mut index: u32 = 0;
            for face in &mesh.faces {
                for face_elem in face {
                    if !indice_map.contains_key(&face_elem.vertex) {
                        index += 1;
                        indice_map.entry(face_elem.vertex).insert_entry(index);

                        let v_indice = face_elem.vertex - 1;
                        mesh.vertices.push(self.vertex[v_indice as usize].x);
                        mesh.vertices.push(self.vertex[v_indice as usize].y);
                        mesh.vertices.push(self.vertex[v_indice as usize].z);
                        if self.tex_coord.len() > face_elem.tex_coord.unwrap_or_default() as usize {
                            mesh.vertices.push(
                                self.tex_coord[face_elem.tex_coord.unwrap_or_default() as usize].0,
                            );
                            mesh.vertices.push(
                                self.tex_coord[face_elem.tex_coord.unwrap_or_default() as usize].1,
                            );
                        } else {
                            mesh.vertices.push(0.0);
                            mesh.vertices.push(0.0);
                        }
                        if self.normals.len() > face_elem.normals.unwrap_or_default() as usize {
                            mesh.vertices.push(
                                self.normals[face_elem.normals.unwrap_or_default() as usize].x,
                            );
                            mesh.vertices.push(
                                self.normals[face_elem.normals.unwrap_or_default() as usize].y,
                            );
                            mesh.vertices.push(
                                self.normals[face_elem.normals.unwrap_or_default() as usize].z,
                            );
                        } else {
                            mesh.vertices.push(0.0);
                            mesh.vertices.push(0.0);
                            mesh.vertices.push(0.0);
                        }
                    }
                    mesh.indices
                        .push(*indice_map.entry(face_elem.vertex).or_default() - 1);
                }
            }
            // dbg!(&mesh.vertices);
            mesh.setup_mesh();
        }
    }

    pub fn get_middle_offset(&self) -> Vec3 {
        let offset: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        offset
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

pub fn read_file(path: &PathBuf) -> Result<String, io::Error> {
    let content = read_to_string(path)?;
    Ok(content)
}

fn data_to_face(data: &str) -> Option<Face> {
    let splited_data = data.split_whitespace();
    if splited_data.clone().count() < 3 {
        eprintln!("face format invalid : {}", data);
        return None;
    } else if splited_data.clone().count() > 3 {
        return None;
    }
    let mut face: Face = vec![];
    for elem in splited_data {
        let mut face_elem: FaceElem = FaceElem {
            vertex: 0,
            tex_coord: None,
            normals: None,
        };
        let mut elem_splited = elem.split("/");
        if let Some(vertex_elem) = elem_splited.next() {
            if let Ok(vertex_indice) = vertex_elem.parse::<u32>() {
                face_elem.vertex = vertex_indice;
            }
        } else {
            eprintln!("Error: a face doesn't have vertex indice");
            return None;
        }
        if let Some(tex_coord_elem) = elem_splited.next() {
            if let Ok(tex_coord_indice) = tex_coord_elem.parse::<u32>() {
                face_elem.tex_coord = Some(tex_coord_indice);
            }
        } else {
            face_elem.tex_coord = None;
        }
        if let Some(normal_elem) = elem_splited.next() {
            if let Ok(normal_indice) = normal_elem.parse::<u32>() {
                face_elem.normals = Some(normal_indice);
            }
        } else {
            face_elem.normals = None;
        }
        face.push(face_elem);
    }
    Some(face)
}
