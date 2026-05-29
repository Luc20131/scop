use std::str::SplitWhitespace;

use scop::graphics::mesh::Mesh;

pub fn read_data(line: &str) -> Mesh {
    let mut mesh = Mesh::new(vertices, normal, texture);
    let mut words: SplitWhitespace<'_> = line.split_whitespace();
    match words.next() {
        Some("o") => {
            mesh.name = extract_name(line);
        }
        // Some("v") => self.extract_vertices(line),
        // Some("vt") => self.extract_texture(line),
        // Some("vn") => self.extract_normal(line),
        // Some("f") => self.extract_face(line),
        Some(_) => {}
        None => {}
    }
    mesh
}

pub fn extract_name(line: &str) -> String {
    let mut words: SplitWhitespace<'_> = line.split_whitespace();
    let ln_count: usize = words.clone().count();
    if ln_count >= 2 {
        return words.nth(1).expect("Name undefined").to_string();
    }
    "Undefined".to_string()
}
