use glow::HasContext;

pub struct Shader {
    program: glow::Program,
}

impl Shader {
    pub fn new(gl: &glow::Context, vertex_source: &str, fragment_source: &str) -> Result<Self, String> {
        unsafe {
            let vertex_shader = Self::compile_shader(gl, glow::VERTEX_SHADER, vertex_source)?;
            let fragment_shader = Self::compile_shader(gl, glow::FRAGMENT_SHADER, fragment_source)?;
            
            let program = gl.create_program().map_err(|e| format!("Failed to create program: {}", e))?;
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            
            if !gl.get_program_link_status(program) {
                let log = gl.get_program_info_log(program);
                return Err(format!("Program linking failed: {}", log));
            }
            
            gl.delete_shader(vertex_shader);
            gl.delete_shader(fragment_shader);
            
            Ok(Shader { program })
        }
    }
    
    unsafe fn compile_shader(gl: &glow::Context, shader_type: u32, source: &str) -> Result<glow::Shader, String> {
        let shader = gl.create_shader(shader_type).map_err(|e| format!("Failed to create shader: {}", e))?;
        // 直接传递 &str，不需要 CString
        gl.shader_source(shader, source);
        gl.compile_shader(shader);
        
        if !gl.get_shader_compile_status(shader) {
            let log = gl.get_shader_info_log(shader);
            return Err(format!("Shader compilation failed: {}", log));
        }
        
        Ok(shader)
    }
    
    pub fn use_program(&self, gl: &glow::Context) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }
    
    pub fn set_uniform_mat4(&self, gl: &glow::Context, name: &str, matrix: &glam::Mat4) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_matrix_4_f32_slice(location.as_ref(), false, matrix.as_ref());
        }
    }
    
    pub fn set_uniform_vec4(&self, gl: &glow::Context, name: &str, vec: &glam::Vec4) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_4_f32(location.as_ref(), vec.x, vec.y, vec.z, vec.w);
        }
    }
    
    pub fn get_uniform_location(&self, gl: &glow::Context, name: &str) -> Option<glow::UniformLocation> {
        unsafe {
            gl.get_uniform_location(self.program, name)
        }
    }
}
