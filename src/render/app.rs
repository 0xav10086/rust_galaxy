use winit::{
    event::{Event, WindowEvent, ElementState},
    keyboard::{KeyCode, PhysicalKey},
    event_loop::{ControlFlow, EventLoop},
};
use glow::*;
use glutin::prelude::*;
use crate::galaxy::CelestialBody;
use crate::camera::Camera;
use glm::{ext::*, *};
use super::context::Context;
use super::graphics::Graphics;
use glow::HasContext;

pub struct App {
    context: Context,
    graphics: Graphics,
}

fn mat4_to_slice(mat: &glm::Mat4) -> &[f32] {
    unsafe {
        std::slice::from_raw_parts(mat as *const _ as *const f32, 16)
    }
}

impl App {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let context = Context::new(event_loop);
        let graphics = Graphics::new(&context);
        Self { context, graphics }
    }

    pub fn run(self, event_loop: EventLoop<()>, bodies: Vec<CelestialBody>, mut camera: Camera) {
        let App { context, graphics } = self;
        let Context { gl, window, context: gl_context, surface } = context;
        let Graphics { shader_program, vao, vbo: _vbo, ebo: _ebo, index_count } = graphics;

        event_loop.run(move |event, elwt| {
            elwt.set_control_flow(ControlFlow::Poll);

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::Resized(size) => {
                        unsafe {
                            gl.viewport(0, 0, size.width as i32, size.height as i32);
                        }
                    }
                    WindowEvent::KeyboardInput { event: key_event, .. } => {
                        if key_event.state == ElementState::Pressed {
                            if let PhysicalKey::Code(key) = key_event.physical_key {
                                let speed = 0.1; // 相机移动速度
                                match key {
                                    KeyCode::KeyW => camera.position[2] -= speed,
                                    KeyCode::KeyS => camera.position[2] += speed,
                                    KeyCode::KeyA => camera.position[0] -= speed,
                                    KeyCode::KeyD => camera.position[0] += speed,
                                    _ => (),
                                }
                            }
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        unsafe {
                            gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);

                            // Set view and projection matrices
                            let (width, height) = {
                                let size = window.inner_size();
                                (size.width as f32, size.height as f32)
                            };

                            let aspect = width / height;
                            let projection = perspective(
                                radians(45.0),
                                aspect,
                                0.1,
                                100.0
                            );

                            let eye = vec3(
                                camera.position[0] as f32 / 1e10,
                                camera.position[1] as f32 / 1e10,
                                camera.position[2] as f32 / 1e10
                            );

                            let center = vec3(
                                camera.focus[0] as f32 / 1e10,
                                camera.focus[1] as f32 / 1e10,
                                camera.focus[2] as f32 / 1e10
                            );

                            let up = vec3(
                                camera.up[0] as f32,
                                camera.up[1] as f32,
                                camera.up[2] as f32
                            );

                            let view = look_at(eye, center, up);

                            gl.use_program(Some(shader_program));

                            // Set projection and view matrices
                            gl.uniform_matrix_4_f32_slice(
                                gl.get_uniform_location(shader_program, "projection").as_ref(),
                                false,
                                mat4_to_slice(&projection)
                            );

                            gl.uniform_matrix_4_f32_slice(
                                gl.get_uniform_location(shader_program, "view").as_ref(),
                                false,
                                mat4_to_slice(&view)
                            );

                            // Render each celestial body
                            for body in &bodies {
                                let position = vec3(
                                    body.position[0] as f32 / 1e10,
                                    body.position[1] as f32 / 1e10,
                                    body.position[2] as f32 / 1e10
                                );

                                let radius = (body.radius / 1e8) as f32;

                                let identity = glm::mat4(
                                    1.0, 0.0, 0.0, 0.0,
                                    0.0, 1.0, 0.0, 0.0,
                                    0.0, 0.0, 1.0, 0.0,
                                    0.0, 0.0, 0.0, 1.0
                                );

                                let model = translate(&identity, position) *
                                    scale(&identity, vec3(radius, radius, radius));

                                gl.uniform_matrix_4_f32_slice(
                                    gl.get_uniform_location(shader_program, "model").as_ref(),
                                    false,
                                    mat4_to_slice(&model)
                                );

                                gl.uniform_3_f32(
                                    gl.get_uniform_location(shader_program, "color").as_ref(),
                                    body.color[0] as f32,
                                    body.color[1] as f32,
                                    body.color[2] as f32
                                );

                                // Draw sphere
                                gl.bind_vertex_array(Some(vao));
                                gl.draw_elements(
                                    glow::TRIANGLES,
                                    index_count,
                                    glow::UNSIGNED_INT,
                                    0,
                                );
                            }

                            surface.swap_buffers(&gl_context).expect("Failed to swap buffers");
                        }
                    }
                    _ => (),
                },
                Event::NewEvents(_) => {
                    window.request_redraw();
                },
                _ => (),
            }
        });
    }
}
