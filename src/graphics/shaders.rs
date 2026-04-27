pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new() {}
}

// const VERT_SHADER: &str = r#"#version 330 core
//   layout (location = 0) in vec3 pos;
//   void main() {
//     gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
//   }
// "#;

// unsafe {
//     let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
//     assert_ne!(vertex_shader, 0);
//     gl::ShaderSource(
//         vertex_shader,
//         1,
//         &(VERT_SHADER.as_bytes().as_ptr().cast()),
//         &(VERT_SHADER.len().try_into().unwrap()),
//     );
//     gl::CompileShader(vertex_shader);
//     let mut success = 0;
//     gl::GetShaderiv(vertex_shader, COMPILE_STATUS, &mut success);
//     if success == 0 {
//         let mut v: Vec<u8> = Vec::with_capacity(1024);
//         let mut log_len = 0_i32;
//         gl::GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
//         v.set_len(log_len.try_into().unwrap());
//         panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
//     }

//     const FRAG_SHADER: &str = r#"#version 330 core
//       out vec4 final_color;

//       void main() {
//         final_color = vec4(1.0, 0.5, 0.2, 1.0);
//       }
//     "#;

//     let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
//     assert_ne!(fragment_shader, 0);
//     gl::ShaderSource(
//         fragment_shader,
//         1,
//         &(FRAG_SHADER.as_bytes().as_ptr().cast()),
//         &(FRAG_SHADER.len().try_into().unwrap()),
//     );
//     gl::CompileShader(fragment_shader);
//     let mut success = 0;
//     gl::GetShaderiv(fragment_shader, COMPILE_STATUS, &mut success);
//     if success == 0 {
//         let mut v: Vec<u8> = Vec::with_capacity(1024);
//         let mut log_len = 0_i32;
//         gl::GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
//         v.set_len(log_len.try_into().unwrap());
//         panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
//     }

//     let shader_program = gl::CreateProgram();
//     gl::AttachShader(shader_program, vertex_shader);
//     gl::AttachShader(shader_program, fragment_shader);
//     gl::LinkProgram(shader_program);
//     let mut success = 0;
//     gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
//     if success == 0 {
//         let mut v: Vec<u8> = Vec::with_capacity(1024);
//         let mut log_len = 0_i32;
//         gl::GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
//         v.set_len(log_len.try_into().unwrap());
//         panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
//     }
//     gl::DeleteShader(vertex_shader);
//     gl::DeleteShader(fragment_shader);
//     let p: *const c_char = null_str!("uni_color").as_ptr().cast();
//     uni_color_loc = gl::GetUniformLocation(shader_program, p);
// }
