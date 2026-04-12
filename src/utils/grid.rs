use glam::Vec3;
use crate::graphics::renderer::Vertex;
use crate::physics::object::Object;
use crate::constants::{GRID_DIVISIONS, G, C};

pub struct GridGenerator;

impl GridGenerator {
    pub fn create_grid_vertices(size: f32, objs: &[Object]) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        let divisions = GRID_DIVISIONS;
        let step = size / divisions as f32;
        let half_size = size / 2.0;
        
        // Generate X-axis lines
        for _ in 0..1 {
            let base_y = -50.0; // Base floor below spheres (which are at y=0)
            for z_step in 0..=divisions {
                let z = -half_size + z_step as f32 * step;
                for x_step in 0..divisions {
                    let x_start = -half_size + x_step as f32 * step;
                    let x_end = x_start + step;
                    
                    vertices.push(Vertex { position: [x_start, base_y, z] });
                    vertices.push(Vertex { position: [x_end, base_y, z] });
                }
            }
        }
        
        // Generate Z-axis lines
        for _ in 0..1 {
            let base_y = -50.0;
            for x_step in 0..=divisions {
                let x = -half_size + x_step as f32 * step;
                for z_step in 0..divisions {
                    let z_start = -half_size + z_step as f32 * step;
                    let z_end = z_start + step;
                    
                    vertices.push(Vertex { position: [x, base_y, z_start] });
                    vertices.push(Vertex { position: [x, base_y, z_end] });
                }
            }
        }
        
        // Apply gravity displacement
        Self::apply_gravity_displacement(&mut vertices, objs);
        
        vertices
    }
    
    fn apply_gravity_displacement(vertices: &mut [Vertex], objs: &[Object]) {
        for vertex in vertices.iter_mut() {
            let mut total_displacement = 0.0;
            let vertex_pos = Vec3::new(vertex.position[0], 0.0, vertex.position[2]); // Calculate dist from y=0
            
            for obj in objs {
                let to_object = Vec3::new(obj.position.x, 0.0, obj.position.z) - vertex_pos;
                let distance = to_object.length();
                let distance_m = distance * 100_000.0;
                let rs = (2.0 * G as f32 * obj.mass) / (C * C);
                
                let term = rs * (distance_m - rs);
                if term > 0.0 {
                    let z = 2.0 * term.sqrt() * 100.0;
                    total_displacement += z;
                }
            }
            
            // Push DOWN (well effect)
            vertex.position[1] = vertex.position[1] - (total_displacement / 10.0);
        }
    }
}
