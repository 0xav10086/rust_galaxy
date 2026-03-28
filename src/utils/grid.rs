use glam::Vec3;
use crate::graphics::renderer::Vertex;
use crate::physics::object::Object;
use crate::constants::{GRID_SIZE, GRID_DIVISIONS, G, C};

pub struct GridGenerator;

impl GridGenerator {
    pub fn create_grid_vertices(objs: &[Object]) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        let size = GRID_SIZE;
        let divisions = GRID_DIVISIONS;
        let step = size / divisions as f32;
        let half_size = size / 2.0;
        
        // 生成 X 轴方向的线
        for y_step in 3..=3 {
            let y = -half_size * 0.3 + y_step as f32 * step;
            for z_step in 0..=divisions {
                let z = -half_size + z_step as f32 * step;
                for x_step in 0..divisions {
                    let x_start = -half_size + x_step as f32 * step;
                    let x_end = x_start + step;
                    
                    vertices.push(Vertex { position: [x_start, y, z] });
                    vertices.push(Vertex { position: [x_end, y, z] });
                }
            }
        }
        
        // 生成 Z 轴方向的线
        for x_step in 0..=divisions {
            let x = -half_size + x_step as f32 * step;
            for y_step in 3..=3 {
                let y = -half_size * 0.3 + y_step as f32 * step;
                for z_step in 0..divisions {
                    let z_start = -half_size + z_step as f32 * step;
                    let z_end = z_start + step;
                    
                    vertices.push(Vertex { position: [x, y, z_start] });
                    vertices.push(Vertex { position: [x, y, z_end] });
                }
            }
        }
        
        // 应用引力位移效果
        Self::apply_gravity_displacement(&mut vertices, objs);
        
        vertices
    }
    
    fn apply_gravity_displacement(vertices: &mut [Vertex], objs: &[Object]) {
        for vertex in vertices.iter_mut() {
            let mut total_displacement = 0.0;
            let vertex_pos = Vec3::new(vertex.position[0], vertex.position[1], vertex.position[2]);
            
            for obj in objs {
                let to_object = obj.position - vertex_pos;
                let distance = to_object.length();
                let distance_m = distance * 1000.0;
                let rs = (2.0 * G as f32 * obj.mass) / (C * C);
                let z = 2.0 * (rs * (distance_m - rs)).sqrt() * 100.0;
                total_displacement += z;
            }
            
            vertex.position[1] = vertex.position[1] / 15.0 - 3000.0;
        }
    }
}