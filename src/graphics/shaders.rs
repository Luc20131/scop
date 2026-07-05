use crate::math::matrix::Matrix4;
use gl::types::{GLchar, GLenum, GLint, GLuint};
use gl::{self, FRAGMENT_SHADER, VERTEX_SHADER};
use std::ffi::CString;
use std::{fs, ptr};

#[derive(Debug, Clone)]
pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, frag_path: &str) -> Shader {
        let mut id: u32 = 0;
        let vertex_code: String =
            fs::read_to_string(vertex_path).expect("Failed to open vertex shader file !");
        let fragment_code: String =
            fs::read_to_string(frag_path).expect("Failed to open fragment shader file !");
        unsafe {
            let vertex = compile_shader(vertex_code, VERTEX_SHADER);
            match vertex {
                Ok(vertex) => {
                    let fragment = compile_shader(fragment_code, FRAGMENT_SHADER);
                    match fragment {
                        Ok(fragment) => {
                            id = gl::CreateProgram();
                            gl::AttachShader(id, vertex);
                            gl::AttachShader(id, fragment);
                            gl::LinkProgram(id);
                            gl::DeleteShader(vertex);
                            gl::DeleteShader(fragment);
                        }
                        Err(error_log) => {
                            eprintln!("Fragment shader compilation error: {}", error_log);
                        }
                    }
                }
                Err(error_log) => {
                    eprintln!("Vertex shader compilation error: {}", error_log);
                }
            }
        }
        Shader { id }
    }

    pub fn use_prog(&mut self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_int(&mut self, name: &str, value: i32) {
        let name_formated = &CString::new(name).unwrap();
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, name_formated.as_ptr()),
                value,
            );
        }
    }

    pub fn set_float(&mut self, name: &str, value: f32) {
        let name_formated = &CString::new(name).unwrap();
        unsafe {
            gl::Uniform1f(
                gl::GetUniformLocation(self.id, name_formated.as_ptr()),
                value,
            );
        }
    }

    pub fn set_vec3(&mut self, name: &str, value: &[f32]) {
        let name_formated = &CString::new(name).unwrap();
        unsafe {
            gl::Uniform3fv(
                gl::GetUniformLocation(self.id, name_formated.as_ptr()),
                1,
                value.as_ptr(),
            );
        }
    }

    pub fn set_matrix4(&mut self, name: &str, matrix: Matrix4<f32>) {
        let name_formated = &CString::new(name).unwrap();
        unsafe {
            let matrix_loc = gl::GetUniformLocation(self.id, name_formated.as_ptr());
            gl::UniformMatrix4fv(matrix_loc, 1, gl::TRUE, matrix.value.as_ptr());
        }
    }
}

fn compile_shader(shader_code: String, _type: GLenum) -> Result<GLuint, String> {
    let shader = unsafe { gl::CreateShader(_type) };
    let v_shader_code = CString::new(shader_code.as_bytes()).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &v_shader_code.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
    }

    let mut success: GLint = 0;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    }

    if success == gl::TRUE as GLint {
        return Ok(shader);
    }

    let mut log_len: GLint = 0;
    unsafe {
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_len);
    }
    let mut buffer = vec![0u8; log_len as usize];
    unsafe {
        gl::GetShaderInfoLog(
            shader,
            log_len,
            ptr::null_mut(),
            buffer.as_mut_ptr() as *mut GLchar,
        );
        gl::DeleteShader(shader);
    }
    let log = String::from_utf8_lossy(&buffer).to_string();
    Err(log)
}
