use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;

use crate::camera::BlocksCameraRotation;
use crate::GameState;

const BUTTON_BG: Color = Color::srgb(0.2, 0.45, 0.85);
const BUTTON_HOVER: Color = Color::srgb(0.3, 0.55, 0.95);
const BUTTON_PRESSED: Color = Color::srgb(0.12, 0.32, 0.7);
const BUTTON_BORDER: Color = Color::srgb(0.7, 0.85, 1.0);

const INPUT_BG: Color = Color::srgb(0.15, 0.15, 0.2);
const INPUT_BORDER: Color = Color::srgb(0.4, 0.4, 0.5);
const INPUT_FOCUSED_BORDER: Color = Color::srgb(0.25, 0.66, 0.94);

#[derive(Component)]
pub(crate) struct LoginButton;

#[derive(Component)]
pub(crate) struct UsernameInput;

#[derive(Component)]
pub(crate) struct PasswordInput;

#[derive(Component)]
pub(crate) struct LoginScreen;

#[derive(Component)]
pub(crate) struct LoginCamera;

#[derive(Component)]
pub(crate) struct UsernameText;

#[derive(Component)]
pub(crate) struct PasswordText;

#[derive(Resource, Default)]
pub(crate) struct InputFocus {
    pub username: bool,
    pub password: bool,
}

#[derive(Resource, Default)]
pub(crate) struct InputValues {
    pub username: String,
    pub password: String,
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(InputFocus::default());
    commands.insert_resource(InputValues::default());

    commands.spawn((Camera2d, LoginCamera));

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
            LoginScreen,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Login"),
                TextColor(Color::WHITE),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
            ));

            // Username field
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    row_gap: Val::Px(4.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((Text::new("Username"), TextColor(Color::srgb(0.7, 0.7, 0.8))));

                    parent
                        .spawn((
                            Node {
                                width: Val::Px(300.0),
                                height: Val::Px(40.0),
                                padding: UiRect::all(Val::Px(8.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BorderColor(INPUT_BORDER),
                            BackgroundColor(INPUT_BG),
                            Interaction::default(),
                            BlocksCameraRotation,
                            UsernameInput,
                        ))
                        .with_children(|parent| {
                            parent.spawn((Text::new(""), TextColor(Color::WHITE), UsernameText));
                        });
                });

            // Password field
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::FlexStart,
                    row_gap: Val::Px(4.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((Text::new("Password"), TextColor(Color::srgb(0.7, 0.7, 0.8))));

                    parent
                        .spawn((
                            Node {
                                width: Val::Px(300.0),
                                height: Val::Px(40.0),
                                padding: UiRect::all(Val::Px(8.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BorderColor(INPUT_BORDER),
                            BackgroundColor(INPUT_BG),
                            Interaction::default(),
                            BlocksCameraRotation,
                            PasswordInput,
                        ))
                        .with_children(|parent| {
                            parent.spawn((Text::new(""), TextColor(Color::WHITE), PasswordText));
                        });
                });

            // Login button
            parent
                .spawn((
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(45.0),
                        padding: UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    BorderColor(BUTTON_BORDER),
                    BackgroundColor(BUTTON_BG),
                    Interaction::default(),
                    BlocksCameraRotation,
                    LoginButton,
                ))
                .with_children(|parent| {
                    parent.spawn((Text::new("Login"), TextColor(Color::WHITE)));
                });
        });
}

pub fn focus_input_system(
    mouse: Res<ButtonInput<MouseButton>>,
    username_q: Query<&Interaction, (With<UsernameInput>, Changed<Interaction>)>,
    password_q: Query<&Interaction, (With<PasswordInput>, Changed<Interaction>)>,
    mut focus: ResMut<InputFocus>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        for interaction in &username_q {
            if *interaction == Interaction::Pressed {
                focus.username = true;
                focus.password = false;
                return;
            }
        }
        for interaction in &password_q {
            if *interaction == Interaction::Pressed {
                focus.username = false;
                focus.password = true;
                return;
            }
        }
    }
}

