pub mod velocity_component; // gravity component needs to modify velocity component

use bevy::prelude::*;

pub use velocity_component::*;

// gravity component
pub struct CGravity {
    pub value: f32
}

pub fn gravity_system(mut q: Query<(&CGravity, &mut CVelocity), With<Transform>>) {
    for (gravity, mut velocity) in q.iter_mut() {
        velocity.direction.y -= gravity.value;
    }   
}