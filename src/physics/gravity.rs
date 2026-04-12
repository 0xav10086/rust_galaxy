use glam::Vec3;
use crate::physics::object::Object;
use crate::constants::G;

pub struct GravityCalculator;

const FORCE_MULTIPLIER: f32 = 500.0; // Adjusted to match C++ example speed
const SOFTENING: f32 = 1.0;

impl GravityCalculator {
    pub fn calculate_acceleration(obj1: &Object, obj2: &Object) -> Vec3 {
        let direction = obj2.position - obj1.position;
        let distance = direction.length();

        if distance > 1.0 { // Minimum distance to prevent extreme forces
            let direction_normalized = direction / distance;
            // Scale: 1 unit = 100,000 meters (100km)
            let distance_meters = distance * 100_000.0;
            let distance_sq = (distance_meters * distance_meters) + SOFTENING;
            let force = (G as f32 * obj1.mass * obj2.mass) / distance_sq;

            // Convert force/mass (acceleration in m/s^2) back to units/s^2
            let acceleration_m_s2 = force / obj1.mass;
            let acceleration_units_s2 = (acceleration_m_s2 / 100_000.0) * FORCE_MULTIPLIER;

            direction_normalized * acceleration_units_s2
        } else {
            Vec3::ZERO
        }
    }
}