use crate::galaxy::CelestialBody;

/// 引力常量 (单位: m^3 kg^-1 s^-2)
const G: f64 = 6.67430e-11;

/// 计算两个天体之间的引力
pub fn calculate_gravitational_force(body1: &CelestialBody, body2: &CelestialBody) -> [f64; 3] {
    let dx = body2.position[0] - body1.position[0];
    let dy = body2.position[1] - body1.position[1];
    let dz = body2.position[2] - body1.position[2];

    let distance_squared = dx * dx + dy * dy + dz * dz;
    let distance = distance_squared.sqrt();

    if distance == 0.0 {
        return [0.0, 0.0, 0.0]; // 避免除以零
    }

    let force_magnitude = G * body1.mass * body2.mass / distance_squared;

    [
        force_magnitude * dx / distance,
        force_magnitude * dy / distance,
        force_magnitude * dz / distance,
    ]
}

/// 更新天体的位置和速度 (简单的欧拉积分)
pub fn update_positions_and_velocities(bodies: &mut [CelestialBody], dt: f64) {
    let mut forces = vec![[0.0; 3]; bodies.len()];

    // 计算所有天体之间的引力
    for i in 0..bodies.len() {
        for j in 0..bodies.len() {
            if i != j {
                let force = calculate_gravitational_force(&bodies[i], &bodies[j]);
                forces[i][0] += force[0];
                forces[i][1] += force[1];
                forces[i][2] += force[2];
            }
        }
    }

    // 更新速度和位置
    for (i, body) in bodies.iter_mut().enumerate() {
        body.velocity[0] += forces[i][0] / body.mass * dt;
        body.velocity[1] += forces[i][1] / body.mass * dt;
        body.velocity[2] += forces[i][2] / body.mass * dt;

        body.position[0] += body.velocity[0] * dt;
        body.position[1] += body.velocity[1] * dt;
        body.position[2] += body.velocity[2] * dt;
    }
}