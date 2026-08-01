use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::GameState;

#[derive(Resource, Default)]
pub(crate) struct LoginData {
    pub username: String,
    pub password: String,
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(LoginData::default());
}

pub fn login_ui_system(
    mut contexts: EguiContexts,
    mut login_data: ResMut<LoginData>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(80.0);

            ui.heading("Login");
            ui.add_space(20.0);

            ui.label("Username");
            ui.text_edit_singleline(&mut login_data.username);
            ui.add_space(8.0);

            ui.label("Password");
            ui.add(egui::TextEdit::singleline(&mut login_data.password).password(true));
            ui.add_space(20.0);

            if ui.button("Login").clicked() {
                info!("Login: username={}, password={}", login_data.username, login_data.password);
                next_state.set(GameState::Playing);
            }
        });
    });
}
