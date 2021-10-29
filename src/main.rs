use bevy::prelude::*;

struct CFlappy {
    strength: f32
}

struct CGravity {
    value: f32
}

struct CAnimation;

struct RFlappyBirdSprites {
    yellow_bird_down_flap : Handle<ColorMaterial>,
    yellow_bird_mid_flap  : Handle<ColorMaterial>,
    yellow_bird_up_flap   : Handle<ColorMaterial>
}

struct SetupPlugin;
impl Plugin for SetupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_startup_system(setup_bird_resources.system())
        .add_startup_stage("setup_bird", SystemStage::single(setup_bird.system()));
    }
}

struct UpdatePlugin;
impl Plugin for UpdatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_system(gravity_system.system())
        .add_system(flappy_system.system());
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
    
    // load sprites
    let down_flap_handle = asset_server.load("sprites/yellowbird-downflap.png");
    let mid_flap_handle  = asset_server.load("sprites/yellowbird-midflap.png");
    let up_flap_handle   = asset_server.load("sprites/yellowbird-upflap.png");

    commands.insert_resource(RFlappyBirdSprites {
        yellow_bird_down_flap : materials.add(down_flap_handle.into()),
        yellow_bird_mid_flap  : materials.add(mid_flap_handle.into()), 
        yellow_bird_up_flap   : materials.add(up_flap_handle.into()), 
    });
}

fn setup_bird(mut commands: Commands, bird_materials: Res<RFlappyBirdSprites>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let _bird = commands.spawn_bundle(SpriteBundle {
        material: bird_materials.yellow_bird_mid_flap.clone(),
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..Default::default()
    })
    .insert(CFlappy  {strength: 12.0})
    .insert(CGravity {value: 5.0})
    .insert(CAnimation)
    .id();
}

fn gravity_system(mut q: Query<(&CGravity, &mut Transform)>) {
    for (gravity, mut transform) in q.iter_mut() {
        transform.translation.y -= gravity.value;
    }   
}

fn flappy_system(keyboard_input: Res<Input<KeyCode>>, mut q: Query<(&CFlappy, &mut Transform)>) {
    if keyboard_input.pressed(KeyCode::Space) {
        for (flappy, mut transform) in q.iter_mut() {
            transform.translation.y += flappy.strength;
        }
    }
}