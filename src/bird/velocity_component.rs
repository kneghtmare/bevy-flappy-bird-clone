use bevy::prelude::*;

pub struct CVelocity {
    pub value: Vec2,
    pub direction: Vec2,
    pub speed: f32
}

impl Default for CVelocity {
    fn default() -> Self {
        CVelocity {
            value: Vec2::ZERO,
            direction: Vec2::ZERO,
            speed: 3.0
        }
    }
}

pub fn velocity_system(mut q: Query<(&mut CVelocity, &mut Transform)>) {
    for (mut velocity, mut transform) in q.iter_mut() {
        velocity.value = velocity.direction * velocity.speed;
        transform.translation += Vec3::new(velocity.value.x, velocity.value.y, 0.0);
    }
}
