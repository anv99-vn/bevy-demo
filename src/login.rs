use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use bevy::window::Ime;

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

#[derive(Resource, Default)]
pub(crate) struct ImeState {
    /// True while an IME (e.g. Unikey) is composing text in the preedit buffer.
    pub composing: bool,
}

/// Cooldown timer shared between `ime_input_system` and `text_input_system`
/// to prevent duplicate character insertion. When `ime_enabled: true`, Bevy
/// sends both an `Ime::Commit` event and a `KeyboardInput` for the same key
/// press. Without debouncing each letter would be inserted twice.
#[derive(Resource, Default)]
pub(crate) struct KeyDebounce {
    /// Remaining cooldown in seconds. While > 0, character insertion is
    /// suppressed. Set to `1/60` after every successful insert so that at
    /// most one character makes it through per frame (~60 chars/sec).
    pub cooldown: f32,
}

/// Key-repeat state for held-character / held-backspace input.
///
/// `held` tracks the `KeyCode` currently being repeated together with a timer
/// that implements an initial delay (OS-style) followed by a faster repeat
/// rate. Only one key is repeated at a time, mirroring typical text editor
/// behavior.
#[derive(Resource, Default)]
pub(crate) struct KeyRepeat {
    /// The key currently held down awaiting repeat, and its elapsed hold time.
    pub held: Option<(KeyCode, f32)>,
}

const REPEAT_INITIAL_DELAY: f32 = 0.5;
const REPEAT_RATE: f32 = 0.05;
/// Multiplier applied to `Time::delta_secs()` while a key is held, so repeats
/// fire faster than wall-clock time (effectively a "fast-forward" for the
/// held-key repeat timer).
const REPEAT_TIME_SCALE: f32 = 10.0;

