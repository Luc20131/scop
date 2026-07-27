use std::{mem, ptr};

use gl::TEXTURE0;
use gl::types::{GLfloat, GLsizei};

use crate::graphics::gl_wrapper::{BufferObject, Vao, VertexAttribute};
use crate::graphics::shaders::Shader;
use crate::graphics::texture::{self, Texture};
use crate::math::vec3::Vec3;
use crate::parsers::mtl_parser::Material;

pub type Normal = Vec3;
pub type TexCoord = (f32, f32);
pub type Face = Vec<FaceElem>;

#[derive(Debug, Clone)]
pub struct FaceElem {
    pub vertex: u32,
    pub tex_coord: Option<u32>,
    pub normals: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct Mesh {
    pub name: String,
    pub textures: Vec<Texture>,
    pub material: Material,
    pub indices: Vec<u32>,
    pub vertices: Vec<f32>,
    pub faces: Vec<Face>,
    pub smoothing: u8,
    vao: Vao,
    vbo: BufferObject,
    ebo: BufferObject,
}

impl Default for Mesh {
    fn default() -> Self {
        Self {
            name: "Undefined".to_string(),
            indices: vec![],
            vertices: vec![],
            textures: vec![],
            faces: vec![],
            smoothing: 0,
            material: Material::default(),
            vao: Vao::new(),
            vbo: BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW),
            ebo: BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW),
        }
    }
}

impl Mesh {
    pub fn new(name: String, texture: Vec<Texture>, smoothing: u8) -> Self {
        Self {
            name,
            indices: vec![],
            vertices: vec![],
            textures: texture,
            faces: vec![],
            smoothing,
            material: Material::default(),
            vao: Vao::new(),
            vbo: BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW),
            ebo: BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW),
        }
    }

    pub fn setup_mesh(&mut self) {
        if self.faces.is_empty() {
            return;
        }
        println!("setup mesh: {}", self.name);
        self.vao.bind();
        self.vbo.bind();
        self.vbo.store_f32_data(self.vertices.as_slice());
        self.ebo.bind();
        self.ebo.store_u32_data(self.indices.as_slice());

        let position_attribute = VertexAttribute::new(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            9 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        position_attribute.enable();

        let tex_attribute = VertexAttribute::new(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            9 * mem::size_of::<GLfloat>() as GLsizei,
            (3 * mem::size_of::<GLfloat>()) as *const _,
        );
        tex_attribute.enable();

        let normal_attribute = VertexAttribute::new(
            2,
            3,
            gl::FLOAT,
            gl::FALSE,
            9 * mem::size_of::<GLfloat>() as GLsizei,
            (5 * mem::size_of::<GLfloat>()) as *const _,
        );
        normal_attribute.enable();

        let color_attribute = VertexAttribute::new(
            3,
            1,
            gl::FLOAT,
            gl::FALSE,
            9 * mem::size_of::<GLfloat>() as GLsizei,
            (8 * mem::size_of::<GLfloat>()) as *const _,
        );
        color_attribute.enable();
        self.vao.unbind();
    }

    pub fn draw(&self, render_type: u32, shader: &mut Shader, cam_pos: Vec3) {
        shader.use_prog();
        self.vao.bind();
        self.vbo.bind();
        shader.set_float("light.linear", 0.007);
        shader.set_float("light.quadratic", 0.000007);
        shader.set_float("light.ambientStrength", self.material.ambient_color().red);
        shader.set_float("light.specularStrength", self.material.specular_color().red);
        shader.set_vec3("viewPos", &cam_pos.as_array());

        unsafe {
            if toggle_texture_mode(false) {
                shader.set_int("aTexMode", 1);

                // for (tex_counter, texture) in (0_i32..).zip(self.textures.iter()) {

                // match texture.type_.as_str() {
                //     "ambient" => {
                if self.material.map_kd.id != 0 {
                    let tex_id: u32 = gl::TEXTURE0 + self.material.map_kd.id;
                    gl::ActiveTexture(tex_id);
                    gl::BindTexture(gl::TEXTURE_2D, self.material.map_kd.id);
                    shader.set_int("diffuseTexture", self.material.map_kd.id as i32);
                }
                if self.material.map_d.id != 0 {
                    let tex_id: u32 = gl::TEXTURE0 + self.material.map_d.id;
                    gl::ActiveTexture(tex_id);
                    gl::BindTexture(gl::TEXTURE_2D, self.material.map_d.id);
                    shader.set_int("has_alpha_tex", 1);
                    shader.set_int("alphaTexture", self.material.map_d.id as i32);
                } else {
                    shader.set_int("has_alpha_tex", 0);
                }
                if self.material.map_bump.id != 0 {
                    let tex_id: u32 = gl::TEXTURE0 + self.material.map_bump.id;
                    gl::ActiveTexture(tex_id);
                    gl::BindTexture(gl::TEXTURE_2D, self.material.map_bump.id);
                    shader.set_int("has_normal_tex", 1);
                    shader.set_int("normalTexture", self.material.map_bump.id as i32);
                } else {
                    shader.set_int("has_normal_tex", 0);
                }

                // "alpha" => {
                // shader.set_int("alphaTexture", tex_counter);
                // }
                // _ => {}
                // }
            } else {
                shader.set_int("aTexMode", 0);
            }
            gl::PolygonMode(gl::FRONT_AND_BACK, render_type);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as GLsizei,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
        self.vbo.unbind();
        self.vao.unbind();
    }
}

pub fn toggle_texture_mode(toggle: bool) -> bool {
    static mut TEXTURE_MODE: bool = false;
    unsafe {
        if toggle {
            TEXTURE_MODE = !TEXTURE_MODE;
        }
        TEXTURE_MODE
    }
}
