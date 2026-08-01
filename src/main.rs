mod camera;
mod cube;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 3D Demo".into(),
                resolution: (800.0_f32, 600.0_f32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (cube::setup, camera::setup))
        .add_systems(Update, (cube::rotate, camera::update))
        .run();
}
