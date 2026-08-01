mod button;
mod camera;
mod cube;
mod hud;
mod login;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Login,
    Playing,
}

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
        .add_plugins(EguiPlugin)
        .init_state::<GameState>()
        // Login scene
        .add_systems(OnEnter(GameState::Login), login::setup)
        .add_systems(
            Update,
            login::login_ui_system.run_if(in_state(GameState::Login)),
        )
        // Game scene
        .add_systems(
            OnEnter(GameState::Playing),
            (cube::setup, camera::setup, hud::setup),
        )
        .add_systems(
            Update,
            (
                cube::rotate,
                camera::update,
                hud::update,
                button::style_button,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .init_resource::<camera::CameraSettings>()
        .init_resource::<hud::SettingsVisible>()
        .run();
}
