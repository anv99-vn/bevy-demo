use bevy::prelude::*;

use crate::camera::BlocksCameraRotation;

const BUTTON_BG: Color = Color::srgb(0.2, 0.45, 0.85);
const BUTTON_HOVER: Color = Color::srgb(0.3, 0.55, 0.95);
const BUTTON_PRESSED: Color = Color::srgb(0.12, 0.32, 0.7);
const BUTTON_BORDER: Color = Color::srgb(0.7, 0.85, 1.0);

#[derive(Component)]
pub struct SettingsButton;

pub fn spawn(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Node {
                width: Val::Px(110.0),
                height: Val::Px(34.0),
                padding: UiRect::axes(Val::Px(16.0), Val::Px(6.0)),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(BUTTON_BORDER),
            BackgroundColor(BUTTON_BG),
            Interaction::default(),
            BlocksCameraRotation,
            SettingsButton,
        ))
        .with_children(|parent| {
            parent.spawn((Text::new("Settings"), TextColor(Color::WHITE)));
        });
}

pub fn style_button(
    mut q: Query<(&Interaction, &mut BackgroundColor, &mut BorderColor), With<SettingsButton>>,
) {
    for (interaction, mut bg, mut border) in &mut q {
        match *interaction {
            Interaction::Pressed => {
                *bg = BackgroundColor(BUTTON_PRESSED);
                *border = BorderColor(BUTTON_PRESSED);
            }
            Interaction::Hovered => {
                *bg = BackgroundColor(BUTTON_HOVER);
                *border = BorderColor(Color::WHITE);
            }
            Interaction::None => {
                *bg = BackgroundColor(BUTTON_BG);
                *border = BorderColor(BUTTON_BORDER);
            }
        }
    }
}
