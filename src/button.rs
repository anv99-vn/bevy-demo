use bevy::prelude::*;

use crate::camera::BlocksCameraRotation;

const BUTTON_BG: Color = Color::srgb(0.2, 0.2, 0.2);
const BUTTON_HOVER: Color = Color::srgb(0.31, 0.31, 0.31);
const BUTTON_PRESSED: Color = Color::srgb(0.25, 0.66, 0.94);
const BUTTON_BORDER: Color = Color::srgb(0.5, 0.5, 0.5);

#[derive(Component)]
pub struct SettingsButton;

pub fn spawn(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Node {
                width: Val::Px(90.0),
                height: Val::Px(28.0),
                padding: UiRect::axes(Val::Px(12.0), Val::Px(4.0)),
                border: UiRect::all(Val::Px(1.0)),
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
            parent.spawn(Text::new("Settings"));
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
