use crate::galaxy::CelestialBody;

/// 表示相机的结构体
pub struct Camera {
    pub position: [f64; 3], // 相机的位置
    pub focus: [f64; 3],    // 相机的焦点
    pub up: [f64; 3],       // 相机的上方向
}

impl Camera {
    /// 创建一个新的相机
    pub fn new(position: [f64; 3], focus: [f64; 3], up: [f64; 3]) -> Self {
        Self { position, focus, up }
    }

    /// 将相机的焦点设置为某个天体
    pub fn focus_on(&mut self, body: &CelestialBody) {
        self.focus = body.position;
    }

    /// 更新相机的位置，使其围绕焦点旋转
    pub fn orbit_around_focus(&mut self, radius: f64, angle: f64) {
        // 假设绕焦点的 Y 轴旋转
        self.position[0] = self.focus[0] + radius * angle.cos();
        self.position[2] = self.focus[2] + radius * angle.sin();
    }
}