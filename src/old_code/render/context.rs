use winit::event_loop::EventLoop;
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextAttributesBuilder, PossiblyCurrentContext},
    prelude::*,
    surface::{Surface, WindowSurface},
    display::GetGlDisplay,
};
use glutin_winit::{DisplayBuilder, GlWindow};
use glow::Context as GlowContext;
use glow::HasContext;

// [新增] 屏蔽 winit 推荐使用 0.6 版本句柄的警告
#[allow(deprecated)]
use winit::raw_window_handle::HasRawWindowHandle; 
use std::ffi::CString;

pub struct Context {
    pub gl: GlowContext, 
    pub window: winit::window::Window,
    pub context: PossiblyCurrentContext,
    pub surface: Surface<WindowSurface>,
}

impl Context {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window_builder = winit::window::WindowBuilder::new()
            .with_title("Rust Galaxy");

        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(true);

        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

        let (window, gl_config) = display_builder
            .build(event_loop, template, |mut configs| {
                configs.reduce(|accum, config| {
                    if config.supports_transparency().unwrap_or(false) && !accum.supports_transparency().unwrap_or(false) {
                        config
                    } else {
                        accum
                    }
                }).expect("No config found")
            })
            .expect("Failed to create display");

        let window = window.expect("Failed to create window");
        
        let display = gl_config.display(); 

        let surface_attributes = window.build_surface_attributes(Default::default());
        let surface = unsafe {
            display.create_window_surface(&gl_config, &surface_attributes)
                .expect("Failed to create GL surface")
        };

        // [修改] 添加 .unwrap() 提取真实的句柄，并屏蔽弃用警告
        #[allow(deprecated)]
        let raw_handle = window.raw_window_handle().unwrap();

        let context_attributes = ContextAttributesBuilder::new()
            .with_profile(glutin::context::GlProfile::Core)
            .with_context_api(glutin::context::ContextApi::OpenGl(Some(glutin::context::Version {
                major: 3,
                minor: 3,
            })))
            .build(Some(raw_handle)); // 传入解包后的句柄

        let context = unsafe {
            display.create_context(&gl_config, &context_attributes)
                .expect("Failed to create GL context")
        };

        let context = context.make_current(&surface).expect("Failed to make context current");

        let gl = unsafe {
            GlowContext::from_loader_function(|s| {
                let c_str = CString::new(s).unwrap();
                display.get_proc_address(&c_str) as *const _
            })
        };

        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
        }

        Self {
            gl,
            window,
            context,
            surface,
        }
    }
}