use glam::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub last_x: f32,
    pub last_y: f32,
}

impl Camera {
    pub fn new(position: Vec3, up: Vec3) -> Self {
        Camera {
            position,
            front: Vec3::new(0.0, 0.0, -1.0),
            up,
            yaw: -90.0,
            pitch: 0.0,
            last_x: 400.0,
            last_y: 300.0,
        }
    }
    
    pub fn get_view_matrix(&self) -> glam::Mat4 {
        glam::Mat4::look_at_rh(self.position, self.position + self.front, self.up)
    }
    
    pub fn process_mouse_movement(&mut self, xpos: f32, ypos: f32, sensitivity: f32) {
        let mut xoffset = xpos - self.last_x;
        let mut yoffset = self.last_y - ypos;
        xoffset *= sensitivity;
        yoffset *= sensitivity;
        
        self.yaw += xoffset;
        self.pitch += yoffset;
        
        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }
        
        let front = Vec3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        
        self.front = front.normalize();
        self.last_x = xpos;
        self.last_y = ypos;
    }
    
    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32, speed: f32) {
        let velocity = speed * delta_time;
        match direction {
            CameraMovement::Forward => self.position += self.front * velocity,
            CameraMovement::Backward => self.position -= self.front * velocity,
            CameraMovement::Left => self.position -= self.front.cross(self.up).normalize() * velocity,
            CameraMovement::Right => self.position += self.front.cross(self.up).normalize() * velocity,
            CameraMovement::Up => self.position += self.up * velocity,
            CameraMovement::Down => self.position -= self.up * velocity,
        }
    }
}

pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}