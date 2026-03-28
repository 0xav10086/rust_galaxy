use crate::physics::object::Object;

pub struct CollisionDetector;

impl CollisionDetector {
    pub fn check_collision(obj1: &Object, obj2: &Object) -> f32 {
        let distance = (obj2.position - obj1.position).length();
        
        if obj1.radius + obj2.radius > distance {
            -0.2  // 碰撞系数
        } else {
            1.0
        }
    }
}