use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

use crate::button::{self, SettingsButton};
use crate::camera::{BlocksCameraRotation, CameraSettings};

const MIN_SENSITIVITY: f32 = 0.001;
const MAX_SENSITIVITY: f32 = 0.1;

#[derive(Component)]
pub struct SensitivityText;

#[derive(Component)]
pub struct SensitivitySlider;

#[derive(Component)]
pub struct SensitivityFill;

#[derive(Component)]
pub struct SensitivityPanel;

#[derive(Resource, Default)]
pub struct SettingsVisible(bool);

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
            button::spawn(parent);
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexStart,
                        row_gap: Val::Px(4.0),
                        ..default()
                    },
                    Visibility::Hidden,
                    SensitivityPanel,
                ))
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
                            BlocksCameraRotation,
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
        });
}

#[allow(clippy::too_many_arguments)]
pub fn update(
    mut settings: ResMut<CameraSettings>,
    mut visible: ResMut<SettingsVisible>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut btn_q: Query<&Interaction, With<SettingsButton>>,
    mut panel_q: Query<&mut Visibility, With<SensitivityPanel>>,
    mut text_q: Query<&mut Text, With<SensitivityText>>,
    slider_q: Query<(&Interaction, &RelativeCursorPosition), With<SensitivitySlider>>,
    mut fill_q: Query<&mut Node, (With<SensitivityFill>, Without<SensitivitySlider>)>,
) {
    for interaction in &mut btn_q {
        if *interaction == Interaction::Pressed && mouse.just_pressed(MouseButton::Left) {
            visible.0 = !visible.0;
        }
    }
    for mut panel in &mut panel_q {
        *panel = if visible.0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

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