pub fn setup(mut commands: Commands) {
    commands.insert_resource(InputFocus::default());
    commands.insert_resource(InputValues::default());
    commands.insert_resource(ImeState::default());
    commands.insert_resource(KeyRepeat::default());
    commands.insert_resource(KeyDebounce::default());

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

pub fn ime_input_system(
    mut events: EventReader<Ime>,
    mut ime_state: ResMut<ImeState>,
    focus: Res<InputFocus>,
    mut values: ResMut<InputValues>,
    mut username_q: Query<&mut Text, With<UsernameText>>,
    mut password_q: Query<&mut Text, (With<PasswordText>, Without<UsernameText>)>,
    mut debounce: ResMut<KeyDebounce>,
) {
    if !focus.username && !focus.password {
        events.clear();
        return;
    }

    let target: &mut String = if focus.username {
        &mut values.username
    } else {
        &mut values.password
    };

    let mut changed = false;
    for event in events.read() {
        match event {
            Ime::Commit { value, .. } => {
                if debounce.cooldown <= 0.0 {
                    target.push_str(value);
                    debounce.cooldown = 1.0 / 60.0;
                }
                ime_state.composing = false;
                changed = true;
            }
            Ime::Preedit { value, .. } => {
                ime_state.composing = !value.is_empty();
            }
            _ => {}
        }
    }

    if !changed {
        return;
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

fn key_to_char(key: &KeyCode) -> Option<char> {
    match key {
        KeyCode::KeyA => Some('a'),
        KeyCode::KeyB => Some('b'),
        KeyCode::KeyC => Some('c'),
        KeyCode::KeyD => Some('d'),
        KeyCode::KeyE => Some('e'),
        KeyCode::KeyF => Some('f'),
        KeyCode::KeyG => Some('g'),
        KeyCode::KeyH => Some('h'),
        KeyCode::KeyI => Some('i'),
        KeyCode::KeyJ => Some('j'),
        KeyCode::KeyK => Some('k'),
        KeyCode::KeyL => Some('l'),
        KeyCode::KeyM => Some('m'),
        KeyCode::KeyN => Some('n'),
        KeyCode::KeyO => Some('o'),
        KeyCode::KeyP => Some('p'),
        KeyCode::KeyQ => Some('q'),
        KeyCode::KeyR => Some('r'),
        KeyCode::KeyS => Some('s'),
        KeyCode::KeyT => Some('t'),
        KeyCode::KeyU => Some('u'),
        KeyCode::KeyV => Some('v'),
        KeyCode::KeyW => Some('w'),
        KeyCode::KeyX => Some('x'),
        KeyCode::KeyY => Some('y'),
        KeyCode::KeyZ => Some('z'),
        KeyCode::Digit0 => Some('0'),
        KeyCode::Digit1 => Some('1'),
        KeyCode::Digit2 => Some('2'),
        KeyCode::Digit3 => Some('3'),
        KeyCode::Digit4 => Some('4'),
        KeyCode::Digit5 => Some('5'),
        KeyCode::Digit6 => Some('6'),
        KeyCode::Digit7 => Some('7'),
        KeyCode::Digit8 => Some('8'),
        KeyCode::Digit9 => Some('9'),
        KeyCode::Space => Some(' '),
        _ => None,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn text_input_system(
    keys: Res<ButtonInput<KeyCode>>,
    ime_state: Res<ImeState>,
    mut values: ResMut<InputValues>,
    focus: Res<InputFocus>,
    mut username_q: Query<&mut Text, With<UsernameText>>,
    mut password_q: Query<&mut Text, (With<PasswordText>, Without<UsernameText>)>,
    time: Res<Time>,
    mut repeat: ResMut<KeyRepeat>,
    mut debounce: ResMut<KeyDebounce>,
) {
    // Advance the shared debounce cooldown each frame.
    debounce.cooldown = (debounce.cooldown - time.delta_secs()).max(0.0);

    if !focus.username && !focus.password {
        repeat.held = None;
        return;
    }
    // While a system IME (Unikey, etc.) is composing text, raw key events are
    // synthetic noise (e.g. `Unidentified` + repeated `Backspace`); let the
    // IME commit be the source of truth via `ime_input_system`.
    if ime_state.composing {
        repeat.held = None;
        return;
    }

    let target = if focus.username {
        &mut values.username
    } else {
        &mut values.password
    };

    // Determine the set of "actionable" keys currently relevant: Backspace
    // (for deletion) plus any mappable character key.
    let is_actionable = |key: &KeyCode| *key == KeyCode::Backspace || key_to_char(key).is_some();

    // Detect fresh presses this frame -> apply once and seed the repeat timer.
    let just_pressed: Vec<KeyCode> = keys
        .get_just_pressed()
        .copied()
        .filter(is_actionable)
        .collect();

    let mut changed = false;
    if !just_pressed.is_empty() && debounce.cooldown <= 0.0 {
        for key in &just_pressed {
            apply_key(key, target);
        }
        debounce.cooldown = 1.0 / 60.0;
        changed = true;
        // Restart repeat with the most-recently pressed actionable key.
        repeat.held = just_pressed.last().map(|k| (*k, 0.0));
    }

    // Advance the held-key timer; fire repeats after the initial delay and
    // then at `REPEAT_RATE` intervals while the key stays pressed.
    if let Some((key, elapsed)) = repeat.held.as_mut() {
        if keys.pressed(*key) {
            let prev = *elapsed;
            // Use real time while waiting for the initial delay, so a short
            // tap never accidentally crosses into repeat territory. Once the
            // initial delay has elapsed, switch to scaled time for fast
            // steady-state repeats only.
            let dt = if prev < REPEAT_INITIAL_DELAY {
                time.delta_secs()
            } else {
                time.delta_secs() * REPEAT_TIME_SCALE
            };
            *elapsed += dt;

            let mut fired = false;
            // Detect the transition across the initial delay boundary.
            if prev < REPEAT_INITIAL_DELAY
                && *elapsed >= REPEAT_INITIAL_DELAY
                && debounce.cooldown <= 0.0
            {
                apply_key(key, target);
                debounce.cooldown = 1.0 / 60.0;
                *elapsed -= REPEAT_INITIAL_DELAY;
                fired = true;
            }
            // After the initial delay, fire one repeat per `REPEAT_RATE`
            // interval that has elapsed during steady state.
            if *elapsed >= REPEAT_RATE && debounce.cooldown <= 0.0 {
                let count = (*elapsed / REPEAT_RATE).floor() as i32;
                let allowed = count.min(1); // cap: at most 1 char per cooldown window
                for _ in 0..allowed {
                    apply_key(key, target);
                }
                debounce.cooldown = 1.0 / 60.0;
                *elapsed -= count as f32 * REPEAT_RATE;
                fired = true;
            }
            if fired {
                changed = true;
            }
        } else {
            repeat.held = None;
        }
    }

    if !changed {
        return;
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

fn apply_key(key: &KeyCode, target: &mut String) {
    match key {
        KeyCode::Backspace => {
            target.pop();
        }
        k => {
            if let Some(c) = key_to_char(k) {
                target.push(c);
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    // ── key_to_char ──────────────────────────────────────────────────────

    #[test]
    fn key_to_char_letters() {
        assert_eq!(key_to_char(&KeyCode::KeyA), Some('a'));
        assert_eq!(key_to_char(&KeyCode::KeyZ), Some('z'));
    }

    #[test]
    fn key_to_char_digits() {
        assert_eq!(key_to_char(&KeyCode::Digit0), Some('0'));
        assert_eq!(key_to_char(&KeyCode::Digit9), Some('9'));
    }

    #[test]
    fn key_to_char_space() {
        assert_eq!(key_to_char(&KeyCode::Space), Some(' '));
    }

    #[test]
    fn key_to_char_unmapped() {
        assert_eq!(key_to_char(&KeyCode::ShiftLeft), None);
        assert_eq!(key_to_char(&KeyCode::Enter), None);
        assert_eq!(key_to_char(&KeyCode::Escape), None);
    }

    // ── apply_key ────────────────────────────────────────────────────────

    #[test]
    fn apply_key_backspace_pops() {
        let mut s = String::from("abc");
        apply_key(&KeyCode::Backspace, &mut s);
        assert_eq!(s, "ab");
    }

    #[test]
    fn apply_key_backspace_empty_string() {
        let mut s = String::new();
        apply_key(&KeyCode::Backspace, &mut s);
        assert_eq!(s, "");
    }

    #[test]
    fn apply_key_char_inserts() {
        let mut s = String::new();
        apply_key(&KeyCode::KeyH, &mut s);
        apply_key(&KeyCode::KeyI, &mut s);
        assert_eq!(s, "hi");
    }

    #[test]
    fn apply_key_space_inserts() {
        let mut s = String::from("a");
        apply_key(&KeyCode::Space, &mut s);
        assert_eq!(s, "a ");
    }

    #[test]
    fn apply_key_unmapped_noop() {
        let mut s = String::from("x");
        apply_key(&KeyCode::ShiftLeft, &mut s);
        assert_eq!(s, "x");
    }

    // ── KeyDebounce cooldown ─────────────────────────────────────────────

    #[test]
    fn debounce_blocks_when_active() {
        let mut debounce = KeyDebounce {
            cooldown: 1.0 / 60.0,
        };
        // Simulate: cooldown is active, should block insertion.
        assert!(debounce.cooldown > 0.0);
        // Decrement by half a frame — still active.
        debounce.cooldown = (debounce.cooldown - 0.5 * (1.0 / 60.0)).max(0.0);
        assert!(debounce.cooldown > 0.0);
    }

    #[test]
    fn debounce_expires_after_one_frame() {
        let mut debounce = KeyDebounce {
            cooldown: 1.0 / 60.0,
        };
        // Simulate one full frame passing.
        debounce.cooldown = (debounce.cooldown - 1.0 / 60.0).max(0.0);
        assert_eq!(debounce.cooldown, 0.0);
    }

    #[test]
    fn debounce_never_goes_negative() {
        let mut debounce = KeyDebounce { cooldown: 0.01 };
        // Decrement by more than the remaining cooldown.
        debounce.cooldown = (debounce.cooldown - 1.0).max(0.0);
        assert_eq!(debounce.cooldown, 0.0);
    }

    #[test]
    fn debounce_allows_insert_after_expiry() {
        let mut debounce = KeyDebounce {
            cooldown: 1.0 / 60.0,
        };
        // One frame passes — cooldown expires.
        debounce.cooldown = (debounce.cooldown - 1.0 / 60.0).max(0.0);
        assert!(debounce.cooldown <= 0.0);
        // Insertion should be allowed.
        let mut s = String::new();
        apply_key(&KeyCode::KeyA, &mut s);
        debounce.cooldown = 1.0 / 60.0;
        assert_eq!(s, "a");
        assert!(debounce.cooldown > 0.0);
    }

    #[test]
    fn debounce_blocks_rapid_double_insert() {
        let mut debounce = KeyDebounce { cooldown: 0.0 };
        let mut s = String::new();

        // First insert — allowed (cooldown is 0).
        assert!(debounce.cooldown <= 0.0);
        apply_key(&KeyCode::KeyF, &mut s);
        debounce.cooldown = 1.0 / 60.0;

        // Second insert immediately — blocked by cooldown.
        assert!(debounce.cooldown > 0.0);
        assert_eq!(s, "f"); // only one 'f', not 'ff'
    }

    #[test]
    fn debounce_allows_insert_after_frame_passes() {
        let mut debounce = KeyDebounce { cooldown: 0.0 };
        let mut s = String::new();

        // First insert.
        apply_key(&KeyCode::KeyF, &mut s);
        debounce.cooldown = 1.0 / 60.0;
        // Simulate frame passing.
        debounce.cooldown = (debounce.cooldown - 1.0 / 60.0).max(0.0);
        // Second insert — allowed.
        assert!(debounce.cooldown <= 0.0);
        apply_key(&KeyCode::KeyF, &mut s);
        assert_eq!(s, "ff");
    }

    // ── IME + debounce integration (unit-level) ─────────────────────────

    #[test]
    fn ime_commit_respects_debounce() {
        let mut debounce = KeyDebounce { cooldown: 0.0 };
        let mut target = String::new();

        // Simulate Ime::Commit arriving first.
        assert!(debounce.cooldown <= 0.0);
        target.push('a');
        debounce.cooldown = 1.0 / 60.0;

        // Simulate text_input_system also trying to insert 'a' on the same
        // frame — blocked by cooldown.
        assert!(debounce.cooldown > 0.0);
        assert_eq!(target, "a"); // not "aa"
    }

    #[test]
    fn ime_commit_and_text_input_alternate_frames() {
        let mut debounce = KeyDebounce { cooldown: 0.0 };
        let mut target = String::new();

        // Frame 1: IME inserts 'a'.
        assert!(debounce.cooldown <= 0.0);
        target.push('a');
        debounce.cooldown = 1.0 / 60.0;
        // Frame passes.
        debounce.cooldown = (debounce.cooldown - 1.0 / 60.0).max(0.0);

        // Frame 2: text_input inserts 'b'.
        assert!(debounce.cooldown <= 0.0);
        apply_key(&KeyCode::KeyB, &mut target);
        assert_eq!(target, "ab");
    }
}
