use std::{
    fs::read_to_string,
    io::{self},
    path::{Path, PathBuf},
    vec,
};

use crate::{
    graphics::{
        mesh::{Mesh, TexCoord},
        model::Model,
    },
    math::vec3::Vec3,
};

use super::mtl_parser::MtlFile;

#[derive(Debug, Clone)]

pub struct ObjFile {
    pub name: String,
    path: PathBuf,
    content: String,
    pub vertex: Vec<Vec3>,
    normals: Vec<Vec3>,
    tex_coord: Vec<TexCoord>,
    mtl_files: Vec<MtlFile>,
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
            content: content.clone(),
            mtl_files: vec![],
        };
        obj.parse(content);
        Some(Self {
            name: obj.name,
            path: obj.path,
            vertex: obj.vertex,
            normals: obj.normals,
            tex_coord: obj.tex_coord,
            content: obj.content,
            mtl_files: obj.mtl_files,
        })
    }

    ///Searches for mtl files within .obj file, parse them and adds them to the mtl_files attribute
    ///# Error:
    /// Print the error on stderr and resumes .obj parsing
    // pub fn get_mtl_files(&mut self) {
    //     println!("Mtl file verification...");
    //     let content_lines = self.content.lines();
    //     let mut files: Vec<MtlFile> = vec![];
    //     for line in content_lines {
    //         let Some((start, file_name)) = line.split_once("mtllib ") else {
    //             continue;
    //         };
    //         if !start.trim().is_empty() {
    //             eprintln!("Error: the mtllib line is in the wrong format ");
    //             return;
    //         }
    //         let name = file_name.trim();
    //         if !name.ends_with(".mtl") || name.is_empty() {
    //             eprintln!("Error: the MTL file name in the OBJ file is invalid or does not exist");
    //             return;
    //         }
    //         let mut path = PathBuf::from(name);
    //         path = self
    //             .path
    //             .parent()
    //             .expect("Incorrect obj file parent path")
    //             .join(&path);
    //         if !path.is_file() {
    //             eprintln!("Error: {} not found", path.display());
    //             return;
    //         }
    //         println!("Loading {}...", path.display());
    //         if let Ok(content) = read_file(&path) {
    //             let mut mtl_file: MtlFile = MtlFile {
    //                 path: (path),
    //                 content: content,
    //                 materials: vec![],
    //             };
    //             if !mtl_file.content.is_empty() {
    //                 mtl_file.parse();
    //                 println!(
    //                     "Number of materials : {}",
    //                     mtl_file.materials.iter().count()
    //                 );
    //                 files.push(mtl_file);
    //             }
    //             continue;
    //         }
    //         println!("Failed to read {}", path.display());
    //     }
    //     if files.len() > 0 {
    //         self.mtl_files = files;
    //     } else {
    //         println!(
    //             "{} doesn't have mtl file, resume obj parsing...",
    //             self.path.file_name().unwrap_or_default().display()
    //         );
    //     }
    // }

    pub fn modelise(&mut self) -> Model {
        println!("Modelise {}", self.name);
        let mut model = Model {
            vertices: vec![],
            meshes: vec![],
            textures: vec![],
            mtl_file: vec![],
        };
        model.mtl_file = self.mtl_files.clone();
        model.parse(self.content.clone());
        // self.parse_mesh();
        // dbg!(&model.vertices);
        model
    }

    fn parse(&mut self, content: String) {
        let lined_content = content.lines();
        // let mut mesh = Mesh::default();
        for elem in lined_content {
            if elem.starts_with("#") {
                continue;
            }
            if elem.starts_with("o ") || elem.starts_with("g ") {
                // mesh = Mesh::default();
                // mesh.name = elem
                //     .split_once(" ")
                //     .unwrap()
                //     .1
                //     .to_string()
                //     .trim()
                //     .to_string();
                continue;
            }
            self.parse_line(elem);
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
                    // if !self.vertex.contains(&value) {
                    self.vertex.push(value);
                    // }
                }
            }
            "vn" => {
                if let Some(value) = data_to_vec3(data) {
                    // if !self.normals.contains(&value) {
                    self.normals.push(value);
                    // }
                }
            }
            "vt" => {
                if let Some(value) = data_to_tex_coord(data) {
                    // if !self.tex_coord.contains(&value) {
                    self.tex_coord.push(value);
                    // }
                }
            }
            _ => {}
        }
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

//let lines = elem.lines();
// for line in lines {
//     let mut words = line.split_whitespace();
//     match words.next() {
//         Some("o") => self.name = words.next().unwrap_or("Undefined").to_string(),
//         Some(_) => {}
//         None => {}
//     }
//}
