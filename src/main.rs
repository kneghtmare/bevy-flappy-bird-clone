mod bird;
mod pipe;

use bevy::prelude::*;
use crate::bird::*;
use crate::pipe::*;

fn main() {
    App::build()
    .add_plugin(PipePlugin)
    .add_plugin(BirdPlugin)
    .add_plugins(DefaultPlugins)
    .run();
}


