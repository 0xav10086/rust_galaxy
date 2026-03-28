mod constants;
mod graphics;
mod physics;
mod ui;
mod utils;

use glow::HasContext;
use winit::event::{Event, WindowEvent, ElementState, MouseButton, MouseScrollDelta};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit::keyboard::{KeyCode, PhysicalKey};
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin_winit::DisplayBuilder;
use winit::raw_window_handle::HasRawWindowHandle;

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
    
    // 创建事件循环
    let event_loop = EventLoop::new()?;
    
    // 创建窗口
    let window_builder = WindowBuilder::new()
        .with_title("3D Gravity Simulation")
        .with_inner_size(winit::dpi::LogicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT));
    
    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .build();
    
    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));
    let (window, gl_config) = display_builder.build(
    event_loop.event_loop_window_target(),
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
).unwrap();
    
    let window = window.unwrap();
    
    // 创建 surface
    let raw_window_handle = window.raw_window_handle();
    let surface_attributes = glutin::surface::SurfaceAttributesBuilder::<glutin::surface::WindowSurface>::new()
        .build(raw_window_handle, Default::default());
    let surface = unsafe {
        glutin::surface::Surface::new(&gl_config.display(), gl_config, surface_attributes).unwrap()
    };
    
    // 创建 context
    let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(Some(raw_window_handle));
    
    let not_current_context = unsafe {
        gl_config.display().create_context(&gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_config.display().create_context(&gl_config, &fallback_context_attributes).unwrap()
            })
    };
    
    // 绑定 context 到 surface
    let context = not_current_context.make_current(&surface).unwrap();
    
    // 初始化 OpenGL
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
    
    // 创建着色器
    let shader = Shader::new(&gl, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)?;
    shader.use_program(&gl);
    
    // 设置投影矩阵
    let projection = glam::Mat4::perspective_rh(
        std::f32::consts::FRAC_PI_4,
        SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32,
        0.1,
        750000.0,
    );
    shader.set_uniform_mat4(&gl, "projection", &projection);
    
    // 相机
    let mut camera = Camera::new(
        glam::Vec3::new(0.0, 1000.0, 5000.0),
        glam::Vec3::new(0.0, 1.0, 0.0),
    );
    
    // 渲染器
    let mut renderer = Renderer::new(gl);
    
    // 物体
    let mut objects = vec![
        Object::new(
            renderer.gl(),
            glam::Vec3::new(3844.0, 0.0, 0.0),
            glam::Vec3::new(0.0, 0.0, 228.0),
            7.34767309e22,
            3344.0,
            glam::Vec4::new(1.0, 0.0, 0.0, 1.0),
        ),
        Object::new(
            renderer.gl(),
            glam::Vec3::ZERO,
            glam::Vec3::ZERO,
            5.97219e24,
            5515.0,
            glam::Vec4::new(0.0, 1.0, 0.0, 1.0),
        ),
    ];
    
    let mut input_handler = InputHandler::new();
    let mut last_frame_time = std::time::Instant::now();
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if let PhysicalKey::Code(key) = event.physical_key {
                        let delta_time = last_frame_time.elapsed().as_secs_f32();
                        input_handler.process_keyboard(
                            key,
                            event.state,
                            &mut camera,
                            delta_time,
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
            Event::AboutToWait => {
                let delta_time = last_frame_time.elapsed().as_secs_f32();
                last_frame_time = std::time::Instant::now();
                
                unsafe {
                    renderer.gl().clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
                }
                
                // 更新相机视图
                shader.set_uniform_mat4(renderer.gl(), "view", &camera.get_view_matrix());
                
                // 物理更新
                let objects_len = objects.len();
                for i in 0..objects_len {
                    for j in 0..objects_len {
                        if i != j {
                            let acc = GravityCalculator::calculate_acceleration(&objects[i], &objects[j]);
                            if !input_handler.pause {
                                objects[i].apply_acceleration(acc, delta_time);
                            }
                            let collision_coef = CollisionDetector::check_collision(&objects[i], &objects[j]);
                            objects[i].velocity *= collision_coef;
                        }
                    }
                    
                    if objects[i].initializing {
                        objects[i].update_radius();
                        objects[i].update_vertices(renderer.gl());
                    }
                    
                    if !input_handler.pause {
                        objects[i].update_position(delta_time);
                    }
                    
                    // 绘制物体
                    let model = glam::Mat4::from_translation(objects[i].position);
                    renderer.draw_object(&shader, &objects[i], &model);
                }
                
                // 生成并绘制网格
                let grid_vertices = GridGenerator::create_grid_vertices(&objects);
                renderer.update_grid_vertices(&grid_vertices);
                renderer.draw_grid(&shader, &grid_vertices);
                
                window.request_redraw();
            }
            _ => {}
        }
        event_loop_window_target.set_control_flow(ControlFlow::Poll);
    })?;
}