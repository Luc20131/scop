mod parser;
use gl::types::*;
use glfw::MouseButton;
use scop::graphics::camera::Camera;
use scop::graphics::gl_wrapper::*;
use scop::graphics::shaders::Shader;
use scop::graphics::window::Window;
use scop::my_lib::bmp_parser::image_loader;
use scop::my_lib::matrice::Matrix4;
use scop::my_lib::vec3::Vec3;
use std::env;
use std::ffi::CString;
use std::fs;
use std::mem;
use std::ptr;
use std::str::Lines;
extern crate glfw;
use self::glfw::Key;

fn main() {
    println!(
        "Loading {}...",
        env::args().nth(1).expect("Failed to read first argument")
    );
    let mut obj: parser::Obj = parser::Obj::new();
    let content = fs::read_to_string(env::args().nth(1).unwrap()).unwrap();
    let lines: Lines = content.lines();
    for line in lines {
        obj.add_data(line);
    }
    obj.build_indices();
    obj.print_data();
    image_loader("./resources/oui.bmp");
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

    let mut time = window.get_time();
    let mut y_angle: f32 = 0.0;
    let mut x_angle: f32 = 0.0;
    let mut z_angle: f32 = 0.0;
    let scaling: f32 = 0.1;
    let mut render_type = gl::LINE;
    let mut camera: Camera = Camera::new();
    // let mut view = camera.set_pos(Vec3 {
    //     x: -500.0,
    //     y: 0.0,
    //     z: 0.0,
    // });
    // println!("View : {:?}", view);
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::PolygonMode(gl::FRONT_AND_BACK, render_type);
            let view = camera.move_cam(Vec3 {
                x: 0.01,
                y: 0.0,
                z: 0.0,
            });
            shaders.set_matrix4(&CString::new("view").unwrap(), view);
            let mut transform: Matrix4<f32> = Matrix4::identity();
            transform.translate(0.0, 0.0, 0.0);
            transform = transform.scale(scaling);
            // transform.rotate(x_angle, y_angle, z_angle);
            shaders.set_matrix4(&CString::new("transform").unwrap(), transform);
            shaders.use_prog();
            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as GLsizei,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
            time = window.update_title(time);
        }
        window.update();
        if *(window.keys_state.get(&Key::A).unwrap()) {
            y_angle -= 0.02;
        }
        if *(window.keys_state.get(&Key::D).unwrap()) {
            y_angle += 0.02;
        }
        if *(window.keys_state.get(&Key::W).unwrap()) {
            x_angle -= 0.02;
        }
        if *(window.keys_state.get(&Key::S).unwrap()) {
            x_angle += 0.02;
        }
        if *(window.keys_state.get(&Key::E).unwrap()) {
            z_angle -= 0.02;
        }
        if *(window.keys_state.get(&Key::Q).unwrap()) {
            z_angle += 0.02;
        }
        if *(window.keys_state.get(&Key::V).unwrap()) {
            render_type = gl::POINT;
        }
        if *(window.keys_state.get(&Key::B).unwrap()) {
            render_type = gl::LINE;
        }
        if *(window.keys_state.get(&Key::N).unwrap()) {
            render_type = gl::FILL;
        }
        if *(window.keys_state.get(&Key::R).unwrap()) {
            z_angle = 0.0;
            x_angle = 0.0;
            y_angle = 0.0;
        }
        if *(window
            .mouse_state
            .mouse_buttons
            .get(&MouseButton::Left)
            .unwrap())
        {}
    }
}
