use std::{
    fs::read_to_string,
    io::{self},
    path::{Path, PathBuf},
};

use crate::graphics::{mesh::Mesh, model::Model};

use super::mtl_parser::MtlFile;

#[derive(Debug, Clone)]

pub struct ObjFile {
    pub name: String,
    path: PathBuf,
    content: String,
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
            path: path.to_path_buf(),
            content: content,
            mtl_files: vec![],
        };
        obj.get_mtl_files();
        Some(Self {
            name: obj.name,
            path: obj.path,
            content: obj.content,
            mtl_files: obj.mtl_files,
        })
    }

    ///Searches for mtl files within .obj file, parse them and adds them to the mtl_files attribute
    ///# Error:
    /// Print the error on stderr and resumes .obj parsing
    pub fn get_mtl_files(&mut self) {
        println!("Mtl file verification...");
        let content_lines = self.content.lines();
        let mut files: Vec<MtlFile> = vec![];
        for line in content_lines {
            let Some((start, file_name)) = line.split_once("mtllib ") else {
                continue;
            };
            if !start.trim().is_empty() {
                eprintln!("Error: the mtllib line is in the wrong format ");
                return;
            }
            let name = file_name.trim();
            if !name.ends_with(".mtl") || name.is_empty() {
                eprintln!("Error: the MTL file name in the OBJ file is invalid or does not exist");
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
                    path: (path),
                    content: content,
                    materials: vec![],
                };
                if !mtl_file.content.is_empty() {
                    mtl_file.parse();
                    println!(
                        "Number of materials : {}",
                        mtl_file.materials.iter().count()
                    );
                    files.push(mtl_file);
                }
                continue;
            }
            println!("Failed to read {}", path.display());
        }
        if files.len() > 0 {
            self.mtl_files = files;
        } else {
            println!(
                "{} doesn't have mtl file, resume obj parsing...",
                self.path.file_name().unwrap_or_default().display()
            );
        }
    }

    pub fn modelise(&mut self) -> Model {
        println!("Modelise {}", self.name);
        let mut model = Model {
            meshes: vec![],
            textures: vec![],
            mtl_file: vec![],
        };
        model.mtl_file = self.mtl_files.clone();
        model.meshes = self.parse_mesh();
        model
    }

    fn parse_mesh(&self) -> Vec<Mesh> {
        let model_content = self.content.split_terminator("o ");
        let mut meshes: Vec<Mesh> = vec![];
        for elem in model_content {
            let mut mesh = Mesh::default();
            let mut lines = elem.lines();
            let first_line = lines.next().unwrap_or_default();
            if first_line.starts_with("#") || first_line.starts_with("mtllib ") {
                continue;
            }

            mesh.name = first_line.to_string();
            // dbg!(&mesh.name);
            for mut line in lines {
                line = line.trim();
                mesh.parse_line(line);
            }

            meshes.push(mesh);
        }
        // println!("New Elem {}", meshes.len());
        meshes
    }
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