pub fn text_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    mut values: ResMut<InputValues>,
    focus: Res<InputFocus>,
    mut username_q: Query<&mut Text, With<UsernameText>>,
    mut password_q: Query<&mut Text, (With<PasswordText>, Without<UsernameText>)>,
) {
    if !focus.username && !focus.password {
        return;
    }

    let target = if focus.username {
        &mut values.username
    } else {
        &mut values.password
    };

    for key in keys.get_pressed() {
        match key {
            KeyCode::Backspace => {
                target.pop();
            }
            KeyCode::KeyA => {
                target.push('a');
            }
            KeyCode::KeyB => {
                target.push('b');
            }
            KeyCode::KeyC => {
                target.push('c');
            }
            KeyCode::KeyD => {
                target.push('d');
            }
            KeyCode::KeyE => {
                target.push('e');
            }
            KeyCode::KeyF => {
                target.push('f');
            }
            KeyCode::KeyG => {
                target.push('g');
            }
            KeyCode::KeyH => {
                target.push('h');
            }
            KeyCode::KeyI => {
                target.push('i');
            }
            KeyCode::KeyJ => {
                target.push('j');
            }
            KeyCode::KeyK => {
                target.push('k');
            }
            KeyCode::KeyL => {
                target.push('l');
            }
            KeyCode::KeyM => {
                target.push('m');
            }
            KeyCode::KeyN => {
                target.push('n');
            }
            KeyCode::KeyO => {
                target.push('o');
            }
            KeyCode::KeyP => {
                target.push('p');
            }
            KeyCode::KeyQ => {
                target.push('q');
            }
            KeyCode::KeyR => {
                target.push('r');
            }
            KeyCode::KeyS => {
                target.push('s');
            }
            KeyCode::KeyT => {
                target.push('t');
            }
            KeyCode::KeyU => {
                target.push('u');
            }
            KeyCode::KeyV => {
                target.push('v');
            }
            KeyCode::KeyW => {
                target.push('w');
            }
            KeyCode::KeyX => {
                target.push('x');
            }
            KeyCode::KeyY => {
                target.push('y');
            }
            KeyCode::KeyZ => {
                target.push('z');
            }
            KeyCode::Digit0 => {
                target.push('0');
            }
            KeyCode::Digit1 => {
                target.push('1');
            }
            KeyCode::Digit2 => {
                target.push('2');
            }
            KeyCode::Digit3 => {
                target.push('3');
            }
            KeyCode::Digit4 => {
                target.push('4');
            }
            KeyCode::Digit5 => {
                target.push('5');
            }
            KeyCode::Digit6 => {
                target.push('6');
            }
            KeyCode::Digit7 => {
                target.push('7');
            }
            KeyCode::Digit8 => {
                target.push('8');
            }
            KeyCode::Digit9 => {
                target.push('9');
            }
            KeyCode::Space => {
                target.push(' ');
            }
            _ => {}
        }
    }

    if focus.username {
        for mut text in &mut username_q {
            text.0 = values.username.clone();
        }
    } else {
        for mut text in &mut password_q {
            text.0 = values.password.clone();
        }
    }
}

pub fn style_input_fields(
    focus: Res<InputFocus>,
    mut username_q: Query<&mut BorderColor, With<UsernameInput>>,
    mut password_q: Query<&mut BorderColor, (With<PasswordInput>, Without<UsernameInput>)>,
) {
    for mut border in &mut username_q {
        *border = if focus.username {
            BorderColor(INPUT_FOCUSED_BORDER)
        } else {
            BorderColor(INPUT_BORDER)
        };
    }
    for mut border in &mut password_q {
        *border = if focus.password {
            BorderColor(INPUT_FOCUSED_BORDER)
        } else {
            BorderColor(INPUT_BORDER)
        };
    }
}

pub fn login_button_system(
    interaction_query: Query<&Interaction, (With<LoginButton>, Changed<Interaction>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::Playing);
        }
    }
}

pub fn despawn_login_screen(
    mut commands: Commands,
    query: Query<Entity, With<LoginScreen>>,
    camera_query: Query<Entity, With<LoginCamera>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &camera_query {
        commands.entity(entity).despawn();
    }
}

pub fn style_login_button(
    mut q: Query<(&Interaction, &mut BackgroundColor, &mut BorderColor), With<LoginButton>>,
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
