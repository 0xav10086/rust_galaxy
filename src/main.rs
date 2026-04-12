mod constants;
mod graphics;
mod physics;
mod ui;
mod utils;

use glow::HasContext;
use winit::event::{Event, WindowEvent, ElementState, MouseButton, MouseScrollDelta};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::keyboard::PhysicalKey;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use std::num::NonZeroU32;

use graphics::{Shader, Camera, Renderer};
use physics::{Object, GravityCalculator, CollisionDetector};
use ui::InputHandler;
use utils::GridGenerator;
use constants::*;

const VERTEX_SHADER_SOURCE: &str = r"#version 330 core
layout(location=0) in vec3 aPos;
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
void main() {
    gl_Position = projection * view * model * vec4(aPos, 1.0);
}";

const FRAGMENT_SHADER_SOURCE: &str = r"#version 330 core
out vec4 FragColor;
uniform vec4 objectColor;
void main() {
    FragColor = objectColor;
}";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    
    let event_loop = EventLoop::new()?;
    
    let window_builder = WindowBuilder::new()
        .with_title("3D Gravity Simulation")
        .with_inner_size(winit::dpi::LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT));
    
    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8);
    
    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));
    let (window, gl_config) = display_builder.build(
        &event_loop,
        template,
        |configs| {
            configs.reduce(|accum, config| {
                if config.num_samples() > 0 && accum.num_samples() == 0 {
                    config
                } else {
                    accum
                }
            }).unwrap()
        }
    )?;
    
    let window = window.ok_or("Failed to create window")?;
    let raw_window_handle = HasRawWindowHandle::raw_window_handle(&window);

    let (width, height): (u32, u32) = window.inner_size().into();
    let attrs = glutin::surface::SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new().build(
        raw_window_handle,
        NonZeroU32::new(width).ok_or("Width is 0")?,
        NonZeroU32::new(height).ok_or("Height is 0")?,
    );

    let surface = unsafe {
        gl_config.display().create_window_surface(&gl_config, &attrs)?
    };
    
    let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(Some(raw_window_handle));
    
    let not_current_context = unsafe {
        gl_config.display().create_context(&gl_config, &context_attributes)
            .or_else(|_| gl_config.display().create_context(&gl_config, &fallback_context_attributes))?
    };
    
    let context = not_current_context.make_current(&surface)?;
    
    let gl = unsafe { glow::Context::from_loader_function(|s| {
        let c_str = std::ffi::CString::new(s).unwrap();
        context.display().get_proc_address(&c_str)
    }) };
    
    unsafe {
        gl.enable(glow::DEPTH_TEST);
        gl.enable(glow::BLEND);
        gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
        gl.viewport(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
    }
    
    let shader = Shader::new(&gl, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
    shader.use_program(&gl);
    
    let projection = glam::Mat4::perspective_rh(
        std::f32::consts::FRAC_PI_4,
        SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32,
        0.1,
        750000.0,
    );
    shader.set_uniform_mat4(&gl, "projection", &projection);
    
    let mut camera = Camera::new(
        glam::Vec3::new(0.0, 5000.0, 15000.0), // Closer camera
        glam::Vec3::new(0.0, 1.0, 0.0),
    );
    
    let mut renderer = Renderer::new(gl);
    
    let mut objects = vec![
        Object::new(
            renderer.gl(),
            glam::Vec3::new(3844.0, 0.0, 0.0),
            glam::Vec3::new(0.0, 0.0, 1000.0), // Standard orbital speed for this visual scale
            7.34767309e22,
            3344.0,
            glam::Vec4::new(1.0, 0.0, 0.0, 1.0), // Red Moon
        ),
        Object::new(
            renderer.gl(),
            glam::Vec3::ZERO,
            glam::Vec3::ZERO,
            5.97219e24,
            5515.0,
            glam::Vec4::new(0.0, 0.0, 1.0, 1.0), // Blue Earth
        ),
    ];
    
    let mut input_handler = InputHandler::new();
    let mut last_frame_time = std::time::Instant::now();
    let start_time = std::time::Instant::now();
    let mut frame_count = 0;
    let mut last_fps_update = std::time::Instant::now();
    
    event_loop.run(move |event, event_loop_window_target| {
        event_loop_window_target.set_control_flow(ControlFlow::Poll);
        
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    event_loop_window_target.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    if physical_size.width > 0 && physical_size.height > 0 {
                        surface.resize(
                            &context,
                            NonZeroU32::new(physical_size.width).unwrap(),
                            NonZeroU32::new(physical_size.height).unwrap(),
                        );
                        unsafe {
                            renderer.gl().viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
                        }
                    }
                }
                
                WindowEvent::RedrawRequested => {
                    let delta_time = last_frame_time.elapsed().as_secs_f32();
                    last_frame_time = std::time::Instant::now();
                    
                    frame_count += 1;
                    let elapsed_fps = last_fps_update.elapsed().as_secs_f32();
                    if elapsed_fps >= 1.0 {
                        let current_fps = frame_count as f32 / elapsed_fps;
                        frame_count = 0;
                        last_fps_update = std::time::Instant::now();
                        
                        println!("--- Debug Info ---");
                        println!("Runtime: {:.2}s | FPS: {:.2}", start_time.elapsed().as_secs_f32(), current_fps);
                        
                        let view = camera.get_view_matrix();
                        let pv = projection * view;
                        
                        for (i, obj) in objects.iter().enumerate() {
                            let clip_pos = pv * obj.position.extend(1.0);
                            let is_visible = if clip_pos.w > 0.0 {
                                let ndc = clip_pos.truncate() / clip_pos.w;
                                ndc.x.abs() <= 1.0 && ndc.y.abs() <= 1.0 && ndc.z >= -1.0 && ndc.z <= 1.0
                            } else {
                                false
                            };
                            
                            println!("Object {}: Pos: {:?} | Visible: {}", i, obj.position, is_visible);
                        }
                    }
                    
                    unsafe {
                        renderer.gl().clear_color(0.0, 0.0, 0.0, 1.0);
                        renderer.gl().clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                    }
                    
                    shader.set_uniform_mat4(renderer.gl(), "view", &camera.get_view_matrix());
                    
                    let objects_len = objects.len();
                    for i in 0..objects_len {
                        for j in 0..objects_len {
                            if i != j {
                                let acc = GravityCalculator::calculate_acceleration(&objects[i], &objects[j]);
                                if !input_handler.pause {
                                    objects[i].apply_acceleration(acc, delta_time);
                                }
                                let collision_coef = CollisionDetector::check_collision(&objects[i], &objects[j]);
                                if collision_coef < 1.0 {
                                    let relative_pos = objects[j].position - objects[i].position;
                                    let relative_vel = objects[j].velocity - objects[i].velocity;
                                    if relative_vel.dot(relative_pos) < 0.0 {
                                        objects[i].velocity *= collision_coef;
                                    }
                                }
                            }
                        }
                        
                        if objects[i].initializing {
                            objects[i].update_radius();
                            objects[i].update_vertices(renderer.gl());
                        }
                        
                        if !input_handler.pause {
                            objects[i].update_position(delta_time);
                        }
                        
                        let scale_vec = glam::Vec3::splat(objects[i].radius);
                        let model = glam::Mat4::from_translation(objects[i].position) * glam::Mat4::from_scale(scale_vec);
                        renderer.draw_object(&shader, &objects[i], &model);
                    }
                    
                    let grid_vertices = GridGenerator::create_grid_vertices(40000.0, &objects);
                    renderer.update_grid_vertices(&grid_vertices);
                    renderer.draw_grid(&shader, &grid_vertices);
                    
                    surface.swap_buffers(&context).unwrap();
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if let PhysicalKey::Code(key) = event.physical_key {
                        // Use a fixed or more reliable delta for input processing to ensure movement is felt
                        let input_delta = 1.0 / 60.0; 
                        input_handler.process_keyboard(
                            key,
                            event.state,
                            &mut camera,
                            input_delta,
                            CAMERA_SPEED,
                        );
                        
                        if !objects.is_empty() {
                            input_handler.process_object_movement(
                                key,
                                event.state,
                                objects.last_mut().unwrap(),
                                false,
                            );
                        }
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    camera.process_mouse_movement(position.x as f32, position.y as f32, MOUSE_SENSITIVITY);
                }
                WindowEvent::MouseWheel { delta, .. } => {
                    let delta_time = last_frame_time.elapsed().as_secs_f32();
                    let camera_speed = 50000.0 * delta_time;
                    match delta {
                        MouseScrollDelta::LineDelta(_, y) => {
                            if y > 0.0 {
                                camera.process_keyboard(graphics::camera::CameraMovement::Forward, delta_time, camera_speed);
                            } else if y < 0.0 {
                                camera.process_keyboard(graphics::camera::CameraMovement::Backward, delta_time, camera_speed);
                            }
                        }
                        MouseScrollDelta::PixelDelta(pos) => {
                            let y = pos.y as f32;
                            if y > 0.0 {
                                camera.process_keyboard(graphics::camera::CameraMovement::Forward, delta_time, camera_speed);
                            } else if y < 0.0 {
                                camera.process_keyboard(graphics::camera::CameraMovement::Backward, delta_time, camera_speed);
                            }
                        }
                    }
                }
                WindowEvent::MouseInput { button, state, .. } => {
                    if button == MouseButton::Left && state == ElementState::Pressed {
                        objects.push(Object::new(
                            renderer.gl(),
                            glam::Vec3::ZERO,
                            glam::Vec3::ZERO,
                            INIT_MASS,
                            3344.0,
                            glam::Vec4::new(0.0, 0.0, 1.0, 1.0),
                        ));
                        if let Some(last) = objects.last_mut() {
                            last.initializing = true;
                        }
                    } else if button == MouseButton::Left && state == ElementState::Released {
                        if let Some(last) = objects.last_mut() {
                            last.initializing = false;
                            last.launched = true;
                        }
                    }
                }
                _ => {}
            },
            // Event::AboutToWait => {
            //     let delta_time = last_frame_time.elapsed().as_secs_f32();
            //     last_frame_time = std::time::Instant::now();
                
            //     unsafe {
            //         renderer.gl().clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            //     }
                
            //     shader.set_uniform_mat4(renderer.gl(), "view", &camera.get_view_matrix());
                
            //     let objects_len = objects.len();
            //     for i in 0..objects_len {
            //         for j in 0..objects_len {
            //             if i != j {
            //                 let acc = GravityCalculator::calculate_acceleration(&objects[i], &objects[j]);
            //                 if !input_handler.pause {
            //                     objects[i].apply_acceleration(acc, delta_time);
            //                 }
            //                 let collision_coef = CollisionDetector::check_collision(&objects[i], &objects[j]);
            //                 objects[i].velocity *= collision_coef;
            //             }
            //         }
                    
            //         if objects[i].initializing {
            //             objects[i].update_radius();
            //             objects[i].update_vertices(renderer.gl());
            //         }
                    
            //         if !input_handler.pause {
            //             objects[i].update_position(delta_time);
            //         }
                    
            //         let model = glam::Mat4::from_translation(objects[i].position);
            //         renderer.draw_object(&shader, &objects[i], &model);
            //     }
                
            //     let grid_vertices = GridGenerator::create_grid_vertices(&objects);
            //     renderer.update_grid_vertices(&grid_vertices);
            //     renderer.draw_grid(&shader, &grid_vertices);
                
            //     window.request_redraw();
            //     surface.swap_buffers(&context).unwrap();
            // }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}
