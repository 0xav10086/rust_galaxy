pub mod context;
pub mod graphics;
pub mod app;

use crate::galaxy::CelestialBody;
use crate::camera::Camera;
use winit::event_loop::EventLoop;

pub fn render_solar_system(bodies: &[CelestialBody], camera: &mut Camera) {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let app = app::App::new(&event_loop);
    
    // Clone bodies and camera to pass ownership to the event loop
    let bodies = bodies.to_vec();
    let camera = Camera {
        position: camera.position,
        focus: camera.focus,
        up: camera.up,
    };

    app.run(event_loop, bodies, camera);
}
