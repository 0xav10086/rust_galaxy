use glam::Vec3;
use crate::constants::PI;

pub struct GeometryUtils;

impl GeometryUtils {
    pub fn spherical_to_cartesian(r: f32, theta: f32, phi: f32) -> Vec3 {
        Vec3::new(
            r * theta.sin() * phi.cos(),
            r * theta.cos(),
            r * theta.sin() * phi.sin(),
        )
    }
}