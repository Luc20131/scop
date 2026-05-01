mod parser;
use gl::types::*;
use scop::graphics::gl_wrapper::*;
use scop::graphics::shaders::Shader;
use scop::my_lib::matrice::Matrice4;
use std::env;
use std::f32::consts::PI;
use std::ffi::CString;
use std::fs;
use std::mem;
use std::os::raw::c_void;
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
    let mut obj: parser::Obj = parser::Obj::new();
    let content = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let lines: Lines = content.lines();
    for line in lines {
        obj.add_data(line);
    }
    obj.build_indices();
    obj.print_data();

    // let mut m1: Matrice4<f64> = Matrice4 {
    //     value: ([
    //         4.0, 0.0, 4.0, 0.0, 4.0, 2.0, 4.0, 2.0, 4.0, 0.0, 4.0, 0.0, 4.0, 2.0, 4.0, 2.0,
    //     ]),
    // };

    // let m2: Matrice4<f64> = Matrice4 {
    //     value: ([
    //         1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     ]),
    // };

    let mut window = Window::new(1000, 1000, &obj.name);

    let vertices: &[GLfloat] = obj.v.as_slice();
    let indices: &[u32] = obj.indices.as_slice();

    // let uni_color_loc: i32;
    println!("Vertices test : {:?}", indices);
    println!("Face test : {:?}", obj.vn_built);

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
        6 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );

    position_attribute.enable();

    let color_attribute = VertexAttribute::new(
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        6 * mem::size_of::<GLfloat>() as GLsizei,
        (3 * mem::size_of::<GLfloat>()) as *const c_void,
    );

    color_attribute.enable();

    let index_attribute = VertexAttribute::new(
        2,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );

    index_attribute.enable();
    shaders.use_prog();
    // let tmp: &str = "someUniform";
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

            let mut transform = Matrice4::<f32>::default(); // make sure to initialize matrix to identity matrix first
            transform.translate(0.5, -0.5, 0.5);
            transform.rotate(PI / 2.0, 0.0, 0.0);

            // get matrix's uniform location and set matrix
            shaders.use_prog();
            let tmp_bis = CString::new("transform").unwrap();
            let transform_loc: GLint = gl::GetUniformLocation(shaders.id, tmp_bis.as_ptr());
            gl::UniformMatrix4fv(transform_loc, 100, gl::FALSE, transform.value.as_ptr());

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
