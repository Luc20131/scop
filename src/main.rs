use gl::ClampColor;
use scop::graphics::camera::Camera;
use scop::graphics::light::Light;
use scop::graphics::shaders::Shader;
use scop::graphics::texture::Texture;
use scop::graphics::window::Window;
use scop::math::matrix::Matrix4;
use scop::math::vec3::Vec3;
use scop::parsers::obj_parser::ObjFile;
use std::env;
use std::f32::consts::PI;
use std::ffi::CString;
use std::path::Path;

extern crate glfw;
use self::glfw::Key;

fn main() -> Result<(), String> {
    if env::args().size_hint().0 < 2 {
        return Err("Missing argument".to_string());
    }
    let arg = env::args().nth(1).unwrap_or_default();

    let mut window = Window::new(1920, 1080, "scop");
    window.init_gl();

    let obj_path: &Path = Path::new(&arg);
    if let Some(obj) = ObjFile::new(&obj_path) {
        let mut light_shader = Shader::new(
            "./src/graphics/shaders_files/light.vert",
            "./src/graphics/shaders_files/light.frag",
        );
        let mut light = Light::new(light_shader.clone());

        println!(
            "Obj vertex : {}\nMtl files nb: {}",
            obj.vertex.len(),
            obj.mtl_files.len()
        );

        let mut model = obj.model;
        for msh in &model.meshes {
            println!(
                "Mesh: {}\n\tnb vertice: {}\n\tnb indices: {}\n\tmaterial: {}",
                msh.name,
                msh.vertices.len(),
                msh.indices.len(),
                msh.material.name
            );
        }

        let mut shaders = Shader::new(
            "./src/graphics/shaders_files/vertex.vert",
            "./src/graphics/shaders_files/fragment.frag",
        );

        let mut time = window.get_time();
        let scaling: f32 = 5.0;
        let mut camera: Camera = Camera::new();
        camera.set_pos(Vec3 {
            x: 0.0,
            y: 50.0,
            z: 100.0,
        });

        let proj: Matrix4<f32> = Matrix4::<f32>::perspective(PI / 2.0, 800.0 / 600.0, 0.1, 3000.0);
        let mut tex = Texture::new(Path::new("resources/texture.bmp"));
        tex.setup_tex();

        light_shader.use_prog();
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
        }
        while !window.should_close() {
            unsafe {
                input_checker(&window, &mut camera);
                camera.update_camera_pos();
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                // shaders.use_prog();

                let mut transform: Matrix4<f32> = Matrix4::identity();
                transform.translate(0.0, 0.0, 0.0);
                transform = transform.scale(scaling);
                // transform.rotate(0.0, window.get_time() as f32, 0.0);
                // shaders.set_matrix4("projection", proj.clone());
                // shaders.set_matrix4("view", camera.view.clone());
                // shaders.set_matrix4("transform", transform.clone());

                light_shader.use_prog();
                light_shader.set_matrix4("projection", proj.clone());
                light_shader.set_matrix4("view", camera.view.clone());
                light_shader.set_matrix4("transform", transform);

                if *(window.keys_state.get(&Key::V).unwrap()) {
                    model.render_type = gl::POINT;
                }
                if *(window.keys_state.get(&Key::B).unwrap()) {
                    model.render_type = gl::LINE;
                }
                if *(window.keys_state.get(&Key::N).unwrap()) {
                    model.render_type = gl::FILL;
                }
                model.draw(&mut light_shader, camera.pos.clone());
                // light.draw();
                time = window.update_title(time);
            }
            window.update();
        }
    }
    Ok(())
}

fn input_checker(window: &Window, camera: &mut Camera) {
    // if *(window.keys_state.get(&Key::Left).unwrap()) {
    //     camera.pitch += 2.0;
    //     if camera.pitch > 89.0 {
    //         camera.pitch = 89.0;
    //     }
    // }
    // if *(window.keys_state.get(&Key::Right).unwrap()) {
    //     camera.pitch -= 2.0;
    //     if camera.pitch < -89.0 {
    //         camera.pitch = -89.0;
    //     }
    // }
    if *(window.keys_state.get(&Key::S).unwrap()) {
        camera.pos.z += 2.0;
    }
    if *(window.keys_state.get(&Key::W).unwrap()) {
        camera.pos.z -= 2.0;
    }
    if *(window.keys_state.get(&Key::D).unwrap()) {
        camera.pos.x += 2.0;
    }
    if *(window.keys_state.get(&Key::A).unwrap()) {
        camera.pos.x -= 2.0;
    }
    if *(window.keys_state.get(&Key::Right).unwrap()) {
        camera.yaw -= 1.0;
    }
    if *(window.keys_state.get(&Key::Left).unwrap()) {
        camera.yaw += 1.0;
    }
    if *(window.keys_state.get(&Key::E).unwrap()) {
        camera.roll -= 2.0;
    }
    if *(window.keys_state.get(&Key::Q).unwrap()) {
        camera.roll += 2.0;
    }
    if *(window.keys_state.get(&Key::R).unwrap()) {
        camera.roll = 0.0;
        camera.pitch = 0.0;
        camera.yaw = 0.0;
    }
    // if *(window.keys_state.get(&Key::T).unwrap()) {
    //     toggle_texture_mode(true);
    // }
}
