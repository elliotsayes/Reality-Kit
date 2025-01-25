// A bevy system that converts a keycode to a game action based on `KeyboardConfig`

use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use crate::model::keyboard::{Key, KeyboardConfig};
use crate::model::game_action::{ActionType, GameActionEvent};
use crate::GameActions;

pub fn bevy_keycode_to_action<GA>(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut game_action_events: EventWriter<GameActionEvent<GA>>,
    config: Res<KeyboardConfig<GA>>,
) where GA: GameActions {
    for ev in evr_kbd.read() {
        let Some(key) = bevy_keycode_to_keyboard_config_key(ev.key_code) else {
            continue;
        };

        if let Some(game_actions) = config.clone().get_actions(&key) {
            for game_action in game_actions {
                let action_type = match ev.state {
                    ButtonState::Pressed => ActionType::Begin,
                    ButtonState::Released => ActionType::End,
                };

                debug!(
                    "keycode: {:?}, state: {:?}, game action: {:?}, action type: {:?}",
                    ev.key_code,
                    ev.state,
                    game_action,
                    action_type
                );
                game_action_events.send(GameActionEvent::new(game_action.clone(), action_type));
            }
        }
    }
}

fn bevy_keycode_to_keyboard_config_key(key: KeyCode) -> Option<Key> {
    let x = match key {
        KeyCode::Backquote => Key::Backquote,
        KeyCode::Backslash => Key::Backslash,
        KeyCode::BracketLeft => Key::BracketLeft,
        KeyCode::BracketRight => Key::BracketRight,
        KeyCode::Comma => Key::Comma,
        KeyCode::Digit0 => Key::Digit0,
        KeyCode::Digit1 => Key::Digit1,
        KeyCode::Digit2 => Key::Digit2,
        KeyCode::Digit3 => Key::Digit3,
        KeyCode::Digit4 => Key::Digit4,
        KeyCode::Digit5 => Key::Digit5,
        KeyCode::Digit6 => Key::Digit6,
        KeyCode::Digit7 => Key::Digit7,
        KeyCode::Digit8 => Key::Digit8,
        KeyCode::Digit9 => Key::Digit9,
        KeyCode::Equal => Key::Equal,
        KeyCode::IntlBackslash => Key::IntlBackslash,
        KeyCode::IntlRo => Key::IntlRo,
        KeyCode::IntlYen => Key::IntlYen,
        KeyCode::KeyA => Key::KeyA,
        KeyCode::KeyB => Key::KeyB,
        KeyCode::KeyC => Key::KeyC,
        KeyCode::KeyD => Key::KeyD,
        KeyCode::KeyE => Key::KeyE,
        KeyCode::KeyF => Key::KeyF,
        KeyCode::KeyG => Key::KeyG,
        KeyCode::KeyH => Key::KeyH,
        KeyCode::KeyI => Key::KeyI,
        KeyCode::KeyJ => Key::KeyJ,
        KeyCode::KeyK => Key::KeyK,
        KeyCode::KeyL => Key::KeyL,
        KeyCode::KeyM => Key::KeyM,
        KeyCode::KeyN => Key::KeyN,
        KeyCode::KeyO => Key::KeyO,
        KeyCode::KeyP => Key::KeyP,
        KeyCode::KeyQ => Key::KeyQ,
        KeyCode::KeyR => Key::KeyR,
        KeyCode::KeyS => Key::KeyS,
        KeyCode::KeyT => Key::KeyT,
        KeyCode::KeyU => Key::KeyU,
        KeyCode::KeyV => Key::KeyV,
        KeyCode::KeyW => Key::KeyW,
        KeyCode::KeyX => Key::KeyX,
        KeyCode::KeyY => Key::KeyY,
        KeyCode::KeyZ => Key::KeyZ,
        KeyCode::Minus => Key::Minus,
        KeyCode::Period => Key::Period,
        KeyCode::Quote => Key::Quote,
        KeyCode::Semicolon => Key::Semicolon,
        KeyCode::Slash => Key::Slash,
        KeyCode::AltLeft => Key::AltLeft,
        KeyCode::AltRight => Key::AltRight,
        KeyCode::Backspace => Key::Backspace,
        KeyCode::CapsLock => Key::CapsLock,
        KeyCode::ContextMenu => Key::ContextMenu,
        KeyCode::ControlLeft => Key::ControlLeft,
        KeyCode::ControlRight => Key::ControlRight,
        KeyCode::Enter => Key::Enter,
        KeyCode::SuperLeft => Key::MetaLeft,  // Super/Win key maps to Meta
        KeyCode::SuperRight => Key::MetaRight,
        KeyCode::ShiftLeft => Key::ShiftLeft,
        KeyCode::ShiftRight => Key::ShiftRight,
        KeyCode::Space => Key::Space,
        KeyCode::Tab => Key::Tab,
        KeyCode::Convert => Key::Convert,
        KeyCode::KanaMode => Key::KanaMode,
        KeyCode::Lang1 => Key::Lang1,
        KeyCode::Lang2 => Key::Lang2,
        KeyCode::Lang3 => Key::Lang3,
        KeyCode::Lang4 => Key::Lang4,
        KeyCode::Lang5 => Key::Lang5,
        KeyCode::NonConvert => Key::NonConvert,
        KeyCode::Delete => Key::Delete,
        KeyCode::End => Key::End,
        KeyCode::Help => Key::Help,
        KeyCode::Home => Key::Home,
        KeyCode::Insert => Key::Insert,
        KeyCode::PageDown => Key::PageDown,
        KeyCode::PageUp => Key::PageUp,
        KeyCode::ArrowDown => Key::ArrowDown,
        KeyCode::ArrowLeft => Key::ArrowLeft,
        KeyCode::ArrowRight => Key::ArrowRight,
        KeyCode::ArrowUp => Key::ArrowUp,
        KeyCode::NumLock => Key::NumLock,
        KeyCode::Numpad0 => Key::Numpad0,
        KeyCode::Numpad1 => Key::Numpad1,
        KeyCode::Numpad2 => Key::Numpad2,
        KeyCode::Numpad3 => Key::Numpad3,
        KeyCode::Numpad4 => Key::Numpad4,
        KeyCode::Numpad5 => Key::Numpad5,
        KeyCode::Numpad6 => Key::Numpad6,
        KeyCode::Numpad7 => Key::Numpad7,
        KeyCode::Numpad8 => Key::Numpad8,
        KeyCode::Numpad9 => Key::Numpad9,
        _ => return None,
    };

    Some(x)
}
