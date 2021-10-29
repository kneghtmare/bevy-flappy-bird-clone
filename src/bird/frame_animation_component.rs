use bevy::prelude::*;

// animation component
pub struct CFrameAnimation {
    pub anim_vec: Vec<Handle<ColorMaterial>>,
    pub current_frame: usize,
}

impl CFrameAnimation {
    // takes a texture handle and sets it the current frame, we have to run go_next_frame() to go to the next frame
    pub fn set_texture_handle_to_current_frame(&self, texture_handle: &mut Handle<ColorMaterial>) {
        *texture_handle = self.anim_vec[self.current_frame].clone();
    }
    
    // self explanatory
    pub fn go_next_frame(&mut self) {
        if !(self.current_frame + 1 >= self.anim_vec.len()) {
            self.current_frame += 1;
        } else {
            self.current_frame = 0;
        }
    }
}

impl Default for CFrameAnimation {
    fn default() -> Self {
        CFrameAnimation {
            anim_vec: vec![],
            current_frame: 0
        }
    }
}


pub fn animation_system(time: Res<Time>, mut q: Query<(&mut Timer, &mut CFrameAnimation, &mut Handle<ColorMaterial>)>) {
    for (mut timer, mut canim, mut texture_handle) in q.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {  
            canim.go_next_frame();
            canim.set_texture_handle_to_current_frame(&mut texture_handle);
        }
    }
}