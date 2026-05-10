mod parser;
use gl::types::*;
use scop::graphics::gl_wrapper::*;
use scop::graphics::shaders::Shader;
use scop::my_lib::bmp_parser::image_loader;
use scop::my_lib::matrice::Matrice4;
use std::env;
use std::f32::consts::PI;
use std::ffi::CStr;
use std::fs;
use std::mem;
use std::ptr;
use std::str::Lines;
extern crate glfw;
// use self::glfw::{Action, Context, Key};

// macro_rules! null_str {
//     ($lit:literal) => {{
//         // "type check" the input
//         const _: &str = $lit;
//         concat!($lit, "\0")
//     }};
// }

macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    };
}

use scop::graphics::window::Window;

fn main() {
    println!("Loading Object...");
    let mut obj: parser::Obj = parser::Obj::new();
    let content = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let lines: Lines = content.lines();
    for line in lines {
        obj.add_data(line);
    }
    obj.build_indices();
    obj.print_data();
    println!("Loading Textures...");
    image_loader("./resources/texture.bmp");

    let mut window = Window::new(1920, 1080, &obj.name);

    let vertices: &[GLfloat] = obj.v.as_slice();
    let indices: &[u32] = obj.indices.as_slice();
    //INIT OpenGL
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
    //     2,
    //     3,
    //     gl::FLOAT,
    //     gl::FALSE,
    //     3 * mem::size_of::<GLfloat>() as GLsizei,
    //     ptr::null(),
    // );

    let color_attribute = VertexAttribute::new(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );

    color_attribute.enable();

    // println!("{:?}", transform);
    // index_attribute.enable();
    shaders.use_prog();

    let mut time = window.get_time();
    // let tmp: &str = "someUniform";
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

            let mut transform: Matrice4<f32> = Matrice4::identity();
            transform = transform.scale(0.05);
            shaders.use_prog();
            let oui = window.get_time() as f32;
            transform.rotate(PI / 6.0, oui, 0.0);
            // transform.translate(-0.1, -1.0, -0.1);
            let transform_loc = gl::GetUniformLocation(shaders.id, c_str!("transform").as_ptr());
            gl::UniformMatrix4fv(transform_loc, 1, gl::TRUE, transform.value.as_ptr());
            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as GLsizei,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
            time = window.update_title(time);
        }
        window.update();
    }
}
