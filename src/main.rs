mod my_obj;
use gl::DrawArrays;
use gl::types::*;
use scop::graphics::gl_wrapper::*;
use std::env;
use std::fs;
use std::mem;
use std::ptr;
use std::str::Lines;

use scop::graphics::window::Window;

fn main() {
    let mut obj: my_obj::Obj = my_obj::Obj::new();
    let content = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let lines: Lines = content.lines();
    for line in lines {
        obj.add_data(line);
    }
    obj.print_data();

    let mut window = Window::new(100, 100, "oui");

    let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    window.init_gl();

    let vao = Vao::new();
    vao.bind();

    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_f32_data(&vertices);

    let position_attribute = VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );

    position_attribute.enable();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.1, 0.5, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.update();
    }
}
