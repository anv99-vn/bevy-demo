use bevy::prelude::*;

use crate::camera::CameraSettings;

#[derive(Component)]
pub struct SensitivityText;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Text::new("Mouse sensitivity: 0.010"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        SensitivityText,
    ));
}

pub fn update(mut q: Query<&mut Text, With<SensitivityText>>, settings: Res<CameraSettings>) {
    for mut text in &mut q {
        text.0 = format!("Mouse sensitivity: {:.3}", settings.sensitivity);
    }
}
