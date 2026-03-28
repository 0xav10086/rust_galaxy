use glam::{Vec3, Vec4};
use glow::HasContext;
use crate::constants::PI;
use crate::graphics::renderer::Vertex;

pub struct Object {
    pub position: Vec3,
    pub velocity: Vec3,
    pub mass: f32,
    pub density: f32,
    pub radius: f32,
    pub color: Vec4,
    pub vao: glow::VertexArray,
    pub vbo: glow::Buffer,
    pub vertex_count: usize,
    pub initializing: bool,
    pub launched: bool,
}

impl Object {
    pub fn new(
        gl: &glow::Context,
        position: Vec3,
        velocity: Vec3,
        mass: f32,
        density: f32,
        color: Vec4,
    ) -> Self {
        let radius = Self::calculate_radius(mass, density);
        let vertices = Self::generate_sphere_vertices(radius);
        let vertex_count = vertices.len();
        
        unsafe {
            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();
            
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&vertices),
                glow::STATIC_DRAW,
            );
            
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, std::mem::size_of::<Vertex>() as i32, 0);
            
            gl.bind_vertex_array(None);
            
            Object {
                position,
                velocity,
                mass,
                density,
                radius,
                color,
                vao,
                vbo,
                vertex_count,
                initializing: false,
                launched: false,
            }
        }
    }
    
    fn calculate_radius(mass: f32, density: f32) -> f32 {
        let volume = mass / density;
        let radius = (3.0 * volume / (4.0 * PI)).powf(1.0 / 3.0);
        radius / 100000.0
    }
    
    fn generate_sphere_vertices(radius: f32) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        let stacks = 10;
        let sectors = 10;
        
        for i in 0..=stacks {
            let theta1 = (i as f32 / stacks as f32) * PI;
            let theta2 = ((i + 1) as f32 / stacks as f32) * PI;
            
            for j in 0..sectors {
                let phi1 = (j as f32 / sectors as f32) * 2.0 * PI;
                let phi2 = ((j + 1) as f32 / sectors as f32) * 2.0 * PI;
                
                let v1 = Self::spherical_to_cartesian(radius, theta1, phi1);
                let v2 = Self::spherical_to_cartesian(radius, theta1, phi2);
                let v3 = Self::spherical_to_cartesian(radius, theta2, phi1);
                let v4 = Self::spherical_to_cartesian(radius, theta2, phi2);
                
                vertices.push(Vertex { position: v1.to_array() });
                vertices.push(Vertex { position: v2.to_array() });
                vertices.push(Vertex { position: v3.to_array() });
                vertices.push(Vertex { position: v2.to_array() });
                vertices.push(Vertex { position: v4.to_array() });
                vertices.push(Vertex { position: v3.to_array() });
            }
        }
        
        vertices
    }
    
    fn spherical_to_cartesian(r: f32, theta: f32, phi: f32) -> Vec3 {
        Vec3::new(
            r * theta.sin() * phi.cos(),
            r * theta.cos(),
            r * theta.sin() * phi.sin(),
        )
    }
    
    pub fn update_position(&mut self, _delta_time: f32) {
        self.position += self.velocity / crate::constants::TIME_STEP;
    }
    
    pub fn apply_acceleration(&mut self, acceleration: Vec3, _delta_time: f32) {
        self.velocity += acceleration / crate::constants::ACC_STEP;
    }
    
    pub fn update_radius(&mut self) {
        self.radius = Self::calculate_radius(self.mass, self.density);
    }
    
    pub fn update_vertices(&self, gl: &glow::Context) {
        let vertices = Self::generate_sphere_vertices(self.radius);
        unsafe {
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&vertices),
                glow::DYNAMIC_DRAW,
            );
        }
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        // OpenGL 资源需要在合适的上下文中删除，这里简化处理
    }
}