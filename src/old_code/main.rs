// main.rs
mod galaxy;
#[path = "render/mod.rs"]
mod render;
mod camera;
mod physics;

use galaxy::create_solar_system;
use render::render_solar_system;
use camera::Camera;

fn main() {
    let solar_system = create_solar_system();

    // 将所有天体放入一个数组中
    let mut bodies = vec![solar_system.star];
    bodies.extend(solar_system.planets);
    bodies.extend(solar_system.satellites);

    // 创建相机并聚焦在水星上
    let mercury = bodies.iter().find(|b| b.name == "Mercury").unwrap();
    let mut camera = Camera::new(
        [mercury.position[0] + 1e10, mercury.position[1], mercury.position[2]],
        mercury.position,
        [0.0, 1.0, 0.0],
    );

    // 进入渲染循环
    render_solar_system(&bodies, &mut camera);
}