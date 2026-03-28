use glow::*;
use std::mem;
use super::context::Context;

pub struct Graphics {
    pub shader_program: Program,
    pub vao: VertexArray,
    pub vbo: Buffer,
    pub ebo: Buffer,
    pub index_count: i32,
}

impl Graphics {
    pub fn new(gl: &Context) -> Self {
        let gl = &gl.gl;
        // Create shader program
        let shader_program = unsafe {
            let program = gl.create_program().expect("Cannot create program");

            let vertex_shader_source = r#"
                #version 330 core
                layout (location = 0) in vec3 aPos;
                uniform mat4 model;
                uniform mat4 view;
                uniform mat4 projection;
                void main() {
                    gl_Position = projection * view * model * vec4(aPos, 1.0);
                }
            "#;

            let fragment_shader_source = r#"
                #version 330 core
                out vec4 FragColor;
                uniform vec3 color;
                void main() {
                    FragColor = vec4(color, 1.0);
                }
            "#;

            let shader_sources = [
                (VERTEX_SHADER, vertex_shader_source),
                (FRAGMENT_SHADER, fragment_shader_source),
            ];

            let mut shaders = Vec::with_capacity(shader_sources.len());

            for (shader_type, source) in shader_sources.iter() {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                gl.shader_source(shader, source);
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    panic!("{}", gl.get_shader_info_log(shader));
                }
                gl.attach_shader(program, shader);
                shaders.push(shader);
            }

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            program
        };

        // Generate sphere geometry
        let (vertices, indices) = generate_sphere(32, 16);
        let index_count = indices.len() as i32;

        // Create and bind VAO, VBO, EBO
        let (vao, vbo, ebo) = unsafe {
            let vao = gl.create_vertex_array().expect("Cannot create vertex array");
            let vbo = gl.create_buffer().expect("Cannot create vertex buffer");
            let ebo = gl.create_buffer().expect("Cannot create element buffer");

            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(
                ARRAY_BUFFER,
                bytemuck::cast_slice(&vertices),
                STATIC_DRAW,
            );

            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(&indices),
                STATIC_DRAW,
            );

            gl.vertex_attrib_pointer_f32(
                0,
                3,
                FLOAT,
                false,
                3 * mem::size_of::<f32>() as i32,
                0,
            );
            gl.enable_vertex_attrib_array(0);

            (vao, vbo, ebo)
        };

        Self {
            shader_program,
            vao,
            vbo,
            ebo,
            index_count,
        }
    }
}

// Generate sphere geometry
fn generate_sphere(stacks: u32, sectors: u32) -> (Vec<f32>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let pi = std::f32::consts::PI;
    let stack_step = pi / stacks as f32;
    let sector_step = 2.0 * pi / sectors as f32;

    // Generate vertices
    for i in 0..=stacks {
        let stack_angle = pi / 2.0 - i as f32 * stack_step;
        let xy = stack_angle.cos();
        let z = stack_angle.sin();

        for j in 0..=sectors {
            let sector_angle = j as f32 * sector_step;
            let x = xy * sector_angle.cos();
            let y = xy * sector_angle.sin();

            vertices.extend_from_slice(&[x, y, z]);
        }
    }

    // Generate indices
    for i in 0..stacks {
        let k1 = i * (sectors + 1);
        let k2 = k1 + sectors + 1;

        for j in 0..sectors {
            if i != 0 {
                indices.extend_from_slice(&[k1 + j, k2 + j, k1 + j + 1]);
            }
            if i != stacks - 1 {
                indices.extend_from_slice(&[k1 + j + 1, k2 + j, k2 + j + 1]);
            }
        }
    }

    (vertices, indices)
}
