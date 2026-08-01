use bevy::input::keyboard::KeyCode;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::window::Ime;

use crate::login::{InputFocus, PasswordText, UsernameText};

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
///
/// The window is wider than one frame so the duplicate event is still
/// suppressed when it arrives in a later frame (the two events frequently
/// straddle a frame boundary). A one-frame window decayed to exactly zero at
/// the start of the next frame, letting the second event insert again.
const KEY_INSERT_COOLDOWN: f32 = 1.0 / 30.0;

#[derive(Resource, Default)]
pub(crate) struct KeyDebounce {
    /// Remaining cooldown in seconds. While > 0, character insertion is
    /// suppressed. Set to `KEY_INSERT_COOLDOWN` after every successful insert
    /// so each key press produces at most one character (~30 chars/sec).
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

pub fn log_key_events(mut events: EventReader<KeyboardInput>) {
    for event in events.read() {
        if event.state == ButtonState::Pressed {
            info!(
                "key pressed: {:?} ({:?})",
                event.key_code, event.logical_key
            );
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
                    debounce.cooldown = KEY_INSERT_COOLDOWN;
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
        debounce.cooldown = KEY_INSERT_COOLDOWN;
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
                debounce.cooldown = KEY_INSERT_COOLDOWN;
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
                debounce.cooldown = KEY_INSERT_COOLDOWN;
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
            cooldown: KEY_INSERT_COOLDOWN,
        };
        // Simulate: cooldown is active, should block insertion.
        assert!(debounce.cooldown > 0.0);
        // Decrement by half a frame — still active.
        debounce.cooldown = (debounce.cooldown - 0.5 * (1.0 / 60.0)).max(0.0);
        assert!(debounce.cooldown > 0.0);
    }

    #[test]
    fn debounce_survives_next_frame() {
        let mut debounce = KeyDebounce {
            cooldown: KEY_INSERT_COOLDOWN,
        };
        // One full frame passes at 60fps — still active.
        debounce.cooldown = (debounce.cooldown - 1.0 / 60.0).max(0.0);
        assert!(debounce.cooldown > 0.0);
        // A second frame elapses the full window.
        debounce.cooldown = (debounce.cooldown - 1.0 / 60.0).max(0.0);
        assert!(debounce.cooldown <= 0.0);
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
            cooldown: KEY_INSERT_COOLDOWN,
        };
        // Two frames pass — cooldown expires.
        debounce.cooldown = (debounce.cooldown - 1.0 / 60.0).max(0.0);
        debounce.cooldown = (debounce.cooldown - 1.0 / 60.0).max(0.0);
        assert!(debounce.cooldown <= 0.0);
        // Insertion should be allowed.
        let mut s = String::new();
        apply_key(&KeyCode::KeyA, &mut s);
        debounce.cooldown = KEY_INSERT_COOLDOWN;
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
        debounce.cooldown = KEY_INSERT_COOLDOWN;

        // Second insert immediately — blocked by cooldown.
        assert!(debounce.cooldown > 0.0);
        assert_eq!(s, "f"); // only one 'f', not 'ff'
    }

    #[test]
    fn debounce_blocks_insert_one_frame_later() {
        let mut debounce = KeyDebounce { cooldown: 0.0 };
        let mut s = String::new();

        // First insert.
        apply_key(&KeyCode::KeyF, &mut s);
        debounce.cooldown = KEY_INSERT_COOLDOWN;
        // One frame passes — cooldown is still active, so the duplicate
        // (IME/keyboard) event landing a frame later is suppressed.
        debounce.cooldown = (debounce.cooldown - 1.0 / 60.0).max(0.0);
        assert!(debounce.cooldown > 0.0);
        apply_key(&KeyCode::KeyF, &mut s);
        assert_eq!(s, "f"); // still only one 'f', not 'ff'
    }

    // ── repeat-run accounting ────────────────────────────────────────────

    #[test]
    fn repeat_runs_count_as_single_press() {
        // Holding a key produces repeated letters, but each hold is one key
        // press. "aaaaabbbaccc" = hold 'a', hold 'b', tap 'a', hold 'c'.
        let typed = "aaaaabbbaccc";
        let mut presses = 0;
        let mut prev: Option<char> = None;
        for c in typed.chars() {
            if Some(c) != prev {
                presses += 1;
                prev = Some(c);
            }
        }
        assert_eq!(presses, 4);
        assert_eq!(typed.chars().last(), Some('c'));
    }

    // ── IME + debounce integration (unit-level) ─────────────────────────

    #[test]
    fn ime_commit_respects_debounce() {
        let mut debounce = KeyDebounce { cooldown: 0.0 };
        let mut target = String::new();

        // Simulate Ime::Commit arriving first.
        assert!(debounce.cooldown <= 0.0);
        target.push('a');
        debounce.cooldown = KEY_INSERT_COOLDOWN;

        // Simulate text_input_system also trying to insert 'a' on the same
        // frame — blocked by cooldown.
        assert!(debounce.cooldown > 0.0);
        assert_eq!(target, "a"); // not "aa"
    }

    #[test]
    fn ime_commit_duplicate_next_frame_blocked() {
        let mut debounce = KeyDebounce { cooldown: 0.0 };
        let mut target = String::new();

        // Frame 1: IME inserts 'a'.
        target.push('a');
        debounce.cooldown = KEY_INSERT_COOLDOWN;
        // One frame passes — cooldown is still active.
        debounce.cooldown = (debounce.cooldown - 1.0 / 60.0).max(0.0);

        // Frame 2: keyboard path tries to insert 'a' again — blocked.
        assert!(debounce.cooldown > 0.0);
        apply_key(&KeyCode::KeyA, &mut target);
        assert_eq!(target, "a"); // not "aa"
    }
}
