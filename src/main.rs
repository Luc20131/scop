mod parser;
use cgmath::SquareMatrix;
use cgmath::prelude::*;
use cgmath::{Matrix4, Rad, vec3};
use gl::types::*;
use scop::graphics::gl_wrapper::*;
use scop::graphics::shaders::Shader;
use scop::my_lib::matrice::Matrice4;
use std::env;
use std::ffi::CStr;
use std::ffi::CString;
use std::fs;
use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str::Lines;
extern crate glfw;
use self::glfw::{Action, Context, Key};

macro_rules! null_str {
    ($lit:literal) => {{
        // "type check" the input
        const _: &str = $lit;
        concat!($lit, "\0")
    }};
}

macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    };
}

use scop::graphics::window::Window;

fn main() {
    let mut obj: parser::Obj = parser::Obj::new();
    let content = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let lines: Lines = content.lines();
    for line in lines {
        obj.add_data(line);
    }
    obj.build_indices();
    obj.print_data();

    let mut window = Window::new(1000, 1000, &obj.name);

    let vertices: &[GLfloat] = obj.v.as_slice();
    let indices: &[u32] = obj.indices.as_slice();

    // let uni_color_loc: i32;
    println!("Vertices test : {:?}", indices);
    println!("Face test : {:?}", obj.v);

    window.init_gl();

    let mut shaders = Shader::new(
        "./src/graphics/shaders_files/vertex.glsl",
        "./src/graphics/shaders_files/fragment.glsl",
    );

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

    // let index_attribute = VertexAttribute::new(
    //     1,
    //     3,
    //     gl::FLOAT,
    //     gl::FALSE,
    //     3 * mem::size_of::<GLfloat>() as GLsizei,
    //     ptr::null(),
    // );

    // let color_attribute = VertexAttribute::new(
    //     2,
    //     3,
    //     gl::FLOAT,
    //     gl::FALSE,
    //     6 * mem::size_of::<GLfloat>() as GLsizei,
    //     (3 * mem::size_of::<GLfloat>()) as *const c_void,
    // );

    // color_attribute.enable();

    // index_attribute.enable();
    shaders.use_prog();
    // let tmp: &str = "someUniform";
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

            let mut transform: Matrix4<f32> = Matrix4::identity();
            // transform = transform * Matrix4::<f32>::from_translation(vec3(0.0, -0.5, 0.0));
            transform = transform * Matrix4::<f32>::from_angle_y(Rad(window.get_time() as f32));
            transform = transform * Matrix4::<f32>::from_scale(0.2);
            // // get matrix's uniform location and set matrix
            shaders.use_prog();
            let transform_loc = gl::GetUniformLocation(shaders.id, c_str!("transform").as_ptr());
            gl::UniformMatrix4fv(transform_loc, 1, gl::FALSE, transform.as_ptr());
            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as GLsizei,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
        window.update();
    }
}
