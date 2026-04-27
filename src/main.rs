mod my_obj;
use gl::COMPILE_STATUS;
use gl::DrawArrays;
use gl::types::*;
use scop::graphics::gl_wrapper::*;
use scop::my_lib::matrice::Matrice4;
use std::env;
use std::ffi::c_char;
use std::fs;
use std::mem;
use std::ptr;
use std::str::Lines;

macro_rules! null_str {
    ($lit:literal) => {{
        // "type check" the input
        const _: &str = $lit;
        concat!($lit, "\0")
    }};
}

use scop::graphics::window::Window;

fn main() {
    let mut obj: my_obj::Obj = my_obj::Obj::new();
    let content = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let lines: Lines = content.lines();
    for line in lines {
        obj.add_data(line);
    }
    obj.print_data();

    let mut m1: Matrice4<f32> = Matrice4 {
        value: ([
            4.0, 0.0, 4.0, 0.0, 4.0, 2.0, 4.0, 2.0, 4.0, 0.0, 4.0, 0.0, 4.0, 2.0, 4.0, 2.0,
        ]),
    };

    let m2: Matrice4<f32> = Matrice4 {
        value: ([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ]),
    };

    m1 = m1.multiply(m2);
    println!("m1 : {:?}", m1);
    let mut window = Window::new(1000, 1000, "oui");

    let vertices: &[GLfloat] = obj.v.as_slice();
    // let indices: &[u32] = obj.f.as_slice();

    // let uni_color_loc: i32;
    // println!("Vertices test : {:?}", vertices);
    println!("Face test : {:?}", obj.f);

    let indices: [u32; 3] = [0, 1, 2];

    window.init_gl();

    let vao = Vao::new();
    vao.bind();

    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_f32_data(&vertices);

    let ebo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
    ebo.bind();

    ebo.store_u32_data(&indices);

    let position_attribute = VertexAttribute::new(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );

    position_attribute.enable();

    let index_attribute = VertexAttribute::new(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );

    index_attribute.enable();

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // gl::Uniform4f(uni_color_loc, 0.1, 0.5, 0.1, 1.0);
            gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, ptr::null());
            // DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.update();
    }
}
