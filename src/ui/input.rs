use winit::event::{ElementState, WindowEvent, MouseButton, MouseScrollDelta};
use winit::keyboard::{KeyCode, PhysicalKey};
use crate::graphics::camera::{Camera, CameraMovement};

pub struct InputHandler {
    pub pause: bool,
    pub running: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            pause: false,
            running: true,
        }
    }
    
    pub fn process_keyboard(
        &mut self,
        key: KeyCode,
        state: ElementState,
        camera: &mut Camera,
        delta_time: f32,
        camera_speed: f32,
    ) {
        let pressed = state == ElementState::Pressed;
        
        match key {
            KeyCode::KeyW if pressed => camera.process_keyboard(CameraMovement::Forward, delta_time, camera_speed),
            KeyCode::KeyS if pressed => camera.process_keyboard(CameraMovement::Backward, delta_time, camera_speed),
            KeyCode::KeyA if pressed => camera.process_keyboard(CameraMovement::Left, delta_time, camera_speed),
            KeyCode::KeyD if pressed => camera.process_keyboard(CameraMovement::Right, delta_time, camera_speed),
            KeyCode::Space if pressed => camera.process_keyboard(CameraMovement::Up, delta_time, camera_speed),
            KeyCode::ShiftLeft if pressed => camera.process_keyboard(CameraMovement::Down, delta_time, camera_speed),
            KeyCode::KeyK if pressed => self.pause = true,
            KeyCode::KeyK if state == ElementState::Released => self.pause = false,
            KeyCode::KeyQ if pressed => self.running = false,
            _ => {}
        }
    }
    
    pub fn process_object_movement(
        &self,
        key: KeyCode,
        state: ElementState,
        object: &mut crate::physics::object::Object,
        _shift_pressed: bool,
    ) {
        if !object.initializing {
            return;
        }
        
        let pressed = state == ElementState::Pressed;
        let movement_speed = 0.5;
        
        match key {
            KeyCode::ArrowUp if pressed => object.position.y += movement_speed,
            KeyCode::ArrowDown if pressed => object.position.y -= movement_speed,
            KeyCode::ArrowRight if pressed => object.position.x += movement_speed,
            KeyCode::ArrowLeft if pressed => object.position.x -= movement_speed,
            _ => {}
        }
    }
}