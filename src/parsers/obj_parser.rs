use std::{
    fs::read_to_string,
    io,
    path::{Path, PathBuf},
    str::SplitWhitespace,
};

use super::mtl_parser::MtlFile;

#[derive(Debug)]
pub struct ObjFile {
    pub name: String,
    path: PathBuf,
    content: String,
    mtl_files: Vec<MtlFile>,
}

impl ObjFile {
    pub fn new(path: &Path) -> Self {
        println!("Loading {}...", path.display());
        Self {
            name: "Undefined".to_string(),
            path: path.to_path_buf(),
            content: ObjFile::obj_file_read(path).unwrap_or_default(),
            mtl_files: vec![],
        }
    }

    fn obj_file_read(path: &Path) -> Result<String, io::Error> {
        let content = read_to_string(path)?;
        Ok(content)
    }

    pub fn get_mtl_files(&mut self) {
        let lines = self.content.lines();
        let mut files: Vec<MtlFile> = vec![];
        for line in lines {
            if line.contains("mtllib ") {
                let mut words: SplitWhitespace<'_> = line.split_whitespace();
                let path = PathBuf::from(words.next_back().unwrap_or("No path found"));
                println!("Loading {}...", path.display());
                let mut mtl_file: MtlFile = MtlFile {
                    path: (path.clone()),
                    content: (read_to_string(self.path.parent().unwrap().join(path))
                        .unwrap_or("empty file".to_string())),
                    materials: vec![],
                };
                mtl_file.parse();
                println!("Nb materials : {}", mtl_file.materials.iter().count());
                files.push(mtl_file);
            }
        }
        self.mtl_files = files;
        todo!("regroup get mtl files and new function to get all data just with new(PATH)");
    }
}
