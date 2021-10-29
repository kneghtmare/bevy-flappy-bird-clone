use bevy::prelude::*;

// flappy behaviour component
struct CFlappy {
    strength: f32,
    rotation_strength: f32
}

// gravity component
struct CGravity {
    value: f32
}


struct CVelocity {
    value: Vec2,
    direction: Vec2,
    speed: f32
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

// animation component
struct CAnimation {
    anim_vec: Vec<Handle<ColorMaterial>>,
    current_frame: usize,
}

impl CAnimation {
    // takes a texture handle and sets it the current frame, we have to run go_next_frame() to go to the next frame
    fn set_texture_handle_to_current_frame(&self, texture_handle: &mut Handle<ColorMaterial>) {
        *texture_handle = self.anim_vec[self.current_frame].clone();
    }
    
    // self explanatory
    fn go_next_frame(&mut self) {
        if !(self.current_frame + 1 >= self.anim_vec.len()) {
            self.current_frame += 1;
        } else {
            self.current_frame = 0;
        }
    }
}

impl Default for CAnimation {
    fn default() -> Self {
        CAnimation {
            anim_vec: vec![],
            current_frame: 0
        }
    }
}

// resource for holding the sprites in one place
struct RFlappyBirdSprites {
    yellow_bird_down_flap : Handle<ColorMaterial>,
    yellow_bird_mid_flap  : Handle<ColorMaterial>,
    yellow_bird_up_flap   : Handle<ColorMaterial>
}

// plugin that sets up all the games resources and objects
struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_startup_system(setup_bird_resources.system())
        // we have to add this to a different stage because this depends on the setup_bird_resources system() and bevy may try to parrarelize
        .add_startup_stage("setup_bird", SystemStage::single(setup_bird.system()));
    }
}

// plugin for stuff that is supposed to happen every frame
struct UpdatePlugin;
impl Plugin for UpdatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_system(gravity_system.system())
        .add_system(flappy_system.system())
        .add_system(animation_system.system())
        .add_system(velocity_system.system())
        ;

    }
}

fn main() {
    App::build()
    .add_plugin(SetupPlugin)
    .add_plugin(UpdatePlugin)
    .add_plugins(DefaultPlugins)
    .run();
}


fn setup_bird_resources(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>) {
    
    // load all sprites here
    let down_flap_handle = asset_server.load("sprites/yellowbird-downflap.png");
    let mid_flap_handle  = asset_server.load("sprites/yellowbird-midflap.png");
    let up_flap_handle   = asset_server.load("sprites/yellowbird-upflap.png");
    
    // we now have access to the bird sprites resource
    commands.insert_resource(RFlappyBirdSprites {
        yellow_bird_down_flap : materials.add(down_flap_handle.into()),
        yellow_bird_mid_flap  : materials.add(mid_flap_handle.into()), 
        yellow_bird_up_flap   : materials.add(up_flap_handle.into()), 
    });
}

fn setup_bird(mut commands: Commands, bird_materials: Res<RFlappyBirdSprites>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d()); // TODO: move this to some other system

    // bird setup, we add the sprite bundle, the flappy component, the animation component, and the timer component
    let _bird = commands.spawn_bundle(SpriteBundle {
        material: bird_materials.yellow_bird_mid_flap.clone(),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..Default::default()
    })
    .insert(CFlappy  {strength: 4.0, rotation_strength: 100.0})
    .insert(CGravity {value: 0.2})

    .insert(CAnimation {
        anim_vec: vec![
            bird_materials.yellow_bird_up_flap.clone_weak(),
            bird_materials.yellow_bird_mid_flap.clone_weak(), 
            bird_materials.yellow_bird_down_flap.clone_weak()
        ],
        ..Default::default()
    })

    .insert(CVelocity {
        speed: 3.0, 
        ..Default::default()
    })

    .insert(Timer::from_seconds(1.0/4.0, true))

    .id()
    ;

}

// -----systems----

fn gravity_system(mut q: Query<(&CGravity, &mut CVelocity), With<Transform>>) {
    for (gravity, mut velocity) in q.iter_mut() {
        velocity.direction.y -= gravity.value;
    }   
}

fn flappy_system(keyboard_input: Res<Input<KeyCode>>, mut q: Query<(&CFlappy, &mut CVelocity), With<Transform>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (flappy, mut velocity) in q.iter_mut() {
            velocity.direction.y = flappy.strength;
            // transform.rotation *= Quat::from_rotation_z(-flappy.rotation_strength);
        }
    }
}

fn velocity_system(mut q: Query<(&mut CVelocity, &mut Transform), With<CFlappy>>) {
    for (mut velocity, mut transform) in q.iter_mut() {
        velocity.value = velocity.direction * velocity.speed;
        transform.translation += Vec3::new(velocity.value.x, velocity.value.y, 0.0);
    }
}

fn animation_system(time: Res<Time>, mut q: Query<(&mut Timer, &mut CAnimation, &mut Handle<ColorMaterial>)>) {
    for (mut timer, mut canim, mut texture_handle) in q.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {  
            canim.go_next_frame();
            canim.set_texture_handle_to_current_frame(&mut texture_handle);
        }
    }
}