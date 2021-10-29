mod bird;

use bevy::prelude::*;
use crate::bird::*;

fn main() {
    App::build()
    .add_plugin(BirdPlugin)
    .add_plugins(DefaultPlugins)
    .run();
}


