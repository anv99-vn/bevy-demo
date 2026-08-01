use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

use crate::camera::CameraSettings;

const MIN_SENSITIVITY: f32 = 0.001;
const MAX_SENSITIVITY: f32 = 0.1;

#[derive(Component)]
pub struct SensitivityText;

#[derive(Component)]
pub struct SensitivitySlider;

#[derive(Component)]
pub struct SensitivityFill;

pub fn setup(mut commands: Commands, settings: Res<CameraSettings>) {
    let fill = ((settings.sensitivity - MIN_SENSITIVITY) / (MAX_SENSITIVITY - MIN_SENSITIVITY))
        .clamp(0.0, 1.0);

    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((Text::new("Mouse sensitivity:"), SensitivityText));
            parent
                .spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(14.0),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BorderColor(Color::WHITE),
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    Interaction::default(),
                    RelativeCursorPosition::default(),
                    SensitivitySlider,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            width: Val::Percent(fill * 100.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.25, 0.66, 0.94)),
                        SensitivityFill,
                    ));
                });
        });
}

pub fn update(
    mut settings: ResMut<CameraSettings>,
    mut text_q: Query<&mut Text, With<SensitivityText>>,
    slider_q: Query<(&Interaction, &RelativeCursorPosition), With<SensitivitySlider>>,
    mut fill_q: Query<&mut Node, (With<SensitivityFill>, Without<SensitivitySlider>)>,
) {
    for (interaction, relative) in &slider_q {
        if *interaction == Interaction::Pressed {
            if let Some(normalized) = relative.normalized {
                let frac = normalized.x.clamp(0.0, 1.0);
                settings.sensitivity = MIN_SENSITIVITY + (MAX_SENSITIVITY - MIN_SENSITIVITY) * frac;
            }
        }
    }

    let fill = ((settings.sensitivity - MIN_SENSITIVITY) / (MAX_SENSITIVITY - MIN_SENSITIVITY))
        .clamp(0.0, 1.0);
    for mut fill_node in &mut fill_q {
        fill_node.width = Val::Percent(fill * 100.0);
    }
    for mut text in &mut text_q {
        text.0 = format!("Mouse sensitivity: {:.3}", settings.sensitivity);
    }
}
