mod camera;
mod cube;
mod hud;

use bevy::prelude::*;

// the `bevy_main` proc_macro generates the required boilerplate for Android
#[bevy_main]
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 3D Demo".into(),
                resolution: (800.0_f32, 600.0_f32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (cube::setup, camera::setup, hud::setup))
        .add_systems(Update, (cube::rotate, camera::update, hud::update))
        .init_resource::<camera::CameraSettings>()
        .run();
}
