use gl::types::*;
use scop::graphics::camera::Camera;
use scop::graphics::gl_wrapper::*;
use scop::graphics::shaders::Shader;
use scop::graphics::texture::Texture;
use scop::graphics::window::Window;
use scop::math::matrix::Matrix4;
use scop::math::vec3::Vec3;
use scop::parsers::obj_parser::ObjFile;
use std::env;
use std::f32::consts::PI;
use std::ffi::CString;
use std::mem;
use std::path::Path;
use std::ptr;
extern crate glfw;
use self::glfw::Key;

fn main() -> Result<(), String> {
    if env::args().size_hint().0 < 2 {
        return Err("Missing argument".to_string());
    }
    let arg = env::args().nth(1).unwrap_or_default();
    let obj_path: &Path = Path::new(&arg);
    // let mut obj: ObjFile;
    if let Some(mut obj) = ObjFile::new(&obj_path) {
        println!("Obj vertex : {}", obj.vertex.len());
        let model = obj.modelise();
        for msh in &model.meshes {
            println!("Mesh: {}\n\tnb vertice: {}", msh.name, msh.indice.len());
        }
        // let vertices: &[GLfloat] = &model.meshes[0].mesh_to_slice();

        // let indices: &[u32] = model.meshes[0].indice.as_slice();
        // let mut window = Window::new(1920, 1080, &obj.name);
        // //INIT OpenGL
        // window.init_gl();

        // // let tex: Vec<Texture> = vec![Texture::new(Path::new("./resources/oui.bmp"))];
        // // let mut text: GLuint = 0;
        // // unsafe {
        // //     gl::GenTextures(1, &mut text);
        // //     gl::BindTexture(gl::TEXTURE_2D, text);
        // //     gl::TexImage2D(
        // //         gl::TEXTURE_2D,
        // //         0,
        // //         gl::BGRA as i32,
        // //         tex[0].width,
        // //         tex[0].height,
        // //         0,
        // //         gl::RGBA,
        // //         gl::UNSIGNED_BYTE,
        // //         tex[0].data.as_slice().as_ptr() as *const _,
        // //     );
        // // }
        // // let v_oui: Vec<Vertex> = vec![Vertex::new()];
        // // let n_oui: Vec<Normal>;

        // // let mesh: Mesh = Mesh::new(v_oui, n_oui, tex);

        // let mut shaders = Shader::new(
        //     "./src/graphics/shaders_files/vertex.vert",
        //     "./src/graphics/shaders_files/fragment.frag",
        // );

        // let vao = Vao::new();
        // vao.bind();

        // let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
        // vbo.bind();
        // // vbo.store_f32_data(&vertices);

        // let ebo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
        // ebo.bind();
        // ebo.store_u32_data(&indices);

        // let position_attribute = VertexAttribute::new(
        //     0,
        //     3,
        //     gl::FLOAT,
        //     gl::FALSE,
        //     6 * mem::size_of::<GLfloat>() as GLsizei,
        //     ptr::null(),
        // );

        // position_attribute.enable();

        // // let tex_attribute = VertexAttribute::new(
        // //     1,
        // //     2,
        // //     gl::FLOAT,
        // //     gl::FALSE,
        // //     2 * mem::size_of::<GLfloat>() as GLsizei,
        // //     ptr::null(),
        // // );
        // // tex_attribute.enable();
        // // unsafe {
        // //     gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        // //     gl::Enable(gl::BLEND);
        // // }
        // // let normal_attribute = VertexAttribute::new(
        // //     2,
        // //     3,
        // //     gl::FLOAT,
        // //     gl::FALSE,
        // //     8 * mem::size_of::<GLfloat>() as GLsizei,
        // //     ptr::null(),
        // // );
        // // normal_attribute.enable();
        // let color_attribute = VertexAttribute::new(
        //     1,
        //     3,
        //     gl::FLOAT,
        //     gl::FALSE,
        //     6 * mem::size_of::<GLfloat>() as GLsizei,
        //     (3 * mem::size_of::<GLfloat>()) as *const _,
        // );
        // color_attribute.enable();
        // // dbg!((indices.len()));

        // let mut time = window.get_time();
        // let mut yaw: f32 = 0.0;
        // let mut pitch: f32 = 0.0;
        // let mut roll: f32 = 0.0;
        // let scaling: f32 = 0.5;
        // let mut render_type = gl::LINE;
        // let mut camera: Camera = Camera::new();
        // camera.set_pos(Vec3 {
        //     x: 0.0,
        //     y: 0.0,
        //     z: 100.0,
        // });
        // let proj: Matrix4<f32> = Matrix4::<f32>::perspective(PI / 2.0, 800.0 / 600.0, 0.1, 100.0);
        // unsafe {
        //     gl::Enable(gl::DEPTH_TEST);
        // }
        // while !window.should_close() {
        //     unsafe {
        //         gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        //         gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        //         // gl::BindTexture(gl::TEXTURE_2D, text);
        //         // vao.bind();
        //         gl::PolygonMode(gl::FRONT_AND_BACK, render_type);
        //         camera.move_cam(
        //             Vec3 {
        //                 x: 0.0,
        //                 y: 0.0,
        //                 z: 10.0,
        //             },
        //             Vec3 {
        //                 x: pitch,
        //                 y: yaw,
        //                 z: roll,
        //             },
        //         );
        //         shaders.set_matrix4(&CString::new("projection").unwrap(), proj.clone());
        //         shaders.set_matrix4(&CString::new("view").unwrap(), camera.view.clone());
        //         let mut transform: Matrix4<f32> = Matrix4::identity();
        //         // transform.translate(0.0, 0.0, 0.0);
        //         transform = transform.scale(scaling);
        //         transform.rotate(0.0, window.get_time() as f32, 0.0);
        //         shaders.set_matrix4(&CString::new("transform").unwrap(), transform);
        //         shaders.use_prog();
        //         gl::DrawElements(
        //             gl::TRIANGLES,
        //             indices.len() as GLsizei,
        //             gl::UNSIGNED_INT,
        //             ptr::null(),
        //         );
        //         time = window.update_title(time);
        //     }
        //     window.update();
        //     if *(window.keys_state.get(&Key::S).unwrap()) {
        //         pitch -= 2.0;
        //         if pitch < -89.0 {
        //             pitch = -89.0;
        //         }
        //     }
        //     if *(window.keys_state.get(&Key::W).unwrap()) {
        //         pitch += 2.0;
        //         if pitch > 89.0 {
        //             pitch = 89.0;
        //         }
        //     }
        //     if *(window.keys_state.get(&Key::D).unwrap()) {
        //         yaw += 2.0;
        //     }
        //     if *(window.keys_state.get(&Key::A).unwrap()) {
        //         yaw -= 2.0;
        //     }
        //     if *(window.keys_state.get(&Key::E).unwrap()) {
        //         roll -= 2.0;
        //     }
        //     if *(window.keys_state.get(&Key::Q).unwrap()) {
        //         roll += 2.0;
        //     }
        //     if *(window.keys_state.get(&Key::V).unwrap()) {
        //         render_type = gl::POINT;
        //     }
        //     if *(window.keys_state.get(&Key::B).unwrap()) {
        //         render_type = gl::LINE;
        //     }
        //     if *(window.keys_state.get(&Key::N).unwrap()) {
        //         render_type = gl::FILL;
        //     }
        //     if *(window.keys_state.get(&Key::R).unwrap()) {
        //         roll = 0.0;
        //         pitch = 0.0;
        //         yaw = 0.0;
        //     }
        // }
    }
    Ok(())
}
