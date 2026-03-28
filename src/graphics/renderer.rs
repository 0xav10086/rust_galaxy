use glow::HasContext;
use glam::Mat4;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

pub struct Renderer {
    gl: glow::Context,  // 改为 pub 或者提供访问方法，这里改为 pub 以便外部使用
    grid_vao: glow::VertexArray,
    grid_vbo: glow::Buffer,
    grid_vertex_count: usize,
}

impl Renderer {
    pub fn new(gl: glow::Context) -> Self {
        unsafe {
            let grid_vao = gl.create_vertex_array().unwrap();
            let grid_vbo = gl.create_buffer().unwrap();
            
            Renderer {
                gl,
                grid_vao,
                grid_vbo,
                grid_vertex_count: 0,
            }
        }
    }
    
    // 提供 gl 的访问方法
    pub fn gl(&self) -> &glow::Context {
        &self.gl
    }
    
    pub fn create_vbo_vao(&self, vertices: &[Vertex]) -> (glow::VertexArray, glow::Buffer) {
        unsafe {
            let vao = self.gl.create_vertex_array().unwrap();
            let vbo = self.gl.create_buffer().unwrap();
            
            self.gl.bind_vertex_array(Some(vao));
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(vertices),
                glow::STATIC_DRAW,
            );
            
            self.gl.enable_vertex_attrib_array(0);
            self.gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, std::mem::size_of::<Vertex>() as i32, 0);
            
            self.gl.bind_vertex_array(None);
            (vao, vbo)
        }
    }
    
    pub fn update_vbo(&self, vbo: glow::Buffer, vertices: &[Vertex]) {
        unsafe {
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(vertices),
                glow::DYNAMIC_DRAW,
            );
        }
    }
    
    pub fn draw_object(&self, shader: &super::shader::Shader, object: &crate::physics::object::Object, model_matrix: &Mat4) {
        unsafe {
            shader.set_uniform_mat4(&self.gl, "model", model_matrix);
            shader.set_uniform_vec4(&self.gl, "objectColor", &object.color);
            
            self.gl.bind_vertex_array(Some(object.vao));
            self.gl.draw_arrays(glow::TRIANGLES, 0, object.vertex_count as i32);
        }
    }
    
    pub fn draw_grid(&self, shader: &super::shader::Shader, vertices: &[Vertex]) {
        unsafe {
            shader.set_uniform_mat4(&self.gl, "model", &Mat4::IDENTITY);
            self.update_vbo(self.grid_vbo, vertices);
            
            self.gl.bind_vertex_array(Some(self.grid_vao));
            self.gl.draw_arrays(glow::LINES, 0, vertices.len() as i32);
        }
    }
    
    pub fn update_grid_vertices(&mut self, vertices: &[Vertex]) {
        self.grid_vertex_count = vertices.len();
        self.update_vbo(self.grid_vbo, vertices);
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            // 直接传对象，不需要 Some
            self.gl.delete_vertex_array(self.grid_vao);
            self.gl.delete_buffer(self.grid_vbo);
        }
    }
}