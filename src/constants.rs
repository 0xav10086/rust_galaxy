pub const G: f64 = 6.6743e-11;
pub const C: f32 = 299792458.0;
pub const PI: f32 = 3.14159265359;

// 用字面量避免 const fn 限制
pub const INIT_MASS: f32 = 5.0 * 1e20 / 5.0;

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;
pub const CAMERA_SPEED: f32 = 1000.0;
pub const MOUSE_SENSITIVITY: f32 = 0.1;

pub const TIME_STEP: f32 = 94.0;
pub const ACC_STEP: f32 = 96.0;

pub const GRID_SIZE: f32 = 100000.0;
pub const GRID_DIVISIONS: i32 = 50;