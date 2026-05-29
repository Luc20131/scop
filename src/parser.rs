use std::{
    fs::read_to_string,
    ops::Add,
    path::{Path, PathBuf},
    str::SplitWhitespace,
};

#[derive(Debug)]
struct MtlFile {
    path: PathBuf,
    content: String,
}

#[derive(Debug)]
pub struct ObjFile {
    pub name: String,
    content: String,
    mtl_files: Vec<MtlFile>,
}

impl ObjFile {
    pub fn new(path: &Path) -> Self {
        Self {
            name: "Undefined".to_string(),
            content: ObjFile::obj_file_read(path),
            mtl_files: vec![],
        }
    }

    fn obj_file_read(path: &Path) -> String {
        read_to_string(path).unwrap_or("empty file".to_string())
    }

    pub fn grep_mtl_files(&mut self) {
        let lines = self.content.lines();
        let mut files: Vec<MtlFile> = vec![];
        for line in lines {
            if line.contains("mtllib ") {
                let mut words: SplitWhitespace<'_> = line.split_whitespace();
                let path = PathBuf::from(words.next_back().unwrap_or("No path found"));
                let mtl_file: MtlFile = MtlFile {
                    path: (path.clone()),
                    content: (read_to_string(
                        "./".to_string().add(path.to_str().unwrap_or_default()),
                    )
                    .unwrap_or("empty file".to_string())),
                };
                files.push(mtl_file);
            }
        }
        println!("files: {:?}", files);
        self.mtl_files = files;
    }
}
