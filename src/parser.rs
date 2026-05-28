

pub fn read_data(&mut self, line: &str) {
    let mut words: SplitWhitespace<'_> = line.split_whitespace();
    match words.next() {
        Some("o") => {
            self.name = extract_name(line);
        }
        Some("v") => self.extract_vertices(line),
        Some("vt") => self.extract_texture(line),
        Some("vn") => self.extract_normal(line),
        Some("f") => self.extract_face(line),
        Some(_) => {}
        None => {}
    }
}
