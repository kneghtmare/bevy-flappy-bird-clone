use bevy::prelude::*;
use rand::Rng;
use crate::CFlappyMovement;

struct CPipe {
    is_upper: bool
}

struct CPipeSpawner {
    current_spawn_pos_x: f32,
    current_pipe_count: u32
}


impl CPipeSpawner {
    const PIPE_SIZE                 : f32 = 1.5;
    const SPAWN_OFFSET_X            : f32 = 200.0;
    const MAX_PIPE_COUNT            : u32 = 8;
    const SPAWN_POS_MIN_Y           : f32 = -400.0;
    const SPAWN_POS_MAX_Y           : f32 = -100.0;
    const UPPER_LOWER_PIPE_OFFSET_Y : f32 = 200.0;
    const PIPE_DESPAWN_OFFSET       : f32 = 25.0;


    fn can_spawn(&self) -> bool {
        !(self.current_pipe_count > CPipeSpawner::MAX_PIPE_COUNT)
    }

    fn spawn_pipe(commands: &mut Commands, pipe_sprites: &Res<RPipeSprites>, pipe_spawner: &mut CPipeSpawner) {
        // ------ spawn lower pipe here --------

        // y position of the spawned pipe
        let lower_pipe_pos_y: f32 = rand::thread_rng().gen_range(CPipeSpawner::SPAWN_POS_MIN_Y..=CPipeSpawner::SPAWN_POS_MAX_Y);
        
        // this will be the transform of the spawned lower pipe
        let mut lower_pipe_transform = Transform::from_xyz(pipe_spawner.current_spawn_pos_x, lower_pipe_pos_y, 0.0);
        lower_pipe_transform.scale = Vec3::splat(CPipeSpawner::PIPE_SIZE);

        commands.spawn_bundle(SpriteBundle {
            material : pipe_sprites.pipe_green_material.clone(),
            transform: lower_pipe_transform,
            ..Default::default()
        })
        .insert(CPipe{is_upper: false})
        ;

        // ------ spawn upper pipe here --------

         // this will be the transform of the spawned pipe 
        let mut upper_pipe_transform = Transform::from_xyz(pipe_spawner.current_spawn_pos_x, 
            // TODO: this 200.0 is supposed to be the height of the sprite, figure out a way to get the sprites size in pixels and add that
            // instead of hardcoding
            lower_pipe_pos_y + 400.0 + CPipeSpawner::UPPER_LOWER_PIPE_OFFSET_Y,
             0.0);

        // set scale
        upper_pipe_transform.scale = Vec3::splat(CPipeSpawner::PIPE_SIZE);
        
        commands.spawn_bundle(SpriteBundle {
            material : pipe_sprites.pipe_green_material.clone(),
            transform: upper_pipe_transform,
            ..Default::default()
        })
        .insert(CPipe{is_upper: true})
        ;

        // increase the x position for the next pipe spawned
        pipe_spawner.current_spawn_pos_x += CPipeSpawner::SPAWN_OFFSET_X;
        pipe_spawner.current_pipe_count += 1

    }


}

struct RPipeSprites {
    pipe_green_material: Handle<ColorMaterial>,
    pipe_red_material: Handle<ColorMaterial>,
}

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_startup_system(setup_pipe_resources.system())
        .add_startup_system(setup_pipe_spawner.system())

        .add_startup_stage("spawn_first_pipe", SystemStage::single(spawn_first_pipe.system()))

        .add_system(spawn_pipe_system.system())
        .add_system(flip_upper_pipes_system.system())
        .add_system(despawn_pipes_out_of_view.system())
        ;
    }
}

fn setup_pipe_resources(mut commands: Commands, asset_server: ResMut<AssetServer>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let pipe_green_texture_handle = asset_server.load("sprites/pipe-green.png");
    let pipe_red_texture_handle   = asset_server.load("sprites/pipe-red.png");

    commands.insert_resource(RPipeSprites {
        pipe_green_material : materials.add(pipe_green_texture_handle.into()),
        pipe_red_material   : materials.add(pipe_red_texture_handle.into()),
    })
}

fn setup_pipe_spawner(mut commands: Commands) {
    commands.spawn()
    .insert(CPipeSpawner {
        current_spawn_pos_x: 0.0,
        current_pipe_count: 0
    } )

    .insert(Timer::from_seconds(0.5, true))
    ;
}

fn spawn_pipe_system(mut commands: Commands, 
    pipe_sprites: Res<RPipeSprites>, 
    mut q: Query<(&mut CPipeSpawner, &mut Timer)>,
    time: Res<Time>) {

    // make sure there is only one pipe spawner
    let (mut pipe_spawner, mut timer) = q.single_mut().expect("There should only be one pipe spawner");
    timer.tick(time.delta());
    if timer.just_finished() && pipe_spawner.can_spawn() {
        CPipeSpawner::spawn_pipe(&mut commands, &pipe_sprites, &mut pipe_spawner);
    }
}

fn spawn_first_pipe(mut commands: Commands, pipe_sprites: Res<RPipeSprites>, mut q: Query<&mut CPipeSpawner>) {
    let mut pipe_spawner = q.single_mut().expect("There should only be one pipe spawner");
    CPipeSpawner::spawn_pipe(&mut commands, &pipe_sprites, &mut pipe_spawner);
}


fn despawn_pipes_out_of_view(mut commands: Commands,
    qset: 
    QuerySet<(
    Query<(Entity, &Transform), With<CPipe>>,
    Query<&Transform, With<CFlappyMovement>>,
    )>,

    windows: Res<Windows>,
    mut q2: Query<&mut CPipeSpawner> // <--- note: I can't have multiple queries but I can have a query set and another query 

    ) {
    let window = windows.get_primary().unwrap();
    let window_half_width: f32 = window.width() / 2.0;
    
    let player_transform = qset.q1().single().expect("There should only be one player");
    let player_pos_x = player_transform.translation.x;
    let mut pipe_spawner = q2.single_mut().expect("There should only be one pipe spawner");

    for (pipe_entity, pipe_transform) in qset.q0().iter() {
        //                                                           subtract a little bit of offset so that despawning wont be visible
        if pipe_transform.translation.x < (player_pos_x - window_half_width - CPipeSpawner::PIPE_DESPAWN_OFFSET) {
            commands.entity(pipe_entity).despawn();
            pipe_spawner.current_pipe_count -= 1;
        }
    }
}

fn flip_upper_pipes_system(mut q: Query<(&mut Sprite, &CPipe)>) {
    for (mut sprite, pipe) in q.iter_mut() {
        if pipe.is_upper {
            sprite.flip_y = true;
        }
    }
}