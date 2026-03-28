use glam::Vec3;
use crate::physics::object::Object;
use crate::constants::G;

pub struct GravityCalculator;

impl GravityCalculator {
    pub fn calculate_acceleration(obj1: &Object, obj2: &Object) -> Vec3 {
        let direction = obj2.position - obj1.position;
        let distance = direction.length();
        
        if distance > 0.0 {
            let direction_normalized = direction / distance;
            let distance_meters = distance * 1000.0;
            let force = (G as f32 * obj1.mass * obj2.mass) / (distance_meters * distance_meters);
            let acceleration = force / obj1.mass;
            direction_normalized * acceleration
        } else {
            Vec3::ZERO
        }
    }
}