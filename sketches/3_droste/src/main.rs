use bevy::{
    prelude::*,
};

pub mod sketch_plugin;
use crate::sketch_plugin::*;

fn main() {
    App::new()
        .add_plugins(SketchSetupPlugin)
        .add_systems(Update, update)
        .run();
}


fn update(
    time: Res<Time>,
) {
}
