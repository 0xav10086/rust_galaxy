use winit::{
    event_loop::EventLoop,
};
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextAttributesBuilder, PossiblyCurrentContext},
    prelude::*,
    surface::{Surface, WindowSurface},
    display::DisplayApiPreference, // Corrected import for DisplayApiPreference
};
use glow::Context as GlowContext;
use glow::HasContext;
use winit::raw_window_handle::{HasWindowHandle, HasDisplayHandle}; // Corrected import path
use std::num::NonZeroU32;
use std::ffi::CString;

pub struct Context {
    pub gl: GlowContext, // Corrected type name
    pub window: winit::window::Window,
    pub context: PossiblyCurrentContext,
    pub surface: Surface<WindowSurface>,
}

impl Context {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = winit::window::WindowBuilder::new()
            .with_title("Rust Galaxy")
            .build(event_loop)
            .expect("Failed to create window");
        
        let display_handle = window.display_handle().expect("Failed to get display handle");
        let window_handle = window.window_handle().expect("Failed to get window handle");
        
        let display = unsafe {
            glutin::display::Display::new(display_handle.as_raw(), DisplayApiPreference::None) // Corrected path
                .expect("Failed to create display")
        };
        
        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(true)
            .build();

        let gl_config = unsafe {
            display.find_configs(template)
                .expect("Failed to find configs")
                .reduce(|accum, config| {
                    if config.supports_transparency().unwrap_or(false) && !accum.supports_transparency().unwrap_or(false) {
                        config
                    } else {
                        accum
                    }
                })
                .expect("No config found")
        };

        let (width, height) = window.inner_size().into();
        let width = NonZeroU32::new(width).unwrap_or(NonZeroU32::new(1).unwrap());
        let height = NonZeroU32::new(height).unwrap_or(NonZeroU32::new(1).unwrap());

        let surface_attributes = glutin::surface::SurfaceAttributesBuilder::<WindowSurface>::new()
            .build(window_handle.as_raw(), width, height);

        let surface = unsafe {
            display.create_window_surface(&gl_config, &surface_attributes)
                .expect("Failed to create GL surface")
        };

        let context_attributes = ContextAttributesBuilder::new()
            .with_profile(glutin::context::GlProfile::Core)
            .with_context_api(glutin::context::ContextApi::OpenGl(Some(glutin::context::Version {
                major: 3,
                minor: 3,
            })))
            .build(Some(window_handle.as_raw()));

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
