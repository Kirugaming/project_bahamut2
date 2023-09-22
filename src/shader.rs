use std::ffi::CString;
use std::fs;
use std::ptr::null;
use gl::types::GLenum;

struct Shader {
    program_id: u32
}

impl Shader {
    unsafe fn new(vert_source: &str, frag_source : &str) -> Shader {
        // load glsl code from text file
        let vert_code = CString::new(fs::read_to_string(vert_source)
                                         .expect("Vertex file failed to be read.")).unwrap();

        let frag_code = CString::new(fs::read_to_string(frag_source)
                                        .expect("Fragment file failed to be read.")).unwrap();


        // compile glsl code
        let vertex = Shader::compile_shader(gl::VERTEX_SHADER, vert_code);
        let fragment = Shader::compile_shader(gl::FRAGMENT_SHADER, frag_code);
        // TODO: compile errors

        // shader program
        let program_id : u32 = gl::CreateProgram();

        gl::AttachShader(program_id, vertex);
        gl::AttachShader(program_id, fragment);
        gl::LinkProgram(program_id);
        // TODO: program errors

        gl::DeleteShader(vertex);
        gl::DeleteShader(fragment);

        return Shader {program_id}
    }

    unsafe fn compile_shader(shader_type: GLenum, code : CString) -> u32 {
        let shader = gl::CreateShader(shader_type);
        gl::ShaderSource(shader, 1, &code.as_ptr(), null());
        gl::CompileShader(shader);

        return shader;
    }
 }

impl Shader {
    unsafe fn set_int(&self, name : &str, value : i32) {
        let c_str = CString::new(name).unwrap();
        gl::Uniform1i(gl::GetUniformLocation(self.program_id, c_str.as_ptr()), value);
    }

}