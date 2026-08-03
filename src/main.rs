use scop::graphics::camera::Camera;
use scop::graphics::light::Light;
use scop::graphics::shaders::Shader;
use scop::graphics::window::Window;
use scop::math::matrix::Matrix4;
use scop::math::vec3::Vec3;
use scop::parsers::mtl_parser::RGB;
use scop::parsers::obj_parser::ObjFile;
use std::env;
use std::f32::consts::PI;
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
    if let Some(obj) = ObjFile::new(obj_path) {
        let mut light_shader = Shader::new(
            "./src/graphics/shaders_files/light.vert",
            "./src/graphics/shaders_files/light.frag",
        );
        let mut shaders = Shader::new(
            "./src/graphics/shaders_files/vertex.vert",
            "./src/graphics/shaders_files/fragment.frag",
        );
        let _light_obj_shader = Shader::new(
            "./src/graphics/shaders_files/light_obj.vert",
            "./src/graphics/shaders_files/light_obj.frag",
        );
        let mut light = Light::new(shaders.clone());
        light.change_light_color(RGB {
            red: 0.7,
            green: 0.7,
            blue: 0.7,
        });
        println!(
            "Obj vertex : {}\nMtl files nb: {}",
            obj.vertex.len(),
            obj.mtl_files.len()
        );

        let mut model = obj.model;
        // for msh in &model.meshes {
        //     println!(
        //         "Mesh: {}\n\tnb vertice: {}\n\tnb indices: {}\n\tmaterial: {}",
        //         msh.name,
        //         msh.vertices.len(),
        //         msh.indices.len(),
        //         msh.material.name
        //     );
        // }

        let mut time = window.get_time();
        let scaling: f32 = 0.1;
        let mut camera: Camera = Camera::new();
        camera.set_pos(Vec3 {
            x: 0.0,
            y: 10.0,
            z: 50.0,
        });

        let mut proj: Matrix4<f32>;
        // let mut tex = Texture::new(Path::new("resources/cathedral/base_diff.jpg"));
        // tex.setup_tex();
        light_shader.use_prog();
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::BLEND);
            // gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::BlendFuncSeparate(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA, gl::ONE, gl::ZERO);
            // gl::Enable(gl::CULL_FACE);
            // gl::CullFace(gl::FRONT);
            // gl::FrontFace(gl::CW);
        }
        let mut delta_time; // Time between current frame and last frame
        let mut last_frame = 0.0; // Time of last frame
        let mut _index: usize = 0;
        while !window.should_close() {
            let (w_width, w_height) = window.get_size();
            unsafe {
                proj = Matrix4::<f32>::perspective(
                    PI / 2.0,
                    w_width as f32 / w_height as f32,
                    0.1,
                    1000.0,
                );
                let current_frame: f32 = window.get_time() as f32;
                delta_time = current_frame - last_frame;
                last_frame = current_frame;

                input_checker(&window, &mut camera, &mut light, delta_time);
                camera.update_camera_pos();
                gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                let mut transform: Matrix4<f32> = Matrix4::identity();
                transform.translate(0.0, 0.0, 0.0);
                let mut light_transform = transform.clone();
                transform = transform.scale(0.1);
                light_transform = light_transform.scale(scaling);
                light_shader.use_prog();
                light_shader.set_vec3("lightColor", light.color().rbg_to_array().as_slice());
                light_shader.set_vec3("light.position", light.pos.as_array().as_slice());
                light_shader.set_matrix4("projection", proj.clone());
                light_shader.set_matrix4("view", camera.view.clone());
                light_shader.set_matrix4("transform", light_transform.clone());

                if *(window.keys_state.get(&Key::V).unwrap()) {
                    model.render_type = gl::POINT;
                }
                if *(window.keys_state.get(&Key::B).unwrap()) {
                    model.render_type = gl::LINE;
                }
                if *(window.keys_state.get(&Key::N).unwrap()) {
                    model.render_type = gl::FILL;
                }
                if *(window.keys_state.get(&Key::KpAdd).unwrap()) {
                    window.keys_state.entry(Key::KpAdd).insert_entry(false);
                    _index += 1;
                }
                if *(window.keys_state.get(&Key::KpSubtract).unwrap()) {
                    window.keys_state.entry(Key::KpSubtract).insert_entry(false);
                    _index -= 1;
                }
                // light.draw();
                model.draw(&mut light_shader, camera.pos.clone());

                shaders.use_prog();
                transform.scale(0.001);
                transform.translate(light.pos.x, light.pos.y, light.pos.z);
                shaders.set_matrix4("view", camera.view.clone());
                shaders.set_matrix4("transform", transform.clone());
                shaders.set_matrix4("projection", proj.clone());
                light.draw(camera.pos.clone());

                time = window.update_title(time);
            }
            window.update();
        }
    }
    Ok(())
}

fn input_checker(window: &Window, camera: &mut Camera, light: &mut Light, delta_time: f32) {
    let camera_speed = 10.0 * delta_time;
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
    if *(window.keys_state.get(&Key::Q).unwrap()) {
        light.pos.x -= camera_speed * 10.0;
    }
    if *(window.keys_state.get(&Key::E).unwrap()) {
        light.pos.x += camera_speed * 10.0;
    }
    if *(window.keys_state.get(&Key::S).unwrap()) {
        camera.pos.z += camera_speed;
    }
    if *(window.keys_state.get(&Key::W).unwrap()) {
        camera.pos.z -= camera_speed;
    }
    if *(window.keys_state.get(&Key::LeftShift).unwrap()) {
        camera.pos.y -= camera_speed;
    }
    if *(window.keys_state.get(&Key::Space).unwrap()) {
        camera.pos.y += camera_speed;
    }
    if *(window.keys_state.get(&Key::D).unwrap()) {
        camera.pos.x += camera_speed;
    }
    if *(window.keys_state.get(&Key::A).unwrap()) {
        camera.pos.x -= camera_speed;
    }
    if *(window.keys_state.get(&Key::Right).unwrap()) {
        camera.yaw -= camera_speed * 10.0;
    }
    if *(window.keys_state.get(&Key::Left).unwrap()) {
        camera.yaw += camera_speed * 10.0;
    }
    // if *(window.keys_state.get(&Key::E).unwrap()) {
    //     camera.pitch -= camera_speed * 10.0;
    // }
    // if *(window.keys_state.get(&Key::Q).unwrap()) {
    //     camera.pitch += camera_speed * 10.0;
    // }
    if *(window.keys_state.get(&Key::R).unwrap()) {
        camera.roll = 0.0;
        camera.pitch = 0.0;
        camera.yaw = 0.0;
    }
    // if *(window.keys_state.get(&Key::T).unwrap()) {
    //     toggle_texture_mode(true);
    // }
}
