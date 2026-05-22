use crate::my_lib::matrice::Matrix4;
use gl::{self};
use std::ffi::CString;
use std::fs;

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, frag_path: &str) -> Shader {
        let id: u32;
        let vertex_code: String =
            fs::read_to_string(vertex_path).expect("Failed to open vertex shader file !");
        let fragment_code: String =
            fs::read_to_string(frag_path).expect("Failed to open fragment shader file !");
        unsafe {
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            let v_shader_code = CString::new(vertex_code.as_bytes()).unwrap();
            gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex);
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            let f_shader_code = CString::new(fragment_code.as_bytes()).unwrap();
            gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), std::ptr::null());
            gl::CompileShader(fragment);
            id = gl::CreateProgram();
            gl::AttachShader(id, vertex);
            gl::AttachShader(id, fragment);
            gl::LinkProgram(id);

            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
        }
        Shader { id }
    }

    pub fn use_prog(&mut self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_int(&mut self, name: &CString, value: i32) {
        unsafe {
            gl::Uniform1i(gl::GetUniformLocation(self.id, name.as_ptr()), value);
        }
    }
    pub fn set_float(&mut self, name: &CString, value: f32) {
        unsafe {
            gl::Uniform1f(gl::GetUniformLocation(self.id, name.as_ptr()), value);
        }
    }

    pub fn set_matrix4(&mut self, name: &CString, matrix: Matrix4<f32>) {
        unsafe {
            let matrix_loc = gl::GetUniformLocation(self.id, name.as_ptr());
            gl::UniformMatrix4fv(matrix_loc, 1, gl::FALSE, matrix.value.as_ptr());
        }
    }
}
